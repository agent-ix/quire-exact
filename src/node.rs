// SPDX-License-Identifier: AGPL-3.0-or-later
//! Checked semantic node identity.
//!
//! A `NodeKey` carries the structural identity of a node in an admitted package.
//! Its owner, path, and ordinal compare componentwise as QSpec FR-322 specifies.
//! The caller checks that the identity joins its admitted package; this leaf
//! checks only the closed shape and never computes a digest.

use alloc::{string::String, vec::Vec};
use core::{
    alloc::Layout,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    sync::atomic::AtomicUsize,
};

use allocator_api2::boxed::Box;
use spin::Mutex;
use triomphe::Arc;

use crate::identity_copy::{array_bytes, copy_text, retype, IdentityCopyError};

#[cfg(test)]
mod construction_probe {
    use std::cell::Cell;

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub(super) struct Counts {
        pub direct_visits: usize,
        pub direct_moves: usize,
        pub descendant_visits: usize,
        pub descendant_copies: usize,
        pub shallow_clones: usize,
    }

    std::thread_local! {
        static COUNTS: Cell<Counts> = Cell::new(Counts::default());
    }

    fn update(change: impl FnOnce(&mut Counts)) {
        COUNTS.with(|cell| {
            let mut counts = cell.get();
            change(&mut counts);
            cell.set(counts);
        });
    }

    pub(super) fn reset() {
        COUNTS.with(|cell| cell.set(Counts::default()));
    }

    pub(super) fn snapshot() -> Counts {
        COUNTS.with(Cell::get)
    }

    pub(super) fn direct_visit() {
        update(|counts| counts.direct_visits += 1);
    }
    pub(super) fn direct_move() {
        update(|counts| counts.direct_moves += 1);
    }
    pub(super) fn descendant_visit() {
        update(|counts| counts.descendant_visits += 1);
    }
    pub(super) fn descendant_copy() {
        update(|counts| counts.descendant_copies += 1);
    }
    pub(super) fn shallow_clone() {
        update(|counts| counts.shallow_clones += 1);
    }
}

/// The greatest node ordinal admitted by the checked-package V2 schema.
pub const MAX_NODE_ORDINAL: u64 = 9_007_199_254_740_991;

/// Whether `text` is `^[A-Za-z_][A-Za-z0-9_]*$`: the one definition of this
/// character class, which every identifier-shaped field and [`Identifier`]
/// validate against.
pub fn is_identifier(text: &str) -> bool {
    let mut bytes = text.bytes();
    bytes
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// An identifier-shaped name segment (`^[A-Za-z_][A-Za-z0-9_]*$`). An
/// invalid identifier is refused at construction rather than reaching a
/// schema-invalid wire shape.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Identifier(String);

/// A string that is not `^[A-Za-z_][A-Za-z0-9_]*$`.
#[derive(Clone, Debug, Eq, Hash, PartialEq, thiserror::Error)]
#[error("an identifier is `^[A-Za-z_][A-Za-z0-9_]*$`")]
pub struct InvalidIdentifier;

impl Identifier {
    /// `value` as an identifier, or [`InvalidIdentifier`] if it is not
    /// `^[A-Za-z_][A-Za-z0-9_]*$`.
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidIdentifier> {
        let value = value.into();
        if is_identifier(&value) {
            Ok(Self(value))
        } else {
            Err(InvalidIdentifier)
        }
    }

    /// The identifier's own text.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One segment of a structural node path. Text sorts before a nested node id.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PathSegment {
    /// An exact UTF-8 segment, including the empty string where admitted.
    Text(String),
    /// An operand node identity nested in a derived node's path.
    Node(NodeKey),
}

impl PathSegment {
    /// Copy this segment, refusing if any storage reservation fails.
    pub fn try_clone(&self) -> Result<Self, InvalidNodeKey> {
        self.try_clone_legacy_with(&mut |_| Ok(()))
    }

    fn try_clone_legacy_with(
        &self,
        before_reserve: &mut impl FnMut(usize) -> Result<(), InvalidNodeKey>,
    ) -> Result<Self, InvalidNodeKey> {
        match self {
            Self::Text(text) => Ok(Self::Text(copy_text_with(text, before_reserve)?)),
            Self::Node(node) => Ok(Self::Node(node.try_clone_legacy_with(before_reserve)?)),
        }
    }
}

fn copy_text_with(
    text: &str,
    before_reserve: &mut impl FnMut(usize) -> Result<(), InvalidNodeKey>,
) -> Result<String, InvalidNodeKey> {
    let mut copy = String::new();
    if !text.is_empty() {
        let requested = text.len();
        before_reserve(requested)?;
        copy.try_reserve_exact(text.len())
            .map_err(|_| InvalidNodeKey::Allocation { requested })?;
        copy.push_str(text);
    }
    Ok(copy)
}

/// A malformed structural node identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum InvalidNodeKey {
    /// The owning package has no name.
    #[error("a node identity requires a nonempty package")]
    EmptyPackage,
    /// The structural path has no segment.
    #[error("a node identity requires a nonempty path")]
    EmptyPath,
    /// The ordinal exceeds the checked-package V2 schema maximum.
    #[error("a node ordinal exceeds the checked-package V2 maximum")]
    OrdinalOutOfRange,
    /// Storage for a structural identity could not be reserved; the payload is
    /// the minimum additional byte count passed to that reservation.
    #[error("a structural node identity allocation failed")]
    Allocation {
        /// The minimum additional bytes requested, or `usize::MAX` if unrepresentable.
        requested: usize,
    },
}

fn flat_reservation_bytes(count: usize) -> Result<usize, InvalidNodeKey> {
    count
        .checked_mul(core::mem::size_of::<FlatSegment>())
        .ok_or(InvalidNodeKey::Allocation {
            requested: usize::MAX,
        })
}

fn nested_token_count(child_count: usize) -> Result<usize, InvalidNodeKey> {
    child_count
        .checked_add(1)
        .ok_or(InvalidNodeKey::Allocation {
            requested: usize::MAX,
        })
}

fn node_arc_bytes() -> Result<usize, ()> {
    Layout::new::<AtomicUsize>()
        .extend(Layout::new::<NodeData>())
        .map(|(layout, _)| layout.pad_to_align().size())
        .map_err(|_| ())
}

/// A structural node identity from an admitted checked package.
///
/// quire:canonical
pub struct NodeKey {
    inner: Arc<NodeData>,
}

impl Clone for NodeKey {
    fn clone(&self) -> Self {
        #[cfg(test)]
        construction_probe::shallow_clone();
        Self {
            inner: self.inner.clone(),
        }
    }
}

struct NodeData {
    package: String,
    path: Vec<FlatSegment>,
    ordinal: u64,
    structural_hash: u64,
    token_count: usize,
    drop_frame: Option<Box<DropFrame>>,
}

struct CopyFrame<'a> {
    source: &'a NodeKey,
    package: String,
    path: Vec<FlatSegment>,
    next: usize,
}

#[derive(Debug, Eq, Hash, PartialEq)]
enum FlatSegment {
    Text(String),
    Node(NodeKey),
}

impl Clone for FlatSegment {
    fn clone(&self) -> Self {
        #[cfg(test)]
        construction_probe::descendant_copy();
        match self {
            Self::Text(text) => Self::Text(text.clone()),
            Self::Node(node) => Self::Node(node.clone()),
        }
    }
}

struct DropFrame {
    pending: Vec<FlatSegment>,
    next: Option<Box<DropFrame>>,
}

struct DropQueue {
    head: Option<Box<DropFrame>>,
    draining: bool,
}

static DROP_QUEUE: Mutex<DropQueue> = Mutex::new(DropQueue {
    head: None,
    draining: false,
});

impl Drop for NodeData {
    fn drop(&mut self) {
        let Some(mut frame) = self.drop_frame.take() else {
            return;
        };
        frame.pending = core::mem::take(&mut self.path);
        let mut queue = DROP_QUEUE.lock();
        frame.next = queue.head.take();
        queue.head = Some(frame);
        if queue.draining {
            return;
        }
        queue.draining = true;
        drop(queue);

        loop {
            let next = {
                let mut queue = DROP_QUEUE.lock();
                match queue.head.take() {
                    Some(mut frame) => {
                        queue.head = frame.next.take();
                        Some(frame)
                    }
                    None => {
                        queue.draining = false;
                        None
                    }
                }
            };
            let Some(mut frame) = next else {
                break;
            };
            while let Some(segment) = frame.pending.pop() {
                drop(segment);
            }
        }
    }
}

/// A borrowed structural node identity. Creating or traversing a view does not allocate.
#[derive(Clone, Copy)]
pub struct NodeKeyView<'a> {
    data: &'a NodeData,
}

