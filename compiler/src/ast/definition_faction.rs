use std::fmt::Display;

use crate::ast::*;
use crate::codegen;
use crate::Spanned;

#[derive(Debug)]
pub struct DefinitionFaction {
  pub name: Spanned<String>,
  pub leaders: Spanned<Vec<CardEntry>>,
  pub heroes: Spanned<Vec<CardEntry>>,
  pub units: Spanned<Vec<CardEntry>>,
}

struct CardEntriesChunk<'a>(&'a Vec<CardEntry>, usize);

impl<'a> Iterator for CardEntriesChunk<'a> {
  type Item = &'a [CardEntry];

  fn next(&mut self) -> Option<Self::Item> {
    let items_per_chunk = 140;
    let mut chunk_item_count = 0;
    let old_index = self.1;

    if old_index >= self.0.len() {
      return None;
    }

    for i in old_index..self.0.len() {
      let entry: &CardEntry = &self.0[i];
      let items_count_from_entry = entry.count.inner();

      if chunk_item_count + items_count_from_entry > items_per_chunk {
        self.1 = i;

        return Some(&self.0[old_index..i]);
      }

      chunk_item_count += items_count_from_entry;
    }

    self.1 = self.0.len();

    Some(&self.0[old_index..])
  }
}

impl Display for DefinitionFaction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let faction = &self.name;
    codegen::indent(f, 1)?;
    writeln!(
      f,
      "function generate{faction}RulesHeroCards(): array<GA_CardEntry> {{"
    )?;
    codegen::indent(f, 2)?;
    writeln!(f, "  var output: array<GA_CardEntry>;")?;
    let cards: &Vec<CardEntry> = &self.heroes;
    for card in cards {
      card.fmt(f)?
    }
    codegen::indent(f, 2)?;
    writeln!(f, "  return output;")?;
    codegen::indent(f, 1)?;
    writeln!(f, "}}")?;

    codegen::indent(f, 1)?;
    writeln!(
      f,
      "function generate{faction}RulesLeaderCards(): array<GA_CardEntry> {{"
    )?;
    codegen::indent(f, 2)?;
    writeln!(f, "  var output: array<GA_CardEntry>;")?;
    let cards: &Vec<CardEntry> = &self.leaders;
    for card in cards {
      card.fmt(f)?
    }
    codegen::indent(f, 2)?;
    writeln!(f, "  return output;")?;
    codegen::indent(f, 1)?;
    writeln!(f, "}}")?;


    // the card inserts are sliced into smaller functions to avoid a compiler
    // crash on large functions. 140 is an arbitrary number that we know won't
    // cause a crash.
    let chunker = CardEntriesChunk(&self.units, 0);
    let mut slice_of_cards = Vec::new();

    for slice in chunker {
      dbg!(&slice.len());
      slice_of_cards.push(slice);
    }

    let slice_count = slice_of_cards.len();

    codegen::indent(f, 1)?;
    writeln!(
      f,
      "function generate{faction}RulesUnitCards(): array<GA_CardEntry> {{"
    )?;
    codegen::indent(f, 2)?;
    writeln!(f, "  var output: array<GA_CardEntry>;")?;

    for i in 0..slice_count {
      codegen::indent(f, 3)?;
      writeln!(f, "this.generate{faction}RulesUnitCardsInner{i}(output);")?;
    }
    
    codegen::indent(f, 2)?;
    writeln!(f, "  return output;")?;
    codegen::indent(f, 1)?;
    writeln!(f, "}}")?;

    for (i, &slice) in slice_of_cards.iter().enumerate() {
      codegen::indent(f, 1)?;
      writeln!(
        f,
        "function generate{faction}RulesUnitCardsInner{i}(out output: array<GA_CardEntry>) {{"
      )?;

      for card in slice {
        card.fmt(f)?;
      }

      codegen::indent(f, 1)?;
      writeln!(
        f,
        "}}"
      )?;
    }

    Ok(())
  }
}
