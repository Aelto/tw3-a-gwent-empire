use crate::ast::*;
use crate::Spanned;

#[derive(Debug)]
pub struct CardEntry {
  pub index: Spanned<NumberOrVariable>,
  pub count: Spanned<NumberOrVariable>,
  pub points: Option<Spanned<NumberOrVariable>>,
  pub difficulty: Option<Spanned<NumberOrVariable>>,
}
