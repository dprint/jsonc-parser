use std::rc::Rc;

/// A string that cannot be changed.
#[derive(Clone, Debug, PartialEq)]
pub struct ImmutableString {
    inner: Rc<String>,
}

impl ImmutableString {
    pub fn as_ref(&self) -> &str {
        &self.inner
    }

    pub(super) fn new(text: String) -> ImmutableString {
        ImmutableString {
            inner: Rc::new(text),
        }
    }

    #[cfg(test)]
    pub(super) fn from(text: &str) -> ImmutableString {
        ImmutableString {
            inner: Rc::new(String::from(text)),
        }
    }
}
