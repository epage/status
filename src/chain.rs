type StdError = dyn std::error::Error + 'static;

#[derive(Debug)]
pub struct Chain<'a> {
    next: Option<&'a StdError>,
}

impl<'a> Chain<'a> {
    pub(crate) fn new(next: Option<&'a StdError>) -> Self {
        Self { next }
    }
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a StdError;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take()?;
        self.next = next.source();
        Some(next)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use static_assertions::*;

    #[test]
    fn chain() {
        assert_impl_all!(Chain: std::fmt::Debug);
    }
}
