use crate::ast::*;
use crate::Spanned;

#[derive(Debug)]
pub struct DefinitionFaction {
  pub name: Spanned<String>,
  pub leaders: Spanned<Vec<CardEntry>>,
  pub heroes: Spanned<Vec<CardEntry>>,
  pub units: Spanned<Vec<CardEntry>>,
}
