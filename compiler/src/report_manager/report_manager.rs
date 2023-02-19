use core::panic;

use ariadne::{Report, Source};

use crate::span_manager::Span;

pub struct ReportManager {
  reports: Vec<(Report, Span)>,
}

impl ReportManager {
  pub fn new() -> Self {
    Self {
      reports: Vec::new(),
    }
  }

  pub fn push(&mut self, report: Report, span: Span) {
    self.reports.push((report, span));
  }

  pub fn push_many(&mut self, reports: Vec<(Report, Span)>) {
    for pair in reports {
      self.push(pair.0, pair.1);
    }
  }

  pub fn flush_reports(&mut self) {
    self.reports.clear();
  }

  pub fn consume(&mut self, content: &str) {
    for (report, _) in &self.reports {
      if let Err(err) = report.print(Source::from(&content)) {
        panic!("{}", err);
      }
    }

    self.flush_reports();
  }
}
