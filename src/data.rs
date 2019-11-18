use std::fmt;

pub trait Data: Default + Clone + fmt::Display + fmt::Debug + 'static {
    fn is_empty(&self) -> bool;
}

#[derive(Default, Clone, Debug)]
pub struct NoData {
    __non_exhaustive: (),
}

impl fmt::Display for NoData {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl Data for NoData {
    fn is_empty(&self) -> bool {
        true
    }
}
