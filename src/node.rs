// SPDX-License-Identifier: AGPL-3.0-or-later
//! Checked semantic node identity.
//!
//! A `NodeKey` carries the structural identity of a node in an admitted package.
//! Its owner, path, and ordinal compare componentwise as QSpec FR-322 specifies.
//! The caller checks that the identity joins its admitted package; this leaf
//! checks only the closed shape and never computes a digest.

use alloc::{string::String, vec::Vec};
use core::{cmp::Ordering, fmt};

use crate::identity_copy::{array_bytes, copy_text, retype, IdentityCopyError};

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

/// A structural node identity from an admitted checked package.
///
/// quire:canonical
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct NodeKey {
    package: String,
    // Preorder tokens. Each nested node header gives the span of its subtree;
    // no owned token contains another token, so ordinary Clone and Drop are
    // depth independent.
    path: Vec<FlatSegment>,
    direct_len: usize,
    ordinal: u64,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum FlatSegment {
    Text(String),
    Node {
        package: String,
        direct_len: usize,
        ordinal: u64,
        span: usize,
    },
}

/// A borrowed structural node identity. Creating or traversing a view does not allocate.
#[derive(Clone, Copy)]
pub struct NodeKeyView<'a> {
    package: &'a str,
    path: &'a [FlatSegment],
    direct_len: usize,
    ordinal: u64,
}

impl<'a> NodeKeyView<'a> {
    /// The exact owning package name.
    pub fn package(self) -> &'a str {
        self.package
    }

    /// The ordinal within the owning package and path.
    pub fn ordinal(self) -> u64 {
        self.ordinal
    }

    /// Iterate over direct path segments without copying nested identities.
    pub fn path(self) -> NodePathIter<'a> {
        NodePathIter {
            path: self.path,
            index: 0,
            remaining: self.direct_len,
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
    path: &'a [FlatSegment],
    index: usize,
    remaining: usize,
}

impl<'a> Iterator for NodePathIter<'a> {
    type Item = PathSegmentView<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        match &self.path[self.index] {
            FlatSegment::Text(text) => {
                self.index += 1;
                Some(PathSegmentView::Text(text))
            }
            FlatSegment::Node {
                package,
                direct_len,
                ordinal,
                span,
            } => {
                let start = self.index + 1;
                self.index += *span;
                Some(PathSegmentView::Node(NodeKeyView {
                    package,
                    path: &self.path[start..self.index],
                    direct_len: *direct_len,
                    ordinal: *ordinal,
                }))
            }
        }
    }
}

struct CompareFrame<'a> {
    left: NodeKeyView<'a>,
    right: NodeKeyView<'a>,
    left_index: usize,
    right_index: usize,
    left_remaining: usize,
    right_remaining: usize,
}

impl<'a> CompareFrame<'a> {
    fn new(left: NodeKeyView<'a>, right: NodeKeyView<'a>) -> Self {
        Self {
            left,
            right,
            left_index: 0,
            right_index: 0,
            left_remaining: left.direct_len,
            right_remaining: right.direct_len,
        }
    }
}

