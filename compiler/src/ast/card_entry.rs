use std::fmt::Display;

use crate::ast::*;
use crate::codegen;
use crate::Spanned;

#[derive(Debug)]
pub struct CardEntry {
  pub index: Spanned<u64>,
  pub count: Spanned<u64>,
  pub points: Option<Spanned<u64>>,
  pub difficulty: Option<Spanned<u64>>,
}

impl Display for CardEntry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let index = &self.index;
    let count = self.count.inner();
    let points = &self
      .points
      .as_ref()
      .and_then(|spanned| Some(spanned.inner()))
      .unwrap_or_else(|| 1);
    let difficulty = &self
      .difficulty
      .as_ref()
      .and_then(|spanned| Some(spanned.inner()))
      .unwrap_or_else(|| 0);

    for i in 0..count {
      codegen::indent(f, 3)?;
      writeln!(f, "output.PushBack(GA_CardEntry({index}, 1, 1, 1, GA_DeckPoints({points}), GA_MatchDifficulty({difficulty}))); // {i}")?;
    }

    Ok(())
  }
}
