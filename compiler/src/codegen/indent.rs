pub fn indent(f: &mut std::fmt::Formatter<'_>, count: usize) -> std::fmt::Result {
  let indent = "  ".repeat(count);

  write!(f, "{indent}")
}
