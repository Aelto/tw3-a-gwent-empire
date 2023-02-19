#[derive(Debug)]
pub enum NumberOrVariable {
  Number(u64),
  Variable(String),
}
