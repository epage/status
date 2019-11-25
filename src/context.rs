use std::fmt;

/// Adds nuance to errors.
///
/// Goals:
/// - Easily add context for any `Kind` at each level of the call stack.
/// - Programmatic access to the context.
/// - User-friendly without losing helpful debug information.
pub trait Context: Default + Clone + fmt::Display + fmt::Debug + Send + Sync + 'static {
    /// Replace fields in `self` with those populated in `replacements`.
    fn update(self, replacements: Self) -> Self;

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
    fn update(self, _replacements: Self) -> Self {
        self
    }

    fn is_empty(&self) -> bool {
        true
    }
}

/// Adhoc [`Context`].
///
/// Unlike most [`Context`]s, this is meant to be opaque and not programmatically specify the status.
/// It is only good for displaying the data to the user when prototyping before one transitions to more formal [`Context`]s.
///
/// Note: This is the default [`Context`] for [`Status`].
#[derive(Default, Clone, Debug)]
pub struct AdhocContext {
    data: indexmap::IndexMap<&'static str, Box<dyn AdhocValue>>,
}

impl AdhocContext {
    /// Create an empty [`Context`].
    pub fn new() -> Self {
        Default::default()
    }

    /// Add `Display`-only context for a [`Status`]
    ///
    /// If an equivalent key already exists: the key remains and retains in its place in
    /// the order, its corresponding value is updated with value.
    ///
    /// If no equivalent key existed: the new key-value pair is inserted, last in order.
    ///
    /// # Example
    ///
    /// ```rust
    /// let c = status::AdhocContext::new().insert("Expected value", 10);
    /// println!("{}", c);
    /// ```
    pub fn insert<V>(mut self, key: &'static str, value: V) -> Self
    where
        V: AdhocValue + Clone,
    {
        self.data.insert(key, Box::new(value));
        self
    }
}

impl fmt::Display for AdhocContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in self.data.iter() {
            writeln!(f, "{}: {}", k, v)?;
        }
        Ok(())
    }
}

impl Context for AdhocContext {
    fn update(mut self, replacements: Self) -> Self {
        self.data.extend(replacements.data);
        self
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Trait alias for values in a [`AdhocContext`]
pub trait AdhocValue: fmt::Display + fmt::Debug + Send + Sync + 'static {
    /// Clone the value
    fn clone_box(&self) -> Box<dyn AdhocValue>;
}

impl<V> AdhocValue for V
where
    V: Clone + fmt::Display + fmt::Debug + Send + Sync + 'static,
{
    fn clone_box(&self) -> Box<dyn AdhocValue> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn AdhocValue> {
    fn clone(&self) -> Box<dyn AdhocValue> {
        self.clone_box()
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

    #[test]
    fn adhoc_context() {
        assert_impl_all!(NoContext: Default, Clone, fmt::Debug, fmt::Display, Context);
    }
}
