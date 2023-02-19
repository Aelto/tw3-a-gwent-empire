use crate::Spanned;

#[derive(Debug)]
pub struct DefinitionVariable {
  pub name: Spanned<String>,
  pub value: Spanned<u64>,
}
