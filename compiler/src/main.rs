pub mod ast;
pub mod program_information;
pub mod report_manager;
pub mod span_manager;

use std::fs;
use std::path::Path;

use ariadne::ColorGenerator;
use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;
use ariadne::Source;
use lalrpop_util::lexer::Token;
pub use program_information::ProgramInformation;
pub use report_manager::ReportManager;
pub use span_manager::*;

mod codegen;
mod parser;

pub enum DictionaryError {
  UnknownVariable(Spanned<String>),
  UnknownFaction(Spanned<String>),
}

fn main() {
  for child in fs::read_dir(".").expect("failed to read current directory") {
    let child = child.expect("failed to get current directory child");
    let path = child.path();

    let Some("ruleset") = path.extension().and_then(|s| s.to_str()) else {
      continue;
    };

    let filename = path
      .file_name()
      .unwrap_or_default()
      .to_str()
      .unwrap_or_default();

    println!("compiling {filename}");
    let mut sources_span_manager = SpanManager::new();
    let mut span_maker = sources_span_manager.add_source(filename.to_owned());
    let content =
      fs::read_to_string(&filename).expect(&format!("could not read `{filename}` file"));
    let filename_without_extension = filename.replace(".ruleset", "");
    let mut program_information = ProgramInformation::new(filename_without_extension.to_owned());

    let result =
      parser::ProgramParser::new().parse(&mut program_information, &mut span_maker, &content);

    let output_filename = child.path().with_extension("ws");
    match result {
      Ok(ast) => {
        println!(" - generating {output_filename:#?}");
        let directory_path = format!("modGwentRuleset{}/content/scripts/AGE/rulesets", program_information.name);
        fs::create_dir_all(&directory_path).expect("failed to write resulting mod folders");

        let file_path = Path::new(&directory_path).join(&program_information.name).with_extension("ws");
        fs::write(file_path, ast.to_string()).expect("failed to write output to file");
      }
      Err(err) => print_parsing_error(&content, err, &span_maker.parent, &program_information),
    }
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
    DictionaryError::UnknownFaction(spanned) => {
      handle_unknown_faction_error(content, spanned, span_manager, program_information)
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

fn handle_unknown_faction_error(
  content: &str, spanned: Spanned<String>, span_manager: &SpanManager,
  _program_information: &ProgramInformation,
) {
  let mut colors = ColorGenerator::new();
  let a = colors.next();

  let span = span_manager.get_range(spanned.span);
  let builder = Report::build(ReportKind::Error, (), span.len())
    .with_message("Unknown faction")
    .with_label(
      Label::new(span)
        .with_message(format!("No faction with this name"))
        .with_color(a),
    )
    .with_help(
      "Possible faction names: NorthernKingdom | Skellige | Nilfgaardian | Monster | Scoiatael",
    );

  builder.finish().print(Source::from(&content)).unwrap();
}
