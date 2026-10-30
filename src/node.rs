// SPDX-License-Identifier: AGPL-3.0-or-later
//! Checked semantic node identity.
//!
//! A `NodeKey` carries the structural identity of a node in an admitted package.
//! Its owner, path, and ordinal compare componentwise as QSpec FR-322 specifies.
//! The caller checks that the identity joins its admitted package; this leaf
//! checks only the closed shape and never computes a digest.

use alloc::{boxed::Box, string::String, vec::Vec};
use core::{cmp::Ordering, fmt};

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
    Node(Box<NodeKey>),
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

#[derive(Clone, Copy)]
struct NodeView<'a> {
    package: &'a str,
    path: &'a [FlatSegment],
    direct_len: usize,
    ordinal: u64,
}

struct CompareFrame<'a> {
    left: NodeView<'a>,
    right: NodeView<'a>,
    left_index: usize,
    right_index: usize,
    left_remaining: usize,
    right_remaining: usize,
}

impl<'a> CompareFrame<'a> {
    fn new(left: NodeView<'a>, right: NodeView<'a>) -> Self {
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
                    let left_node = NodeView {
                        package: left_package,
                        path: &left_path[frame.left_index + 1..frame.left_index + *left_span],
                        direct_len: *left_len,
                        ordinal: *left_ordinal,
                    };
                    let right_node = NodeView {
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
        let package = package.into();
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
                PathSegment::Text(text) => flat.push(FlatSegment::Text(text)),
                PathSegment::Node(node) => {
                    let NodeKey {
                        package,
                        path,
                        direct_len,
                        ordinal,
                    } = *node;
                    flat.push(FlatSegment::Node {
                        package,
                        direct_len,
                        ordinal,
                        span: path.len() + 1,
                    });
                    flat.extend(path);
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
                    result.push(PathSegment::Node(Box::new(NodeKey {
                        package: package.clone(),
                        path: self.path[index + 1..index + *span].to_vec(),
                        direct_len: *direct_len,
                        ordinal: *ordinal,
                    })));
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

    fn view(&self) -> NodeView<'_> {
        NodeView {
            package: &self.package,
            path: &self.path,
            direct_len: self.direct_len,
            ordinal: self.ordinal,
        }
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
    use alloc::string::ToString;
    use ix_trace_rs::trace;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    use super::*;

    fn key(package: &str, path: Vec<PathSegment>, ordinal: u64) -> NodeKey {
        NodeKey::new(package, path, ordinal).unwrap()
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
                < key("Pkg", vec![PathSegment::Node(Box::new(a.clone()))], 0)
        );
        assert!(
            key(
                "Pkg",
                vec![PathSegment::Node(Box::new(key(
                    "Pkg",
                    vec![PathSegment::Text("A".into())],
                    0,
                )))],
                0,
            ) < key(
                "Pkg",
                vec![PathSegment::Node(Box::new(key(
                    "Pkg",
                    vec![PathSegment::Text("B".into())],
                    0,
                )))],
                0,
            )
        );
        assert!(
            key(
                "Pkg",
                vec![PathSegment::Node(Box::new(key(
                    "Pkg",
                    vec![PathSegment::Text("A".into())],
                    0,
                )))],
                0,
            ) < key(
                "Pkg",
                vec![PathSegment::Node(Box::new(key(
                    "Pkg",
                    vec![PathSegment::Text("A".into())],
                    1,
                )))],
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
        let nested = PathSegment::Node(Box::new(key(
            "Pkg",
            vec![PathSegment::Text("child".into())],
            0,
        )));
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

    #[trace("TC-938", "FR-389-AC-1", "FR-389-AC-7")]
    #[test]
    fn deep_nested_path_clones_compares_hashes_formats_and_drops_on_small_stack() {
        std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(|| {
                let mut node = key("Pkg", vec![PathSegment::Text("root".into())], 0);
                for _ in 0..10_000 {
                    node = key("Pkg", vec![PathSegment::Node(Box::new(node))], 0);
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
                let [PathSegment::Node(parent)] = node.path().try_into().unwrap() else {
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
