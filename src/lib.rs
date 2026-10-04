//! The exact-value kernel: checked identity, typestate, outcome and refusal, provenance, and bound value types, with no dependency on any other crate in the quire ecosystem.

#![warn(missing_docs)]

/// Placeholder entry point.
pub fn hello() -> &'static str {
    "hello from quire_exact"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hello_returns_greeting() {
        assert!(hello().contains("quire_exact"));
    }
}