impl<'a> NodeKeyView<'a> {
    /// The exact owning package name.
    pub fn package(self) -> &'a str {
        &self.data.package
    }

    /// The ordinal within the owning package and path.
    pub fn ordinal(self) -> u64 {
        self.data.ordinal
    }

    /// Iterate over direct path segments without copying nested identities.
    pub fn path(self) -> NodePathIter<'a> {
        #[cfg(test)]
        construction_probe::descendant_visit();
        NodePathIter {
            path: self.data.path.iter(),
        }
    }
}

/// One borrowed direct path segment.
#[derive(Clone, Copy)]
pub enum PathSegmentView<'a> {
    /// Exact borrowed UTF-8 text, including an empty string.
    Text(&'a str),
    /// A borrowed nested node identity.
    Node(NodeKeyView<'a>),
}

/// Allocation-free iterator over a node identity's direct path.
pub struct NodePathIter<'a> {
    path: core::slice::Iter<'a, FlatSegment>,
}

impl<'a> Iterator for NodePathIter<'a> {
    type Item = PathSegmentView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.path.next()? {
            FlatSegment::Text(text) => {
                #[cfg(test)]
                construction_probe::descendant_visit();
                Some(PathSegmentView::Text(text))
            }
            FlatSegment::Node(node) => {
                #[cfg(test)]
                construction_probe::descendant_visit();
                Some(PathSegmentView::Node(node.view()))
            }
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
enum ComparePart<'a> {
    Package(&'a str),
    PathEnd,
    Text(&'a str),
    NodeStart,
    Ordinal(u64),
    Done,
}

// The inline frames make the common mixed path a single walk without changing
// the zero-allocation comparison contract. Deeper branching uses the existing
// index-based traversal, which has no depth limit.
const COMPARE_INLINE_DEPTH: usize = 1_024;

#[derive(Clone, Copy)]
struct CompareFrame<'a> {
    node: &'a NodeKey,
    index: usize,
    phase: u8,
}

struct CompareCursor<'a> {
    frames: [CompareFrame<'a>; COMPARE_INLINE_DEPTH],
    depth: usize,
    pending_child: Option<&'a NodeKey>,
}

impl<'a> CompareCursor<'a> {
    fn new(node: &'a NodeKey) -> Self {
        Self {
            frames: [CompareFrame {
                node,
                index: 0,
                phase: 0,
            }; COMPARE_INLINE_DEPTH],
            depth: 1,
            pending_child: None,
        }
    }

    fn skip_shared_child(&mut self) {
        self.pending_child = None;
    }

    // None means the inline frames filled; the caller restarts the comparison
    // with the index-based traversal.
    fn next_part(&mut self) -> Option<(ComparePart<'a>, Option<&'a NodeKey>)> {
        if let Some(child) = self.pending_child.take() {
            if self.depth == COMPARE_INLINE_DEPTH {
                return None;
            }
            self.frames[self.depth] = CompareFrame {
                node: child,
                index: 0,
                phase: 0,
            };
            self.depth += 1;
        }

        loop {
            if self.depth == 0 {
                return Some((ComparePart::Done, None));
            }
            let frame = &mut self.frames[self.depth - 1];
            match frame.phase {
                0 => {
                    frame.phase = 1;
                    return Some((ComparePart::Package(&frame.node.inner.package), None));
                }
                1 => match frame.node.inner.path.get(frame.index) {
                    Some(FlatSegment::Text(text)) => {
                        frame.index += 1;
                        return Some((ComparePart::Text(text), None));
                    }
                    Some(FlatSegment::Node(child)) => {
                        frame.index += 1;
                        self.pending_child = Some(child);
                        return Some((ComparePart::NodeStart, Some(child)));
                    }
                    None => frame.phase = 2,
                },
                2 => {
                    frame.phase = 3;
                    return Some((ComparePart::PathEnd, None));
                }
                3 => {
                    frame.phase = 4;
                    return Some((ComparePart::Ordinal(frame.node.inner.ordinal), None));
                }
                _ => self.depth -= 1,
            }
        }
    }
}

fn compare_streaming(left: &NodeKey, right: &NodeKey) -> Option<Ordering> {
    let mut a = CompareCursor::new(left);
    let mut b = CompareCursor::new(right);
    loop {
        let (left_part, left_child) = a.next_part()?;
        let (right_part, right_child) = b.next_part()?;
        let order = left_part.cmp(&right_part);
        if !order.is_eq() || left_part == ComparePart::Done {
            return Some(order);
        }
        if let (Some(left_child), Some(right_child)) = (left_child, right_child) {
            if Arc::ptr_eq(&left_child.inner, &right_child.inner) {
                a.skip_shared_child();
                b.skip_shared_child();
            }
        }
    }
}

fn compare_part_at(mut node: &NodeKey, mut index: usize) -> ComparePart<'_> {
    loop {
        if index == 0 {
            return ComparePart::Package(&node.inner.package);
        }
        index -= 1;
        let mut descended = false;
        for segment in &node.inner.path {
            match segment {
                FlatSegment::Text(text) => {
                    if index == 0 {
                        return ComparePart::Text(text);
                    }
                    index -= 1;
                }
                FlatSegment::Node(child) => {
                    if index == 0 {
                        return ComparePart::NodeStart;
                    }
                    index -= 1;
                    if index < child.inner.token_count {
                        node = child;
                        descended = true;
                        break;
                    }
                    index -= child.inner.token_count;
                }
            }
        }
        if descended {
            continue;
        }
        return match index {
            0 => ComparePart::PathEnd,
            1 => ComparePart::Ordinal(node.inner.ordinal),
            _ => ComparePart::Done,
        };
    }
}

fn first_nested_child(node: &NodeKey) -> Option<(usize, &NodeKey)> {
    for (index, segment) in node.inner.path.iter().enumerate() {
        if let FlatSegment::Node(child) = segment {
            return Some((index, child));
        }
    }
    None
}

fn text_segments_order(left: &[FlatSegment], right: &[FlatSegment]) -> Ordering {
    for (left_segment, right_segment) in left.iter().zip(right) {
        let (FlatSegment::Text(left_text), FlatSegment::Text(right_text)) =
            (left_segment, right_segment)
        else {
            unreachable!("segments before the first nested child contain only text");
        };
        let order = left_text.cmp(right_text);
        if !order.is_eq() {
            return order;
        }
    }
    left.len().cmp(&right.len())
}

fn compare_indexed(left: &NodeKey, right: &NodeKey) -> Ordering {
    if Arc::ptr_eq(&left.inner, &right.inner) {
        return Ordering::Equal;
    }
    let mut index = 0;
    loop {
        let a = compare_part_at(left, index);
        let b = compare_part_at(right, index);
        let order = a.cmp(&b);
        if !order.is_eq() || a == ComparePart::Done {
            return order;
        }
        index += 1;
    }
}

fn path_suffix_order(left: &[FlatSegment], right: &[FlatSegment]) -> Ordering {
    for (left_segment, right_segment) in left.iter().zip(right) {
        let order = match (left_segment, right_segment) {
            (FlatSegment::Text(a), FlatSegment::Text(b)) => a.cmp(b),
            (FlatSegment::Text(_), FlatSegment::Node(_)) => Ordering::Less,
            (FlatSegment::Node(_), FlatSegment::Text(_)) => Ordering::Greater,
            (FlatSegment::Node(a), FlatSegment::Node(b)) => compare_indexed(a, b),
        };
        if !order.is_eq() {
            return order;
        }
    }
    left.len().cmp(&right.len())
}

impl Ord for NodeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        if Arc::ptr_eq(&self.inner, &other.inner) {
            return Ordering::Equal;
        }
        // A derived chain commonly wraps one operand at every level. Text
        // before its first child sorts first; later segments and the ordinal
        // sort after it. Only the deepest differing local order is kept.
        let mut left = self;
        let mut right = other;
        let mut deferred_order = Ordering::Equal;
        while let (Some((left_index, left_child)), Some((right_index, right_child))) =
            (first_nested_child(left), first_nested_child(right))
        {
            let package_order = left.inner.package.cmp(&right.inner.package);
            if !package_order.is_eq() {
                return package_order;
            }
            if left_index != right_index {
                break;
            }
            let prefix_order = text_segments_order(
                &left.inner.path[..left_index],
                &right.inner.path[..right_index],
            );
            if !prefix_order.is_eq() {
                return prefix_order;
            }
            let suffix_order = path_suffix_order(
                &left.inner.path[left_index + 1..],
                &right.inner.path[right_index + 1..],
            );
            let local_order = if suffix_order.is_eq() {
                left.inner.ordinal.cmp(&right.inner.ordinal)
            } else {
                suffix_order
            };
            if !local_order.is_eq() {
                deferred_order = local_order;
            }
            left = left_child;
            right = right_child;
            if Arc::ptr_eq(&left.inner, &right.inner) {
                return deferred_order;
            }
        }
        if let Some(order) = compare_streaming(left, right) {
            return if order.is_eq() { deferred_order } else { order };
        }
        let order = compare_indexed(left, right);
        if order.is_eq() {
            deferred_order
        } else {
            order
        }
    }
}

