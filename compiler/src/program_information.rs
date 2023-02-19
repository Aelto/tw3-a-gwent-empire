use std::collections::HashMap;

use crate::ast::DefinitionVariable;
use crate::Spanned;

pub struct ProgramInformation {
  pub variables: HashMap<String, u64>,
}

impl ProgramInformation {
  pub fn new() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }

  pub fn register_variable(
    &mut self, name: Spanned<String>, value: Spanned<u64>,
  ) -> DefinitionVariable {
    self.variables.insert(name.to_owned(), value.inner());

    DefinitionVariable { name, value }
  }

  pub fn get_variable(&self, key: &str) -> Option<u64> {
    self.variables.get(key).cloned()
  }
}
