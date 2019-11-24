use std::fmt;

/// Adds nuance to errors.
///
/// Goals:
/// - Easily add context for any `Kind` at each level of the call stack.
/// - Programmatic access to the context.
/// - User-friendly without losing helpful debug information.
pub trait Context: Default + Clone + fmt::Display + fmt::Debug + Send + Sync + 'static {
    /// Returns `true` is the `Context` has no content.
    fn is_empty(&self) -> bool;
}

/// No context needed.
#[derive(Default, Copy, Clone, Debug)]
pub struct NoContext;

impl fmt::Display for NoContext {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Context for NoContext {
    fn is_empty(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use static_assertions::*;

    #[test]
    fn no_context() {
        assert_impl_all!(
            NoContext: Default,
            Copy,
            Clone,
            fmt::Debug,
            fmt::Display,
            Context
        );
    }
}