impl PartialEq for NodeKey {
    fn eq(&self, other: &Self) -> bool {
        self.inner.structural_hash == other.inner.structural_hash && self.cmp(other).is_eq()
    }
}

impl Eq for NodeKey {}

impl Hash for NodeKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner.structural_hash.hash(state);
    }
}

struct StructuralHasher(u64);

impl Hasher for StructuralHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= u64::from(*byte);
            self.0 = self.0.wrapping_mul(0x100_0000_01b3);
        }
    }
}

fn structural_hash(package: &str, path: &[FlatSegment], ordinal: u64) -> u64 {
    let mut hasher = StructuralHasher(0xcbf2_9ce4_8422_2325);
    0_u8.hash(&mut hasher);
    package.hash(&mut hasher);
    for segment in path {
        match segment {
            FlatSegment::Text(text) => {
                2_u8.hash(&mut hasher);
                text.hash(&mut hasher);
            }
            FlatSegment::Node(node) => {
                3_u8.hash(&mut hasher);
                node.inner.structural_hash.hash(&mut hasher);
            }
        }
    }
    1_u8.hash(&mut hasher);
    4_u8.hash(&mut hasher);
    ordinal.hash(&mut hasher);
    hasher.finish()
}

impl PartialOrd for NodeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl NodeKey {
    /// Carry a node identity already admitted by the caller. This checks its
    /// closed shape, but does not resolve the node against a package.
    pub fn new(
        package: impl Into<String>,
        path: Vec<PathSegment>,
        ordinal: u64,
    ) -> Result<Self, InvalidNodeKey> {
        Self::try_new(package.into(), path, ordinal)
    }

    /// Carry an already owned structural node identity, refusing failed storage
    /// reservations separately from invalid shape. The caller owns any fallible
    /// copy needed to obtain the package string and path before this call.
    pub fn try_new(
        package: String,
        path: Vec<PathSegment>,
        ordinal: u64,
    ) -> Result<Self, InvalidNodeKey> {
        Self::try_new_with(package, path, ordinal, &mut |_| Ok(()))
    }

    fn try_new_with(
        package: String,
        path: Vec<PathSegment>,
        ordinal: u64,
        before_reserve: &mut impl FnMut(usize) -> Result<(), InvalidNodeKey>,
    ) -> Result<Self, InvalidNodeKey> {
        if package.is_empty() {
            return Err(InvalidNodeKey::EmptyPackage);
        }
        if path.is_empty() {
            return Err(InvalidNodeKey::EmptyPath);
        }
        if ordinal > MAX_NODE_ORDINAL {
            return Err(InvalidNodeKey::OrdinalOutOfRange);
        }
        let mut token_count = 3_usize;
        for segment in &path {
            #[cfg(test)]
            construction_probe::direct_visit();
            let additional = match segment {
                PathSegment::Text(_) => 1,
                PathSegment::Node(node) => nested_token_count(node.inner.token_count)?,
            };
            token_count =
                token_count
                    .checked_add(additional)
                    .ok_or(InvalidNodeKey::Allocation {
                        requested: usize::MAX,
                    })?;
        }
        let requested = flat_reservation_bytes(path.len())?;
        before_reserve(requested)?;
        let mut direct = Vec::new();
        direct
            .try_reserve_exact(path.len())
            .map_err(|_| InvalidNodeKey::Allocation { requested })?;
        for segment in path {
            #[cfg(test)]
            construction_probe::direct_move();
            direct.push(match segment {
                PathSegment::Text(text) => FlatSegment::Text(text),
                PathSegment::Node(node) => FlatSegment::Node(node),
            });
        }
        let frame_request = core::mem::size_of::<DropFrame>();
        before_reserve(frame_request)?;
        let frame = Box::try_new(DropFrame {
            pending: Vec::new(),
            next: None,
        })
        .map_err(|_| InvalidNodeKey::Allocation {
            requested: frame_request,
        })?;
        let arc_request = node_arc_bytes().map_err(|_| InvalidNodeKey::Allocation {
            requested: usize::MAX,
        })?;
        before_reserve(arc_request)?;
        let structural_hash = structural_hash(&package, &direct, ordinal);
        let inner = Arc::try_new(NodeData {
            package,
            path: direct,
            ordinal,
            structural_hash,
            token_count,
            drop_frame: Some(frame),
        })
        .map_err(|_| InvalidNodeKey::Allocation {
            requested: arc_request,
        })?;
        Ok(Self { inner })
    }

    /// The exact owning package name.
    pub fn package(&self) -> &str {
        &self.inner.package
    }

    /// The path segments in component order. Nested identities share storage.
    pub fn path(&self) -> Vec<PathSegment> {
        #[cfg(test)]
        construction_probe::descendant_visit();
        self.inner
            .path
            .iter()
            .map(|segment| match segment {
                FlatSegment::Text(text) => PathSegment::Text(text.clone()),
                FlatSegment::Node(node) => PathSegment::Node(node.clone()),
            })
            .collect()
    }

    /// The ordinal within the owning package and path.
    pub fn ordinal(&self) -> u64 {
        self.inner.ordinal
    }

    /// Borrow the package, ordinal and direct path without constructing an owned path.
    pub fn view(&self) -> NodeKeyView<'_> {
        NodeKeyView { data: &self.inner }
    }

    /// Copy this identity with a fallible reservation for every owned string
    /// and flat path token; nested identity depth does not grow the call stack.
    pub fn try_clone(&self) -> Result<Self, InvalidNodeKey> {
        self.try_clone_legacy_with(&mut |_| Ok(()))
    }

    /// Copy this structural key, presenting each actual additional payload-byte
    /// request to `before_reserve` before attempting its native reservation.
    /// The caller's refusal is returned unchanged inside [`IdentityCopyError::Policy`].
    /// Count and layout incapacity remain distinct from allocator denial.
    pub fn try_clone_with<E>(
        &self,
        before_reserve: &mut impl FnMut(usize) -> Result<(), E>,
    ) -> Result<Self, IdentityCopyError<E>> {
        self.try_clone_with_native_hook(before_reserve, &mut |_| true)
    }

    fn try_clone_with_native_hook<E>(
        &self,
        before_reserve: &mut impl FnMut(usize) -> Result<(), E>,
        native_reserve: &mut impl FnMut(usize) -> bool,
    ) -> Result<Self, IdentityCopyError<E>> {
        fn make_frame<'a, E>(
            source: &'a NodeKey,
            before_reserve: &mut impl FnMut(usize) -> Result<(), E>,
            native_reserve: &mut impl FnMut(usize) -> bool,
        ) -> Result<CopyFrame<'a>, IdentityCopyError<E>> {
            #[cfg(test)]
            construction_probe::descendant_copy();
            let package = copy_text(&source.inner.package, before_reserve, native_reserve)?;
            let requested = array_bytes::<FlatSegment>(source.inner.path.len()).map_err(retype)?;
            before_reserve(requested).map_err(IdentityCopyError::Policy)?;
            if !native_reserve(requested) {
                return Err(IdentityCopyError::Allocation { requested });
            }
            let mut path = Vec::new();
            path.try_reserve_exact(source.inner.path.len())
                .map_err(|_| IdentityCopyError::Allocation { requested })?;
            Ok(CopyFrame {
                source,
                package,
                path,
                next: 0,
            })
        }

        let mut stack = Vec::new();
        let first = make_frame(self, before_reserve, native_reserve)?;
        let mut pending = Some(first);
        loop {
            if let Some(frame) = pending.take() {
                if stack.len() == stack.capacity() {
                    let requested = array_bytes::<CopyFrame<'_>>(1).map_err(retype)?;
                    before_reserve(requested).map_err(IdentityCopyError::Policy)?;
                    if !native_reserve(requested) {
                        return Err(IdentityCopyError::Allocation { requested });
                    }
                    stack
                        .try_reserve_exact(1)
                        .map_err(|_| IdentityCopyError::Allocation { requested })?;
                }
                stack.push(frame);
            }
            let Some(frame) = stack.last_mut() else {
                unreachable!("copy begins with one frame")
            };
            if let Some(segment) = frame.source.inner.path.get(frame.next) {
                frame.next += 1;
                match segment {
                    FlatSegment::Text(text) => {
                        frame.path.push(FlatSegment::Text(copy_text(
                            text,
                            before_reserve,
                            native_reserve,
                        )?));
                    }
                    FlatSegment::Node(child) => {
                        pending = Some(make_frame(child, before_reserve, native_reserve)?);
                    }
                }
                continue;
            }
            let frame = stack.pop().expect("copy frame exists");
            let frame_request = core::mem::size_of::<DropFrame>();
            before_reserve(frame_request).map_err(IdentityCopyError::Policy)?;
            if !native_reserve(frame_request) {
                return Err(IdentityCopyError::Allocation {
                    requested: frame_request,
                });
            }
            let drop_frame = Box::try_new(DropFrame {
                pending: Vec::new(),
                next: None,
            })
            .map_err(|_| IdentityCopyError::Allocation {
                requested: frame_request,
            })?;
            let arc_request = node_arc_bytes().map_err(|_| IdentityCopyError::Capacity {
                requested: usize::MAX,
            })?;
            before_reserve(arc_request).map_err(IdentityCopyError::Policy)?;
            if !native_reserve(arc_request) {
                return Err(IdentityCopyError::Allocation {
                    requested: arc_request,
                });
            }
            let inner = Arc::try_new(NodeData {
                package: frame.package,
                path: frame.path,
                ordinal: frame.source.inner.ordinal,
                structural_hash: frame.source.inner.structural_hash,
                token_count: frame.source.inner.token_count,
                drop_frame: Some(drop_frame),
            })
            .map_err(|_| IdentityCopyError::Allocation {
                requested: arc_request,
            })?;
            let copied = Self { inner };
            if let Some(parent) = stack.last_mut() {
                parent.path.push(FlatSegment::Node(copied));
            } else {
                return Ok(copied);
            }
        }
    }

    fn try_clone_legacy_with(
        &self,
        before_reserve: &mut impl FnMut(usize) -> Result<(), InvalidNodeKey>,
    ) -> Result<Self, InvalidNodeKey> {
        self.try_clone_with_native_hook(before_reserve, &mut |_| true)
            .map_err(|error| match error {
                IdentityCopyError::Policy(error) => error,
                IdentityCopyError::Allocation { requested } => {
                    InvalidNodeKey::Allocation { requested }
                }
                IdentityCopyError::Capacity { .. } => InvalidNodeKey::Allocation {
                    requested: usize::MAX,
                },
            })
    }

    #[cfg(test)]
    pub(crate) fn fixture(byte: u8) -> Self {
        use alloc::string::ToString;

        Self::new("test", Vec::from([PathSegment::Text(byte.to_string())]), 0)
            .expect("test key has valid structural components")
    }
}

