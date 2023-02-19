use crate::ast::*;
use crate::Spanned;

#[derive(Debug)]
pub struct Program {
  pub definitions: Vec<Spanned<Definition>>,
}
