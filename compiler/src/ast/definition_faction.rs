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

    codegen::indent(f, 1)?;
    writeln!(
      f,
      "function generate{faction}RulesUnitCards(): array<GA_CardEntry> {{"
    )?;
    codegen::indent(f, 2)?;
    writeln!(f, "  var output: array<GA_CardEntry>;")?;
    let cards: &Vec<CardEntry> = &self.units;
    for card in cards {
      card.fmt(f)?
    }
    codegen::indent(f, 2)?;
    writeln!(f, "  return output;")?;
    codegen::indent(f, 1)?;
    writeln!(f, "}}")?;

    Ok(())
  }
}
