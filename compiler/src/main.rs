pub mod ast;
pub mod program_information;
pub mod report_manager;
pub mod span_manager;

use std::fs;

use ariadne::ColorGenerator;
use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;
use ariadne::Source;
use lalrpop_util::lexer::Token;
pub use program_information::ProgramInformation;
pub use report_manager::ReportManager;
pub use span_manager::*;

mod parser;

pub enum DictionaryError {
  UnknownVariable(Spanned<String>),
}

fn main() {
  let mut sources_span_manager = SpanManager::new();
  let mut span_maker = sources_span_manager.add_source("dictionary".to_owned());
  let content = fs::read_to_string("dictionary").expect("could not read `dictionary` file");
  let mut program_information = ProgramInformation::new();

  let result =
    parser::ProgramParser::new().parse(&mut program_information, &mut span_maker, &content);

  match result {
    Ok(ast) => {
      dbg!(ast);
    }
    Err(err) => print_parsing_error(&content, err, &span_maker.parent, &program_information),
  }
}

fn print_parsing_error(
  content: &str, error: lalrpop_util::ParseError<usize, Token, DictionaryError>,
  span_manager: &SpanManager, program_information: &ProgramInformation,
) {
  match error {
    lalrpop_util::ParseError::InvalidToken { location } => {
      let mut colors = ColorGenerator::new();
      let a = colors.next();

      Report::build(ReportKind::Error, (), location)
        .with_message("Invalid token in dictionary file")
        .with_label(
          Label::new(location..location.checked_add(1).unwrap_or(location))
            .with_message("The invalid token")
            .with_color(a),
        )
        .finish()
        .print(Source::from(&content))
        .unwrap();
    }
    lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
      let mut colors = ColorGenerator::new();
      let a = colors.next();

      let mut builder = Report::build(ReportKind::Error, (), token.0)
        .with_message("Unrecognized token in dictionary file")
        .with_label(
          Label::new(token.0..token.2)
            .with_message(format!("Expected one of these {}", expected.join(" | ")))
            .with_color(a),
        );

      let error_slice = &content[token.0..token.2];
      if "points".contains(error_slice) {
        builder = builder.with_help("Did you mean to use `points`?");
      }

      if "difficulty".contains(error_slice) {
        builder = builder.with_help("Did you mean to use `difficulty`?");
      }

      builder.finish().print(Source::from(&content)).unwrap();
    }

    lalrpop_util::ParseError::User { error } => {
      handle_user_error(content, error, &span_manager, program_information)
    }
    _ => {}
  }
}

fn handle_user_error(
  content: &str, error: DictionaryError, span_manager: &SpanManager,
  program_information: &ProgramInformation,
) {
  match error {
    DictionaryError::UnknownVariable(spanned) => {
      handle_unknown_variable_error(content, spanned, span_manager, program_information)
    }
  }
}

fn handle_unknown_variable_error(
  content: &str, spanned: Spanned<String>, span_manager: &SpanManager,
  program_information: &ProgramInformation,
) {
  let mut colors = ColorGenerator::new();
  let a = colors.next();

  let span = span_manager.get_range(spanned.span);
  let mut builder = Report::build(ReportKind::Error, (), span.len())
    .with_message("Unknown variable")
    .with_label(
      Label::new(span)
        .with_message(format!("Cannot find value nor variable with this name"))
        .with_color(a),
    );

  let error_slice = &content[span_manager.get_range(spanned.span)];

  for key in program_information.variables.keys() {
    if key.contains(error_slice) || error_slice.contains(key) {
      builder = builder.with_help(&format!("variable with similar name: `{key}`"));
    }
  }
  if "points".contains(error_slice) {}

  if "difficulty".contains(error_slice) {
    builder = builder.with_help("Did you mean to use `difficulty`?");
  }

  builder.finish().print(Source::from(&content)).unwrap();
}
