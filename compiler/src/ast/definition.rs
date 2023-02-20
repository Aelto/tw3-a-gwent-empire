use std::fmt::Display;

use crate::ast::*;
use crate::Spanned;

#[derive(Debug)]
pub enum Definition {
  Variables(Spanned<Vec<DefinitionVariable>>),
  Faction(Spanned<DefinitionFaction>),
}

impl Display for Definition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      // variables don't emit anything
      Definition::Variables(_) => {}

      Definition::Faction(faction) => {
        faction.fmt(f)?;
      }
    };

    Ok(())
  }
}