impl fmt::Display for NodeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        enum Part<'a> {
            Node(&'a NodeKey),
            Segment(&'a FlatSegment),
            End(u64),
            CloseSegment,
        }
        let mut pending = Vec::new();
        pending.push(Part::Node(self));
        while let Some(part) = pending.pop() {
            match part {
                Part::Node(node) => {
                    write!(f, "{}:[", node.inner.package)?;
                    pending.push(Part::End(node.inner.ordinal));
                    for segment in node.inner.path.iter().rev() {
                        pending.push(Part::Segment(segment));
                    }
                }
                Part::Segment(FlatSegment::Text(text)) => write!(f, "{text:?},")?,
                Part::Segment(FlatSegment::Node(node)) => {
                    f.write_str("node(")?;
                    pending.push(Part::CloseSegment);
                    pending.push(Part::Node(node));
                }
                Part::End(ordinal) => write!(f, "]:{ordinal}")?,
                Part::CloseSegment => f.write_str("),")?,
            }
        }
        Ok(())
    }
}

impl fmt::Debug for NodeKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NodeKey({self})")
    }
}

#[cfg(test)]
mod tests {
    use ix_trace_rs::trace;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use super::*;

    fn key(package: &str, path: Vec<PathSegment>, ordinal: u64) -> NodeKey {
        NodeKey::new(package, path, ordinal).unwrap()
    }

    /// Trace: FR-389-AC-7, TC-938.
    #[trace("FR-389-AC-7", "TC-938")]
    #[test]
    fn display_closes_only_nested_node_segments() {
        let leaf = key("Leaf", vec![PathSegment::Text("name".into())], 2);
        assert_eq!(alloc::format!("{leaf}"), "Leaf:[\"name\",]:2");
        assert_eq!(alloc::format!("{leaf:?}"), "NodeKey(Leaf:[\"name\",]:2)");

        let root = key(
            "Root",
            vec![PathSegment::Node(leaf), PathSegment::Text("tail".into())],
            3,
        );
        assert_eq!(
            alloc::format!("{root}"),
            "Root:[node(Leaf:[\"name\",]:2),\"tail\",]:3"
        );
        assert_eq!(
            alloc::format!("{root:?}"),
            "NodeKey(Root:[node(Leaf:[\"name\",]:2),\"tail\",]:3)"
        );
    }

    /// Trace: FR-391-AC-3, FR-391-AC-5, TC-940.
    #[trace("FR-391-AC-3", "FR-391-AC-5", "TC-940")]
    #[test]
    fn policy_node_copy_checks_counts_and_retains_legacy_overflow() {
        let token_size = core::mem::size_of::<FlatSegment>();
        let max_tokens = usize::MAX / token_size;
        assert_eq!(
            array_bytes::<FlatSegment>(max_tokens + 1),
            Err(IdentityCopyError::Capacity {
                requested: usize::MAX
            })
        );
        assert_eq!(
            array_bytes::<FlatSegment>(max_tokens),
            Err(IdentityCopyError::Capacity {
                requested: max_tokens * token_size
            })
        );
        assert_eq!(
            flat_reservation_bytes(max_tokens + 1),
            Err(InvalidNodeKey::Allocation {
                requested: usize::MAX
            })
        );
    }

    /// Trace: FR-391-AC-5, TC-940.
    #[trace("FR-391-AC-5", "TC-940")]
    #[test]
    fn policy_node_copy_reports_each_real_request_and_preserves_source() {
        let nested = key("nested", vec![PathSegment::Text("inner".into())], 2);
        let source = key(
            "outer",
            vec![PathSegment::Text("first".into()), PathSegment::Node(nested)],
            1,
        );
        let token_bytes = core::mem::size_of::<FlatSegment>();
        let frame_bytes = core::mem::size_of::<CopyFrame<'_>>();
        let drop_bytes = core::mem::size_of::<DropFrame>();
        let arc_bytes = node_arc_bytes().unwrap();
        let requests = [
            5,
            2 * token_bytes,
            frame_bytes,
            5,
            6,
            token_bytes,
            frame_bytes,
            5,
            drop_bytes,
            arc_bytes,
            drop_bytes,
            arc_bytes,
        ];
        let copied = source
            .try_clone_with(&mut |requested| {
                assert!(requested > 0);
                Ok::<_, ()>(())
            })
            .unwrap();
        assert_eq!(copied, source);
        let mut original_hash = DefaultHasher::new();
        source.hash(&mut original_hash);
        let mut copied_hash = DefaultHasher::new();
        copied.hash(&mut copied_hash);
        assert_eq!(original_hash.finish(), copied_hash.finish());
        assert_eq!(source.try_clone().unwrap(), source);
        for denied_at in 0..requests.len() {
            let mut seen = 0;
            let result = source.try_clone_with(&mut |requested| {
                assert_eq!(requested, requests[seen]);
                let deny = seen == denied_at;
                seen += 1;
                if deny {
                    Err("policy denied")
                } else {
                    Ok(())
                }
            });
            assert_eq!(result, Err(IdentityCopyError::Policy("policy denied")));
            assert_eq!(seen, denied_at + 1);
            let mut native_seen = 0;
            let result =
                source.try_clone_with_native_hook(&mut |_| Ok::<_, ()>(()), &mut |requested| {
                    assert_eq!(requested, requests[native_seen]);
                    let permit = native_seen != denied_at;
                    native_seen += 1;
                    permit
                });
            assert_eq!(
                result,
                Err(IdentityCopyError::Allocation {
                    requested: requests[denied_at]
                })
            );
            assert_eq!(native_seen, denied_at + 1);
            assert_eq!(source, copied);
        }
        let empty_text = key("p", vec![PathSegment::Text(String::new())], 0);
        let mut requests_seen = Vec::new();
        assert_eq!(
            empty_text
                .try_clone_with(&mut |requested| {
                    requests_seen.push(requested);
                    Ok::<_, ()>(())
                })
                .unwrap(),
            empty_text
        );
        assert_eq!(
            requests_seen,
            [1, token_bytes, frame_bytes, drop_bytes, arc_bytes]
        );
    }