impl Ord for NodeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut frames = Vec::new();
        frames.push(CompareFrame::new(self.view(), other.view()));
        while let Some(frame) = frames.last_mut() {
            match frame.left.package.cmp(frame.right.package) {
                Ordering::Equal => {}
                different => return different,
            }
            match (frame.left_remaining, frame.right_remaining) {
                (0, 0) => {
                    let order = frame.left.ordinal.cmp(&frame.right.ordinal);
                    if !order.is_eq() {
                        return order;
                    }
                    frames.pop();
                    continue;
                }
                (0, _) => return Ordering::Less,
                (_, 0) => return Ordering::Greater,
                _ => {}
            }
            let left = &frame.left.path[frame.left_index];
            let right = &frame.right.path[frame.right_index];
            match (left, right) {
                (FlatSegment::Text(a), FlatSegment::Text(b)) => {
                    let order = a.cmp(b);
                    if !order.is_eq() {
                        return order;
                    }
                    frame.left_index += 1;
                    frame.right_index += 1;
                    frame.left_remaining -= 1;
                    frame.right_remaining -= 1;
                }
                (FlatSegment::Text(_), FlatSegment::Node { .. }) => return Ordering::Less,
                (FlatSegment::Node { .. }, FlatSegment::Text(_)) => return Ordering::Greater,
                (
                    FlatSegment::Node {
                        package: left_package,
                        direct_len: left_len,
                        ordinal: left_ordinal,
                        span: left_span,
                    },
                    FlatSegment::Node {
                        package: right_package,
                        direct_len: right_len,
                        ordinal: right_ordinal,
                        span: right_span,
                    },
                ) => {
                    let left_path = frame.left.path;
                    let right_path = frame.right.path;
                    let left_node = NodeKeyView {
                        package: left_package,
                        path: &left_path[frame.left_index + 1..frame.left_index + *left_span],
                        direct_len: *left_len,
                        ordinal: *left_ordinal,
                    };
                    let right_node = NodeKeyView {
                        package: right_package,
                        path: &right_path[frame.right_index + 1..frame.right_index + *right_span],
                        direct_len: *right_len,
                        ordinal: *right_ordinal,
                    };
                    frame.left_index += *left_span;
                    frame.right_index += *right_span;
                    frame.left_remaining -= 1;
                    frame.right_remaining -= 1;
                    frames.push(CompareFrame::new(left_node, right_node));
                }
            }
        }
        Ordering::Equal
    }
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
        let direct_len = path.len();
        let mut flat = Vec::new();
        for segment in path {
            match segment {
                PathSegment::Text(text) => {
                    let requested = flat_reservation_bytes(1)?;
                    before_reserve(requested)?;
                    flat.try_reserve(1)
                        .map_err(|_| InvalidNodeKey::Allocation { requested })?;
                    flat.push(FlatSegment::Text(text));
                }
                PathSegment::Node(node) => {
                    let NodeKey {
                        package,
                        path,
                        direct_len,
                        ordinal,
                    } = node;
                    let needed = nested_token_count(path.len())?;
                    let requested = flat_reservation_bytes(needed)?;
                    before_reserve(requested)?;
                    flat.try_reserve(needed)
                        .map_err(|_| InvalidNodeKey::Allocation { requested })?;
                    flat.push(FlatSegment::Node {
                        package,
                        direct_len,
                        ordinal,
                        span: needed,
                    });
                    // The reservation above admits this header and every child token.
                    for token in path {
                        flat.push(token);
                    }
                }
            }
        }
        Ok(Self {
            package,
            path: flat,
            direct_len,
            ordinal,
        })
    }

    /// The exact owning package name.
    pub fn package(&self) -> &str {
        &self.package
    }

    /// The path segments in component order. This reconstructs owned segment
    /// views from the flat carrier; each nested node remains flat.
    pub fn path(&self) -> Vec<PathSegment> {
        let mut result = Vec::with_capacity(self.direct_len);
        let mut index = 0;
        while index < self.path.len() {
            match &self.path[index] {
                FlatSegment::Text(text) => {
                    result.push(PathSegment::Text(text.clone()));
                    index += 1;
                }
                FlatSegment::Node {
                    package,
                    direct_len,
                    ordinal,
                    span,
                } => {
                    result.push(PathSegment::Node(NodeKey {
                        package: package.clone(),
                        path: self.path[index + 1..index + *span].to_vec(),
                        direct_len: *direct_len,
                        ordinal: *ordinal,
                    }));
                    index += *span;
                }
            }
        }
        result
    }

    /// The ordinal within the owning package and path.
    pub fn ordinal(&self) -> u64 {
        self.ordinal
    }

    /// Borrow the package, ordinal and direct path without constructing an owned path.
    pub fn view(&self) -> NodeKeyView<'_> {
        NodeKeyView {
            package: &self.package,
            path: &self.path,
            direct_len: self.direct_len,
            ordinal: self.ordinal,
        }
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
        let package = copy_text(&self.package, before_reserve, native_reserve)?;
        let requested = array_bytes::<FlatSegment>(self.path.len()).map_err(retype)?;
        before_reserve(requested).map_err(IdentityCopyError::Policy)?;
        if !native_reserve(requested) {
            return Err(IdentityCopyError::Allocation { requested });
        }
        let mut path = Vec::new();
        path.try_reserve_exact(self.path.len())
            .map_err(|_| IdentityCopyError::Allocation { requested })?;
        for segment in &self.path {
            let copied = match segment {
                FlatSegment::Text(text) => {
                    FlatSegment::Text(copy_text(text, before_reserve, native_reserve)?)
                }
                FlatSegment::Node {
                    package,
                    direct_len,
                    ordinal,
                    span,
                } => FlatSegment::Node {
                    package: copy_text(package, before_reserve, native_reserve)?,
                    direct_len: *direct_len,
                    ordinal: *ordinal,
                    span: *span,
                },
            };
            path.push(copied);
        }
        Ok(Self {
            package,
            path,
            direct_len: self.direct_len,
            ordinal: self.ordinal,
        })
    }

    fn try_clone_legacy_with(
        &self,
        before_reserve: &mut impl FnMut(usize) -> Result<(), InvalidNodeKey>,
    ) -> Result<Self, InvalidNodeKey> {
        let package = copy_text_with(&self.package, before_reserve)?;
        let mut path = Vec::new();
        let requested = flat_reservation_bytes(self.path.len())?;
        before_reserve(requested)?;
        path.try_reserve_exact(self.path.len())
            .map_err(|_| InvalidNodeKey::Allocation { requested })?;
        for segment in &self.path {
            let copied = match segment {
                FlatSegment::Text(text) => FlatSegment::Text(copy_text_with(text, before_reserve)?),
                FlatSegment::Node {
                    package,
                    direct_len,
                    ordinal,
                    span,
                } => FlatSegment::Node {
                    package: copy_text_with(package, before_reserve)?,
                    direct_len: *direct_len,
                    ordinal: *ordinal,
                    span: *span,
                },
            };
            path.push(copied);
        }
        Ok(Self {
            package,
            path,
            direct_len: self.direct_len,
            ordinal: self.ordinal,
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
        write!(f, "{}:{:?}:{}", self.package, self.path, self.ordinal)
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
        let requests = [5, 3 * token_bytes, 5, 6, 5];
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
        assert_eq!(requests_seen, [1, token_bytes]);
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
        let new_requests = [token_bytes, 2 * token_bytes];
        for fail_at in 0..2 {
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
        let clone_requests = [5, 3 * token_bytes, 5, 4];
        for fail_at in 0..4 {
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
        for fail_at in 0..4 {
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
                let mut changed = copy.clone();
                *changed.path.last_mut().expect("nested path has a root") =
                    FlatSegment::Text("root2".into());
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
