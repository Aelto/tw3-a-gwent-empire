use crate::ast::*;
use crate::Spanned;

#[derive(Debug)]
pub enum Definition {
  Variables(Spanned<Vec<DefinitionVariable>>),
  Faction(Spanned<DefinitionFaction>),
}