    /// Trace: FR-389-AC-1, TC-938.
    #[trace("FR-389-AC-1", "TC-938")]
    #[test]
    fn structural_node_components_determine_equality_hash_and_order() {
        use std::collections::HashSet;

        let path = vec![PathSegment::Text("Type".into())];
        let a = key("Pkg", path.clone(), 0);
        let b = key("Pkg", path.clone(), 0);
        let c = key("Pkg", path, 1);
        assert_eq!(a, b);
        assert_eq!(a.cmp(&b), core::cmp::Ordering::Equal);
        assert_eq!(HashSet::from([a.clone(), b, c.clone()]).len(), 2);
        assert!(a < c);
        assert!(key("A", vec![PathSegment::Text("Type".into())], 9) < a);
        assert_ne!(
            key("owner", vec![PathSegment::Text("Type".into())], 0),
            key("importer", vec![PathSegment::Text("Type".into())], 0)
        );
        assert!(
            key("Pkg", vec![PathSegment::Text("Type".into())], 0)
                < key("Pkg", vec![PathSegment::Node(a.clone())], 0)
        );
        assert!(
            key(
                "Pkg",
                vec![PathSegment::Node(key(
                    "Pkg",
                    vec![PathSegment::Text("A".into())],
                    0,
                ))],
                0,
            ) < key(
                "Pkg",
                vec![PathSegment::Node(key(
                    "Pkg",
                    vec![PathSegment::Text("B".into())],
                    0,
                ))],
                0,
            )
        );
        assert!(
            key(
                "Pkg",
                vec![PathSegment::Node(key(
                    "Pkg",
                    vec![PathSegment::Text("A".into())],
                    0,
                ))],
                0,
            ) < key(
                "Pkg",
                vec![PathSegment::Node(key(
                    "Pkg",
                    vec![PathSegment::Text("A".into())],
                    1,
                ))],
                0,
            )
        );
        assert!(
            a < key(
                "Pkg",
                vec![
                    PathSegment::Text("Type".into()),
                    PathSegment::Text("".into())
                ],
                0
            )
        );
        let nested = PathSegment::Node(key("Pkg", vec![PathSegment::Text("child".into())], 0));
        assert!(
            key(
                "Pkg",
                vec![nested.clone(), PathSegment::Text("a".into())],
                0
            ) < key("Pkg", vec![nested, PathSegment::Text("b".into())], 0)
        );
    }

    fn assert_unallocated_order(left: &NodeKey, right: &NodeKey, expected: Ordering) {
        let mut actual = None;
        let allocations = allocation_counter::measure(|| actual = Some(left.cmp(right)));
        assert_eq!(allocations.count_total, 0);
        assert_eq!(allocations.bytes_total, 0);
        assert_eq!(actual, Some(expected));

        let mut partial = None;
        let allocations = allocation_counter::measure(|| partial = left.partial_cmp(right));
        assert_eq!(allocations.count_total, 0);
        assert_eq!(allocations.bytes_total, 0);
        assert_eq!(partial, Some(expected));
    }

