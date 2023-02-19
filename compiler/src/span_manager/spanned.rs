use std::ops::Deref;

use super::*;

#[derive(Debug)]
pub struct Spanned<T> {
  pub span: Span,
  value: T,
}

impl<T> Spanned<T> {
  pub fn new(value: T, span: Span) -> Self {
    Self { span, value }
  }
}

impl<T> Deref for Spanned<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

impl<T> Spanned<T>
where
  T: Copy,
{
  pub fn inner(&self) -> T {
    self.value
  }
}
