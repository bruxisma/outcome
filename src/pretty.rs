extern crate std;

use std::{error::Error, fmt::Display};

use color_eyre::Section as EyreSection; //, SectionExt as EyreSectionExt};
use eyre::Report;

use crate::prelude::*;

/// Reimplementation of [`color_eyre::Section`]
///
/// This trait reimplements the entire interface of [`color_eyre::Section`], as
/// that trait is "sealed", and thus cannot be implemented directly onto
/// [`Outcome`]. To keep disruption to a mininmum, a blanket implementation is
/// provided for all types that implement [`color_eyre::Section`]. To use
/// [`Outcome`] with this trait, simply `use outcome::prelude::Section` instead
/// of `use color_eyre::Section`.
pub trait Section: crate::private::Sealed {
  /// See [`color_eyre::Section::Return`] for more information
  type Return;

  /// See [`color_eyre::Section::with_suggestion`] for more info
  fn with_suggestion<D, F>(self, suggestion: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D;

  /// See [`color_eyre::Section::with_warning`] for more info
  fn with_warning<D, F>(self, warning: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D;

  /// See [`color_eyre::Section::with_section`] for more info
  fn with_section<D, F>(self, section: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D;

  /// See [`color_eyre::Section::with_error`] for more info
  fn with_error<E, F>(self, error: F) -> Self::Return
  where
    E: Error + Send + Sync + 'static,
    F: FnOnce() -> E;

  /// See [`color_eyre::Section::with_note`] for more info
  fn with_note<D, F>(self, note: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D;

  /// See [`color_eyre::Section::suggestion`] for more info
  fn suggestion<D>(self, suggestion: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static;

  /// See [`color_eyre::Section::warning`] for more info
  fn warning<D>(self, warning: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static;

  /// See [`color_eyre::Section::section`] for more info
  fn section<D>(self, section: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static;

  /// See [`color_eyre::Section::error`] for more info
  fn error<E>(self, error: E) -> Self::Return
  where
    E: Error + Send + Sync + 'static;

  /// See [`color_eyre::Section::note`] for more info
  fn note<D>(self, note: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static;
}

impl<T> Section for T
where
  T: EyreSection,
{
  type Return = <Self as EyreSection>::Return;

  fn with_suggestion<D, F>(self, suggestion: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    <Self as EyreSection>::with_suggestion(self, suggestion)
  }

  fn with_warning<D, F>(self, warning: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    <Self as EyreSection>::with_warning(self, warning)
  }

  fn with_section<D, F>(self, section: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    <Self as EyreSection>::with_section(self, section)
  }

  fn with_error<E, F>(self, error: F) -> Self::Return
  where
    E: Error + Send + Sync + 'static,
    F: FnOnce() -> E,
  {
    <Self as EyreSection>::with_error(self, error)
  }

  fn with_note<D, F>(self, note: F) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    F: FnOnce() -> D,
  {
    <Self as EyreSection>::with_note(self, note)
  }

  fn suggestion<D>(self, suggestion: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    <Self as EyreSection>::suggestion(self, suggestion)
  }

  fn warning<D>(self, warning: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    <Self as EyreSection>::warning(self, warning)
  }

  fn section<D>(self, section: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    <Self as EyreSection>::section(self, section)
  }

  fn error<E>(self, error: E) -> Self::Return
  where
    E: Error + Send + Sync + 'static,
  {
    <Self as EyreSection>::error(self, error)
  }

  fn note<D>(self, note: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    <Self as EyreSection>::note(self, note)
  }
}

impl<S, M, F> Section for Outcome<S, M, F>
where
  F: Into<Report>,
{
  type Return = Outcome<S, M, Report>;

  fn with_suggestion<D, C>(self, suggestion: C) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    C: FnOnce() -> D,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::suggestion(report, suggestion()))
  }

  fn with_warning<D, C>(self, warning: C) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    C: FnOnce() -> D,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::warning(report, warning()))
  }

  fn with_section<D, C>(self, section: C) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    C: FnOnce() -> D,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::section(report, section()))
  }

  fn with_error<E, C>(self, error: C) -> Self::Return
  where
    E: Error + Send + Sync + 'static,
    C: FnOnce() -> E,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::error(report, error()))
  }

  fn with_note<D, C>(self, note: C) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
    C: FnOnce() -> D,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::note(report, note()))
  }

  fn suggestion<D>(self, suggestion: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::suggestion(report, suggestion))
  }

  fn warning<D>(self, warning: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::warning(report, warning))
  }

  fn section<D>(self, section: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::section(report, section))
  }

  fn error<E>(self, error: E) -> Self::Return
  where
    E: Error + Send + Sync + 'static,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::error(report, error))
  }

  fn note<D>(self, note: D) -> Self::Return
  where
    D: Display + Send + Sync + 'static,
  {
    self
      .map_failure(Into::into)
      .map_failure(|report| EyreSection::note(report, note))
  }
}