    /// Trace: FR-389-AC-13, TC-938.
    #[trace("FR-389-AC-13", "TC-938")]
    #[test]
    fn structural_order_uses_no_heap_or_depth_dependent_stack() {
        let text = |value: &str| PathSegment::Text(value.into());
        let base = key("p", vec![text("a")], 0);
        let equal = key("p", vec![text("a")], 0);
        let pairs = [
            (base.clone(), equal, Ordering::Equal),
            (base.clone(), key("q", vec![text("a")], 0), Ordering::Less),
            (base.clone(), key("p", vec![text("b")], 0), Ordering::Less),
            (
                base.clone(),
                key("p", vec![PathSegment::Node(base.clone())], 0),
                Ordering::Less,
            ),
            (
                base.clone(),
                key("p", vec![text("a"), text("b")], 0),
                Ordering::Less,
            ),
            (base.clone(), key("p", vec![text("a")], 1), Ordering::Less),
            (
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("x")], 0))],
                    0,
                ),
                key(
                    "p",
                    vec![PathSegment::Node(key("b", vec![text("x")], 0))],
                    0,
                ),
                Ordering::Less,
            ),
            (
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("a")], 0))],
                    0,
                ),
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("b")], 0))],
                    0,
                ),
                Ordering::Less,
            ),
            (
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("a")], 0))],
                    0,
                ),
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("a"), text("b")], 0))],
                    0,
                ),
                Ordering::Less,
            ),
            (
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("a")], 0))],
                    0,
                ),
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("a")], 1))],
                    0,
                ),
                Ordering::Less,
            ),
            (
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("a")], 0)), text("a")],
                    0,
                ),
                key(
                    "p",
                    vec![PathSegment::Node(key("a", vec![text("a")], 0)), text("b")],
                    0,
                ),
                Ordering::Less,
            ),
        ];
        for (left, right, expected) in &pairs {
            let before_left = left.clone();
            let before_right = right.clone();
            assert_unallocated_order(left, right, *expected);
            assert_unallocated_order(right, left, expected.reverse());
            assert_eq!(left, &before_left);
            assert_eq!(right, &before_right);
            if expected.is_eq() {
                assert_eq!(left, right);
                let mut left_hash = DefaultHasher::new();
                let mut right_hash = DefaultHasher::new();
                left.hash(&mut left_hash);
                right.hash(&mut right_hash);
                assert_eq!(left_hash.finish(), right_hash.finish());
            }
            let rebuilt = key(left.package(), left.path(), left.ordinal());
            assert_eq!(rebuilt, *left);
            assert_unallocated_order(&rebuilt, left, Ordering::Equal);
        }

        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                let mut left = key("leaf", vec![PathSegment::Text("a".into())], 0);
                let mut equal = key("leaf", vec![PathSegment::Text("a".into())], 0);
                let mut differing = key("leaf", vec![PathSegment::Text("b".into())], 0);
                for _ in 0..10_000 {
                    left = key("outer", vec![PathSegment::Node(left)], 0);
                    equal = key("outer", vec![PathSegment::Node(equal)], 0);
                    differing = key("outer", vec![PathSegment::Node(differing)], 0);
                }
                assert_unallocated_order(&left, &equal, Ordering::Equal);
                assert_unallocated_order(&equal, &left, Ordering::Equal);
                assert_unallocated_order(&left, &differing, Ordering::Less);
                assert_unallocated_order(&differing, &left, Ordering::Greater);
                assert_eq!(left, equal);
                let mut left_hash = DefaultHasher::new();
                let mut equal_hash = DefaultHasher::new();
                left.hash(&mut left_hash);
                equal.hash(&mut equal_hash);
                assert_eq!(left_hash.finish(), equal_hash.finish());
                assert_ne!(left, differing);
                let rebuilt = key(left.package(), left.path(), left.ordinal());
                assert_eq!(rebuilt, left);
                assert_unallocated_order(&rebuilt, &left, Ordering::Equal);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// Trace: FR-389-AC-13, FR-389-AC-14, TC-938.
    #[trace("FR-389-AC-13", "FR-389-AC-14", "TC-938")]
    #[test]
    fn mixed_path_comparison_preserves_order_past_inline_depth() {
        let nested = |leaf: &str, depth: usize| {
            let mut node = key("leaf", vec![PathSegment::Text(leaf.into())], 0);
            for _ in 0..depth {
                node = key(
                    "outer",
                    vec![PathSegment::Node(node), PathSegment::Text("tail".into())],
                    0,
                );
            }
            node
        };

        let depth = COMPARE_INLINE_DEPTH + 1;
        let left = nested("a", depth);
        let equal = nested("a", depth);
        let greater = nested("b", depth);
        assert_unallocated_order(&left, &equal, Ordering::Equal);
        assert_unallocated_order(&left, &greater, Ordering::Less);
        assert_eq!(left, equal);
        assert_ne!(left, greater);

        let mut suffix_left = key("leaf", vec![PathSegment::Text("same".into())], 0);
        let mut suffix_right = key("leaf", vec![PathSegment::Text("same".into())], 0);
        for level in 0..depth {
            let (left_tail, right_tail) = if level == 0 { ("a", "b") } else { ("z", "a") };
            suffix_left = key(
                "outer",
                vec![
                    PathSegment::Node(suffix_left),
                    PathSegment::Text(left_tail.into()),
                ],
                0,
            );
            suffix_right = key(
                "outer",
                vec![
                    PathSegment::Node(suffix_right),
                    PathSegment::Text(right_tail.into()),
                ],
                0,
            );
        }
        assert_unallocated_order(&suffix_left, &suffix_right, Ordering::Less);

        let sibling = key("sibling", vec![PathSegment::Text("same".into())], 0);
        let branching = |leaf: &str| {
            let mut node = key("leaf", vec![PathSegment::Text(leaf.into())], 0);
            for _ in 0..depth {
                node = key(
                    "outer",
                    vec![PathSegment::Node(node), PathSegment::Node(sibling.clone())],
                    0,
                );
            }
            node
        };
        let branching_left = branching("a");
        let branching_equal = branching("a");
        let branching_greater = branching("b");
        assert_unallocated_order(&branching_left, &branching_equal, Ordering::Equal);
        assert_unallocated_order(&branching_left, &branching_greater, Ordering::Less);

        let mut sibling_left = key("leaf", vec![PathSegment::Text("same".into())], 0);
        let mut sibling_right = key("leaf", vec![PathSegment::Text("same".into())], 0);
        for level in 0..depth {
            let (left_name, right_name) = if level == 0 { ("a", "b") } else { ("z", "a") };
            let left_sibling = key("sibling", vec![PathSegment::Text(left_name.into())], 0);
            let right_sibling = key("sibling", vec![PathSegment::Text(right_name.into())], 0);
            sibling_left = key(
                "outer",
                vec![
                    PathSegment::Node(sibling_left),
                    PathSegment::Node(left_sibling),
                ],
                0,
            );
            sibling_right = key(
                "outer",
                vec![
                    PathSegment::Node(sibling_right),
                    PathSegment::Node(right_sibling),
                ],
                0,
            );
        }
        assert_unallocated_order(&sibling_left, &sibling_right, Ordering::Less);

        let prefixed = |prefix: &str, leaf: &str| {
            let mut node = key("leaf", vec![PathSegment::Text(leaf.into())], 0);
            for _ in 0..depth {
                node = key(
                    "outer",
                    vec![
                        PathSegment::Text(prefix.into()),
                        PathSegment::Node(node),
                        PathSegment::Text("tail".into()),
                    ],
                    0,
                );
            }
            node
        };
        let prefixed_left = prefixed("a", "b");
        let prefixed_equal = prefixed("a", "b");
        let prefixed_greater = prefixed("a", "c");
        let prefix_greater = prefixed("b", "a");
        assert_unallocated_order(&prefixed_left, &prefixed_equal, Ordering::Equal);
        assert_unallocated_order(&prefixed_left, &prefixed_greater, Ordering::Less);
        assert_unallocated_order(&prefixed_left, &prefix_greater, Ordering::Less);

        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(move || {
                let left = nested("a", 10_000);
                let equal = nested("a", 10_000);
                let greater = nested("b", 10_000);
                assert_unallocated_order(&left, &equal, Ordering::Equal);
                assert_unallocated_order(&left, &greater, Ordering::Less);
            })
            .unwrap()
            .join()
            .unwrap();

        let shared = nested("a", COMPARE_INLINE_DEPTH);
        let left = key(
            "owner",
            vec![
                PathSegment::Node(shared.clone()),
                PathSegment::Text("a".into()),
            ],
            0,
        );
        let right = key(
            "owner",
            vec![PathSegment::Node(shared), PathSegment::Text("b".into())],
            0,
        );
        assert_unallocated_order(&left, &right, Ordering::Less);
    }

    /// Trace: FR-389-AC-13, TC-938.
    #[trace("FR-389-AC-13", "TC-938")]
    #[test]
    #[ignore = "bounded performance lane; run on a quiet host"]
    fn mixed_path_comparison_has_bounded_time_past_inline_depth() {
        use std::time::{Duration, Instant};

        let mut left = key("leaf", vec![PathSegment::Text("a".into())], 0);
        let mut right = key("leaf", vec![PathSegment::Text("a".into())], 0);
        for _ in 0..4_000 {
            left = key(
                "outer",
                vec![PathSegment::Node(left), PathSegment::Text("tail".into())],
                0,
            );
            right = key(
                "outer",
                vec![PathSegment::Node(right), PathSegment::Text("tail".into())],
                0,
            );
        }
        let mut prefixed_left = key("leaf", vec![PathSegment::Text("a".into())], 0);
        let mut prefixed_right = key("leaf", vec![PathSegment::Text("a".into())], 0);
        for _ in 0..4_000 {
            prefixed_left = key(
                "outer",
                vec![
                    PathSegment::Text("prefix".into()),
                    PathSegment::Node(prefixed_left),
                    PathSegment::Text("tail".into()),
                ],
                0,
            );
            prefixed_right = key(
                "outer",
                vec![
                    PathSegment::Text("prefix".into()),
                    PathSegment::Node(prefixed_right),
                    PathSegment::Text("tail".into()),
                ],
                0,
            );
        }
        let sibling = key("sibling", vec![PathSegment::Text("same".into())], 0);
        let mut branching_left = key("leaf", vec![PathSegment::Text("a".into())], 0);
        let mut branching_right = key("leaf", vec![PathSegment::Text("a".into())], 0);
        for _ in 0..4_000 {
            branching_left = key(
                "outer",
                vec![
                    PathSegment::Node(branching_left),
                    PathSegment::Node(sibling.clone()),
                ],
                0,
            );
            branching_right = key(
                "outer",
                vec![
                    PathSegment::Node(branching_right),
                    PathSegment::Node(sibling.clone()),
                ],
                0,
            );
        }
        let start = Instant::now();
        for _ in 0..100 {
            assert_eq!(left.cmp(&right), Ordering::Equal);
            assert_eq!(prefixed_left.cmp(&prefixed_right), Ordering::Equal);
            assert_eq!(branching_left.cmp(&branching_right), Ordering::Equal);
        }
        assert!(
            start.elapsed() < Duration::from_secs(2),
            "300 depth-4000 comparisons exceeded the bounded performance lane"
        );
    }

    /// Trace: FR-389-AC-2, TC-938.
    #[trace("FR-389-AC-2", "TC-938")]
    #[test]
    fn malformed_structural_node_shape_refuses() {
        let path = vec![PathSegment::Text(String::new())];
        assert_eq!(
            NodeKey::new("", path.clone(), 0),
            Err(InvalidNodeKey::EmptyPackage)
        );
        assert_eq!(
            NodeKey::new("Pkg", vec![], 0),
            Err(InvalidNodeKey::EmptyPath)
        );
        assert_eq!(
            NodeKey::new("Pkg", path.clone(), MAX_NODE_ORDINAL + 1),
            Err(InvalidNodeKey::OrdinalOutOfRange)
        );
        assert_eq!(
            key("Pkg", path, MAX_NODE_ORDINAL).ordinal(),
            MAX_NODE_ORDINAL
        );
    }

    /// Trace: FR-389-AC-9, FR-389-AC-10, TC-938.
    #[trace("FR-389-AC-9", "FR-389-AC-10", "TC-938")]
    #[test]
    fn borrowed_path_preserves_direct_components_at_depth() {
        let child = key("nested", vec![PathSegment::Text("leaf".into())], 7);
        let parent = key(
            "owner",
            vec![
                PathSegment::Text(String::new()),
                PathSegment::Node(child),
                PathSegment::Text("tail".into()),
            ],
            9,
        );
        let view = parent.view();
        assert_eq!((view.package(), view.ordinal()), ("owner", 9));
        let mut path = view.path();
        assert!(matches!(path.next(), Some(PathSegmentView::Text(""))));
        let Some(PathSegmentView::Node(nested)) = path.next() else {
            panic!("second direct segment is nested");
        };
        assert_eq!((nested.package(), nested.ordinal()), ("nested", 7));
        assert!(matches!(
            nested.path().next(),
            Some(PathSegmentView::Text("leaf"))
        ));
        assert!(matches!(path.next(), Some(PathSegmentView::Text("tail"))));
        assert!(path.next().is_none());
        let allocations = allocation_counter::measure(|| {
            let mut segments = 0;
            for segment in parent.view().path() {
                segments += 1;
                if let PathSegmentView::Node(child) = segment {
                    assert_eq!(child.path().count(), 1);
                }
            }
            assert_eq!(segments, 3);
        });
        assert_eq!(allocations.count_total, 0);

        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                let mut node = key("root-owner", vec![PathSegment::Text("root".into())], 17);
                for depth in 0..10_000 {
                    let package = if depth % 2 == 0 { "even" } else { "odd" };
                    node = key(package, vec![PathSegment::Node(node)], depth);
                }
                let copy = node.try_clone().unwrap();
                assert_eq!(copy, node);
                let segment = PathSegment::Node(copy);
                assert_eq!(segment.try_clone().unwrap(), segment);
                let mut pending = Vec::new();
                pending.try_reserve(1).unwrap();
                pending.push((node.view(), 10_000));
                let mut nodes = 0;
                while let Some((view, depth)) = pending.pop() {
                    nodes += 1;
                    let mut path = view.path();
                    if depth == 0 {
                        assert_eq!((view.package(), view.ordinal()), ("root-owner", 17));
                        assert!(matches!(path.next(), Some(PathSegmentView::Text("root"))));
                    } else {
                        let source_depth = depth - 1;
                        let expected_package = if source_depth % 2 == 0 { "even" } else { "odd" };
                        assert_eq!(
                            (view.package(), view.ordinal()),
                            (expected_package, source_depth)
                        );
                        let Some(PathSegmentView::Node(child)) = path.next() else {
                            panic!("depth {depth} must contain its nested child");
                        };
                        pending.try_reserve(1).unwrap();
                        pending.push((child, source_depth));
                    }
                    assert!(
                        path.next().is_none(),
                        "depth {depth} has an extra path segment"
                    );
                }
                assert_eq!(nodes, 10_001);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// Trace: FR-389-AC-10, TC-938.
    #[trace("FR-389-AC-10", "TC-938")]
    #[test]
    fn borrowed_consumer_refuses_failed_stack_or_output_reservation() {
        #[derive(Debug, Eq, PartialEq)]
        enum ConsumerAllocation {
            Stack,
            Output,
        }

        fn collect<'a>(
            view: NodeKeyView<'a>,
            fail_stack: bool,
            fail_output: bool,
        ) -> Result<Vec<&'a str>, ConsumerAllocation> {
            let mut stack = Vec::new();
            stack
                .try_reserve(if fail_stack { usize::MAX } else { 1 })
                .map_err(|_| ConsumerAllocation::Stack)?;
            stack.push(view);
            let mut output = Vec::new();
            while let Some(view) = stack.pop() {
                for segment in view.path() {
                    match segment {
                        PathSegmentView::Text(text) => {
                            output
                                .try_reserve(if fail_output { usize::MAX } else { 1 })
                                .map_err(|_| ConsumerAllocation::Output)?;
                            output.push(text);
                        }
                        PathSegmentView::Node(child) => {
                            stack
                                .try_reserve(1)
                                .map_err(|_| ConsumerAllocation::Stack)?;
                            stack.push(child);
                        }
                    }
                }
            }
            Ok(output)
        }

        let node = key("owner", vec![PathSegment::Text("name".into())], 0);
        assert_eq!(collect(node.view(), false, false), Ok(vec!["name"]));
        assert_eq!(
            collect(node.view(), true, false),
            Err(ConsumerAllocation::Stack)
        );
        assert_eq!(
            collect(node.view(), false, true),
            Err(ConsumerAllocation::Output)
        );
        assert_eq!(node.view().path().count(), 1);
    }

    /// Trace: FR-389-AC-11, FR-389-AC-12, TC-938.
    #[trace("FR-389-AC-11", "FR-389-AC-12", "TC-938")]
    #[test]
    fn fallible_structural_construction_and_copy_preserve_identity() {
        let segment = PathSegment::Node(key(
            "child",
            vec![PathSegment::Text("text".into())],
            MAX_NODE_ORDINAL,
        ));
        let node = NodeKey::try_new(
            "owner".into(),
            vec![PathSegment::Text("".into()), segment.try_clone().unwrap()],
            0,
        )
        .unwrap();
        let infallible = NodeKey::new(
            "owner",
            vec![PathSegment::Text("".into()), segment.try_clone().unwrap()],
            0,
        )
        .unwrap();
        assert_eq!(node, infallible);
        let upper = NodeKey::try_new(
            "owner".into(),
            vec![segment.try_clone().unwrap()],
            MAX_NODE_ORDINAL,
        )
        .unwrap();
        assert_eq!(upper.ordinal(), MAX_NODE_ORDINAL);
        let copied = node.try_clone().unwrap();
        assert_eq!(copied, node);
        assert_eq!(copied.cmp(&node), Ordering::Equal);
        assert_eq!(segment.try_clone().unwrap(), segment);
        let text = PathSegment::Text("text".into());
        assert_eq!(text.try_clone().unwrap(), text);
        let hash = |key: &NodeKey| {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(hash(&copied), hash(&node));
        assert_eq!(
            NodeKey::try_new(String::new(), vec![segment.try_clone().unwrap()], 0),
            Err(InvalidNodeKey::EmptyPackage)
        );
        assert_eq!(
            NodeKey::try_new("owner".into(), Vec::new(), 0),
            Err(InvalidNodeKey::EmptyPath)
        );
        assert_eq!(
            NodeKey::try_new("owner".into(), vec![segment], MAX_NODE_ORDINAL + 1),
            Err(InvalidNodeKey::OrdinalOutOfRange)
        );
    }

    /// Trace: FR-389-AC-11, FR-389-AC-12, TC-938.
    #[trace("FR-389-AC-11", "FR-389-AC-12", "TC-938")]
    #[test]
    fn each_fallible_reservation_refuses_without_changing_source() {
        let child = key("child", vec![PathSegment::Text("text".into())], 0);
        let source = key(
            "owner",
            vec![
                PathSegment::Text(String::new()),
                PathSegment::Node(child.clone()),
            ],
            0,
        );
        let original = source.clone();
        let token_bytes = core::mem::size_of::<FlatSegment>();
        let frame_bytes = core::mem::size_of::<CopyFrame<'_>>();
        let drop_bytes = core::mem::size_of::<DropFrame>();
        let arc_bytes = node_arc_bytes().unwrap();
        let new_requests = [2 * token_bytes, drop_bytes, arc_bytes];
        for fail_at in 0..new_requests.len() {
            let mut call = 0;
            let result = NodeKey::try_new_with(
                "owner".into(),
                vec![
                    PathSegment::Text(String::new()),
                    PathSegment::Node(child.clone()),
                ],
                0,
                &mut |requested| {
                    assert_eq!(requested, new_requests[call]);
                    let reject = call == fail_at;
                    call += 1;
                    if reject {
                        Err(InvalidNodeKey::Allocation { requested })
                    } else {
                        Ok(())
                    }
                },
            );
            assert_eq!(
                result,
                Err(InvalidNodeKey::Allocation {
                    requested: new_requests[fail_at]
                })
            );
            assert_eq!(call, fail_at + 1);
        }
        let clone_requests = [
            5,
            2 * token_bytes,
            frame_bytes,
            5,
            token_bytes,
            frame_bytes,
            4,
            drop_bytes,
            arc_bytes,
            drop_bytes,
            arc_bytes,
        ];
        for fail_at in 0..clone_requests.len() {
            let mut call = 0;
            let result = source.try_clone_legacy_with(&mut |requested| {
                assert_eq!(requested, clone_requests[call]);
                let reject = call == fail_at;
                call += 1;
                if reject {
                    Err(InvalidNodeKey::Allocation { requested })
                } else {
                    Ok(())
                }
            });
            assert_eq!(
                result,
                Err(InvalidNodeKey::Allocation {
                    requested: clone_requests[fail_at]
                })
            );
            assert_eq!(call, fail_at + 1);
            assert_eq!(source, original);
        }
        let segment = PathSegment::Node(source.clone());
        for fail_at in 0..clone_requests.len() {
            let mut call = 0;
            let result = segment.try_clone_legacy_with(&mut |requested| {
                assert_eq!(requested, clone_requests[call]);
                let reject = call == fail_at;
                call += 1;
                if reject {
                    Err(InvalidNodeKey::Allocation { requested })
                } else {
                    Ok(())
                }
            });
            assert_eq!(
                result,
                Err(InvalidNodeKey::Allocation {
                    requested: clone_requests[fail_at]
                })
            );
            assert_eq!(call, fail_at + 1);
            assert_eq!(segment, PathSegment::Node(original.clone()));
        }
        let text = PathSegment::Text("text".into());
        assert_eq!(
            text.try_clone_legacy_with(&mut |requested| {
                assert_eq!(requested, 4);
                Err(InvalidNodeKey::Allocation { requested })
            }),
            Err(InvalidNodeKey::Allocation { requested: 4 })
        );
        assert_eq!(text, PathSegment::Text("text".into()));
        let empty = PathSegment::Text(String::new());
        assert_eq!(
            empty.try_clone_legacy_with(&mut |_| panic!("empty text never reserves")),
            Ok(empty)
        );
        let max_tokens = usize::MAX / token_bytes;
        assert_eq!(
            flat_reservation_bytes(max_tokens),
            Ok(max_tokens * token_bytes)
        );
        assert_eq!(
            flat_reservation_bytes(max_tokens + 1),
            Err(InvalidNodeKey::Allocation {
                requested: usize::MAX
            })
        );
        assert_eq!(
            nested_token_count(usize::MAX),
            Err(InvalidNodeKey::Allocation {
                requested: usize::MAX
            })
        );
        assert_eq!(
            NodeKey::try_new_with(
                String::new(),
                vec![PathSegment::Text("x".into())],
                0,
                &mut |_| { panic!("shape refusal precedes reservation") }
            ),
            Err(InvalidNodeKey::EmptyPackage)
        );
        assert_eq!(
            NodeKey::try_new_with("owner".into(), Vec::new(), 0, &mut |_| {
                panic!("shape refusal precedes reservation")
            }),
            Err(InvalidNodeKey::EmptyPath)
        );
        assert_eq!(
            NodeKey::try_new_with(
                "owner".into(),
                vec![PathSegment::Text("x".into())],
                MAX_NODE_ORDINAL + 1,
                &mut |_| panic!("shape refusal precedes reservation")
            ),
            Err(InvalidNodeKey::OrdinalOutOfRange)
        );
    }

    #[trace("TC-938", "FR-389-AC-1", "FR-389-AC-7")]
    #[test]
    fn deep_nested_path_clones_compares_hashes_formats_and_drops_on_small_stack() {
        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                let mut node = key("Pkg", vec![PathSegment::Text("root".into())], 0);
                for _ in 0..10_000 {
                    node = key("Pkg", vec![PathSegment::Node(node)], 0);
                }
                let copy = node.clone();
                assert_eq!(node, copy);
                assert_eq!(node.cmp(&copy), core::cmp::Ordering::Equal);
                let mut changed = key("Pkg", vec![PathSegment::Text("root2".into())], 0);
                for _ in 0..10_000 {
                    changed = key("Pkg", vec![PathSegment::Node(changed)], 0);
                }
                assert!(node < changed);
                let hash = |value: &NodeKey| {
                    let mut hasher = DefaultHasher::new();
                    value.hash(&mut hasher);
                    hasher.finish()
                };
                assert_eq!(hash(&node), hash(&copy));
                assert!(alloc::format!("{node:?}").starts_with("NodeKey("));
                let rebuilt = key(node.package(), node.path(), node.ordinal());
                assert_eq!(rebuilt, node);
                let [PathSegment::Node(parent)] =
                    <[PathSegment; 1]>::try_from(node.path()).unwrap()
                else {
                    panic!("one nested path segment")
                };
                assert_eq!(parent.path().len(), 1);
                drop(parent);
                drop(copy);
                drop(changed);
                drop(rebuilt);
                drop(node);
            })
            .expect("spawn small stack")
            .join()
            .expect("deep path operations complete");
    }

    /// Trace: FR-389-AC-1, FR-389-AC-11, FR-389-AC-12, FR-389-AC-14, TC-938.
    #[trace(
        "FR-389-AC-1",
        "FR-389-AC-11",
        "FR-389-AC-12",
        "FR-389-AC-14",
        "TC-938"
    )]
    #[test]
    fn deep_structural_hash_writes_one_word_for_shared_and_independent_keys() {
        struct CountingHasher {
            bytes: usize,
        }

        impl Hasher for CountingHasher {
            fn finish(&self) -> u64 {
                self.bytes as u64
            }

            fn write(&mut self, bytes: &[u8]) {
                self.bytes += bytes.len();
            }
        }

        fn chain(depth: usize) -> NodeKey {
            let mut node = key("leaf", vec![PathSegment::Text("x".into())], 0);
            for _ in 0..depth {
                node = key("owner", vec![PathSegment::Node(node)], 0);
            }
            node
        }

        fn hash_once(key: &NodeKey) -> (usize, u64) {
            let mut counted = CountingHasher { bytes: 0 };
            let allocations = allocation_counter::measure(|| key.hash(&mut counted));
            assert_eq!(allocations.count_total, 0, "hash must not allocate");
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            (counted.bytes, hasher.finish())
        }

        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                for depth in [1_000, 10_000] {
                    let shared = chain(depth);
                    let same = chain(depth);
                    let copied = shared.try_clone().expect("fallible copy of deep key");
                    assert_eq!(shared, same, "independent keys at depth {depth}");
                    assert_eq!(shared, copied, "copied key at depth {depth}");
                    let (written, expected_hash) = hash_once(&shared);
                    assert_eq!(
                        written,
                        core::mem::size_of::<u64>(),
                        "hash work at depth {depth}"
                    );
                    assert_eq!(hash_once(&same), (written, expected_hash));
                    assert_eq!(hash_once(&copied), (written, expected_hash));
                }
            })
            .expect("spawn small stack")
            .join()
            .expect("deep hash operations complete");
    }

    /// Trace: FR-389-AC-14, TC-938.
    #[trace("FR-389-AC-14", "TC-938")]
    #[test]
    fn shared_operand_construction_clone_and_concurrent_drop_are_depth_independent() {
        fn chain(depth: usize) -> NodeKey {
            let mut key = NodeKey::new("leaf", vec![PathSegment::Text("x".into())], 0).unwrap();
            for _ in 0..depth {
                key = NodeKey::new("owner", vec![PathSegment::Node(key)], 0).unwrap();
            }
            key
        }

        fn measured_derived(operand: NodeKey) -> (NodeKey, u64, u64, construction_probe::Counts) {
            let package = String::from("derived");
            let path = vec![PathSegment::Node(operand)];
            let mut result = None;
            construction_probe::reset();
            let allocations = allocation_counter::measure(|| {
                result = Some(NodeKey::new(package, path, 0).unwrap());
            });
            let counts = construction_probe::snapshot();
            (
                result.expect("derived key was constructed"),
                allocations.count_total,
                allocations.bytes_total,
                counts,
            )
        }

        fn write_json_string(output: &mut String, value: &str) {
            use core::fmt::Write;

            output.push('"');
            for character in value.chars() {
                match character {
                    '"' => output.push_str("\\\""),
                    '\\' => output.push_str("\\\\"),
                    '\u{0000}'..='\u{001f}' => {
                        write!(output, "\\u{:04x}", u32::from(character)).unwrap();
                    }
                    other => output.push(other),
                }
            }
            output.push('"');
        }

        fn encode_borrowed_node(view: NodeKeyView<'_>) -> String {
            use core::fmt::Write;

            enum Frame<'a> {
                Node(NodeKeyView<'a>),
                Path(NodePathIter<'a>, u64, bool),
            }

            let mut output = String::new();
            let mut stack = vec![Frame::Node(view)];
            while let Some(frame) = stack.pop() {
                match frame {
                    Frame::Node(node) => {
                        output.push_str("{\"package\":");
                        write_json_string(&mut output, node.package());
                        output.push_str(",\"path\":[");
                        stack.push(Frame::Path(node.path(), node.ordinal(), false));
                    }
                    Frame::Path(mut path, ordinal, already_wrote_segment) => {
                        if let Some(segment) = path.next() {
                            if already_wrote_segment {
                                output.push(',');
                            }
                            stack.push(Frame::Path(path, ordinal, true));
                            match segment {
                                PathSegmentView::Text(text) => {
                                    write_json_string(&mut output, text);
                                }
                                PathSegmentView::Node(child) => stack.push(Frame::Node(child)),
                            }
                        } else {
                            write!(output, "],\"ordinal\":{ordinal}}}").unwrap();
                        }
                    }
                }
            }
            output
        }

        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                let (shallow, shallow_count, shallow_bytes, shallow_visits) =
                    measured_derived(chain(1_000));
                let (deep, deep_count, deep_bytes, deep_visits) = measured_derived(chain(10_000));
                assert_eq!((shallow_count, shallow_bytes), (deep_count, deep_bytes));
                assert_eq!(deep_count, 3, "direct path, drop frame, shared payload");
                for visits in [shallow_visits, deep_visits] {
                    assert_eq!(visits.direct_visits, 1);
                    assert_eq!(visits.direct_moves, 1);
                    assert_eq!(visits.descendant_visits, 0);
                    assert_eq!(visits.descendant_copies, 0);
                }

                for source in [&shallow, &deep] {
                    let mut copied = None;
                    construction_probe::reset();
                    let allocations =
                        allocation_counter::measure(|| copied = Some((*source).clone()));
                    let visits = construction_probe::snapshot();
                    assert_eq!((allocations.count_total, allocations.bytes_total), (0, 0));
                    assert_eq!(visits.shallow_clones, 1);
                    assert_eq!(visits.descendant_visits, 0);
                    assert_eq!(visits.descendant_copies, 0);
                    assert!(Arc::ptr_eq(&source.inner, &copied.unwrap().inner));
                }
                let copied = deep.clone();
                assert!(Arc::ptr_eq(&deep.inner, &copied.inner));
                assert_eq!(deep, copied);

                let encoded = encode_borrowed_node(deep.view());
                let expected = [
                    "{\"package\":\"derived\",\"path\":[",
                    &"{\"package\":\"owner\",\"path\":[".repeat(10_000),
                    "{\"package\":\"leaf\",\"path\":[\"x\"],\"ordinal\":0}",
                    &"],\"ordinal\":0}".repeat(10_001),
                ]
                .concat();
                assert_eq!(encoded, expected);

                let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
                let workers: Vec<_> = [deep.clone(), copied]
                    .into_iter()
                    .map(|key| {
                        let barrier = barrier.clone();
                        std::thread::Builder::new()
                            .stack_size(512 * 1024)
                            .spawn(move || {
                                barrier.wait();
                                drop(key);
                            })
                            .unwrap()
                    })
                    .collect();
                barrier.wait();
                drop(deep);
                for worker in workers {
                    worker.join().unwrap();
                }
                drop(shallow);
            })
            .unwrap()
            .join()
            .unwrap();
    }

    /// A name that is not `^[A-Za-z_][A-Za-z0-9_]*$` is refused at
    /// `Identifier::new`.
    #[test]
    fn a_non_identifier_name_is_refused() {
        for invalid in ["not an id!", "", "1starts_with_digit", "has-a-dash"] {
            assert_eq!(
                Identifier::new(invalid),
                Err(InvalidIdentifier),
                "{invalid:?} must be refused"
            );
        }
    }

    /// The positive complement of the refusal above.
    #[test]
    fn ordinary_identifiers_are_accepted() {
        for valid in [
            "quantity",
            "source",
            "totalPrice",
            "add",
            "_leading_underscore",
        ] {
            assert!(Identifier::new(valid).is_ok(), "{valid:?} must be accepted");
        }
    }
}
