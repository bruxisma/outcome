//! Traits for retryable conversions between types.
//!
//! Much like the rust standard library, the traits in this module provide a
//! way to convert from one type to another type, albeit with a focus on using
//! the [`Outcome`](crate::prelude::Outcome). In `outcome`'s case, only two
//! traits are provided, [`AttemptFrom`] and [`AttemptInto`], which mirror
//! [`TryFrom`] and [`TryInto`] respectively.
//!
//! As a library author, you should always prefer implementing [`AttemptFrom`]
//! over [`AttemptInto`], as [`AttemptFrom`] offers greater flexibility and
//! offers an equivalent [`AttemptInto`] implementation for free, thanks to a
//! blanket implementation in the `outcome` crate.
//!
//! # Generic Implementations
//!
//!  - [`AttemptFrom`]`<U> for T` implies [`AttemptInto`]`<T> for U`
//!
//! [`AttemptFrom`]: crate::convert::AttemptFrom
//! [`AttemptInto`]: crate::convert::AttemptInto
//! [`TryFrom`]: core::convert::TryFrom
//! [`TryInto`]: core::convert::TryInto

use core::convert::Infallible;

use crate::prelude::{Outcome, Success};

/// Outcome's analogue to [`TryFrom`], and the reciprocal of [`TryInto`].
///
/// This is useful when doing a type conversion that *might* trivially succeed,
/// but also might need special error handling. `AttemptFrom` adds the
/// additional ability to inform the caller that they are free to *retry* the
/// conversion event. This is extremely useful in cases where non-[`Copy`]able
/// types are consumed during the conversion, but users
///
///   1. Cannot control the error type returned to give useful diagnostics,
///      either to a caller further up the chain OR to logging information
///   2. May want to try a *different* conversion operation instead (e.g.,
///      trying to treat a string as a filepath *without* having to convert it
///      to the underlying native storage format first)
///
/// # Examples
///
/// ```
/// use outcome::convert::*;
/// use outcome::prelude::*;
///
/// #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
/// enum Version { V1, V2 }
///
/// #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
/// struct EmptyInput;
///
/// #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
/// enum ParseError {
///   InvalidVersion(u8),
/// }
///
/// impl std::fmt::Display for ParseError {
///   fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///     match self {
///       Self::InvalidVersion(v) => write!(f, "Expected a valid version, received: {:x?}", v)
///     }
///   }
/// }
///
/// impl<const N: usize> AttemptFrom<&[u8; N]> for Version {
///   type Mistake = EmptyInput;
///   type Failure = ParseError;
///
///   fn attempt_from (value: &[u8; N]) -> Outcome<Self, Self::Mistake, Self::Failure> {
///     match value.get(0) {
///       None => Mistake(EmptyInput),
///       Some(&1) => Success(Version::V1),
///       Some(&2) => Success(Version::V2),
///       Some(&value) => Failure(ParseError::InvalidVersion(value)),
///     }
///   }
/// }
///
/// let empty = Version::attempt_from(&[]);
/// let v1 = Version::attempt_from(&[1u8]);
/// let v2 = Version::attempt_from(&[2u8]);
/// let v3 = Version::attempt_from(&[3u8]);
/// assert_eq!(empty, Mistake(EmptyInput));
/// assert_eq!(v1, Success(Version::V1));
/// assert_eq!(v2, Success(Version::V2));
/// assert_eq!(v3, Failure(ParseError::InvalidVersion(3)));
/// ```
///
/// [`TryFrom`]: core::convert::TryFrom
/// [`TryInto`]: core::convert::TryInto
/// [`Copy`]: core::marker::Copy
pub trait AttemptFrom<T>: Sized {
  /// The *retryable* error type
  type Mistake;
  /// The *failure* error type
  type Failure;

  /// Performs the conversion
  fn attempt_from(value: T) -> Outcome<Self, Self::Mistake, Self::Failure>;
}

/// An attempted conversion that consumes `self`, which may or may not be
/// expensive. Outcome's analogue to [`TryInto`].
///
/// Library writers should *usually* not implement this trait directly, but
/// should prefer implementing the [`AttemptFrom`] trait, which offers more
/// flexibility and provides an equivalent `AttemptInto` implementation for
/// free, thanks to the blanket implementation provided by the `outcome` crate.
///
/// Unlike [`TryInto`], users are free to return a *retryable* error, which
/// *should* return the data consumed (however this cannot be enforced in
/// practice).
///
/// For more information on this, see the documentation for [`Into`].
///
/// # Examples
///
/// The following example uses the same code from [`AttemptFrom`], but calls
/// `attempt_into` on each object instead.
///
/// ```
/// use outcome::convert::*;
/// use outcome::prelude::*;
///
/// #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
/// enum Version { V1, V2 }
///
/// #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
/// struct EmptyInput;
///
/// #[derive(Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
/// enum ParseError {
///   InvalidVersion(u8),
/// }
///
/// impl std::fmt::Display for ParseError {
///   fn fmt (&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///     match self {
///       Self::InvalidVersion(v) => write!(f, "Expected a valid version, received: {:x?}", v)
///     }
///   }
/// }
///
/// impl<const N: usize> AttemptFrom<&[u8; N]> for Version {
///   type Mistake = EmptyInput;
///   type Failure = ParseError;
///
///   fn attempt_from (value: &[u8; N]) -> Outcome<Self, Self::Mistake, Self::Failure> {
///     match value.get(0) {
///       None => Mistake(EmptyInput),
///       Some(&1) => Success(Version::V1),
///       Some(&2) => Success(Version::V2),
///       Some(&value) => Failure(ParseError::InvalidVersion(value)),
///     }
///   }
/// }
///
/// type ParseOutcome = Outcome<Version, EmptyInput, ParseError>;
///
/// let empty: ParseOutcome = (&[]).attempt_into();
/// let v1: ParseOutcome = (&[1u8]).attempt_into();
/// let v2: ParseOutcome = (&[2u8]).attempt_into();
/// let v3: ParseOutcome = (&[3u8]).attempt_into();
/// assert_eq!(empty, Mistake(EmptyInput));
/// assert_eq!(v1, Success(Version::V1));
/// assert_eq!(v2, Success(Version::V2));
/// assert_eq!(v3, Failure(ParseError::InvalidVersion(3)));
/// ```
///
///
/// [`TryInto`]: core::convert::TryInto
/// [`Mutex`]: core::sync::Mutex
/// [`Into`]: core::convert::Into
pub trait AttemptInto<T>: Sized {
  /// The type returned in the event of a conversion error where the caller
  /// *may* retry the conversion.
  type Mistake;
  /// The type returned in the event of a conversion error where the caller
  /// *may not* retry the conversion.
  type Failure;

  /// Performs the conversion.
  fn attempt_into(self) -> Outcome<T, Self::Mistake, Self::Failure>;
}

/* Blanket Trait Implementations */
impl<T, U> AttemptInto<U> for T
where
  U: AttemptFrom<Self>,
{
  type Mistake = U::Mistake;
  type Failure = U::Failure;

  fn attempt_into(self) -> Outcome<U, Self::Mistake, Self::Failure> {
    U::attempt_from(self)
  }
}

impl<T, U> AttemptFrom<U> for T
where
  U: Into<Self>,
{
  type Mistake = Infallible;
  type Failure = Infallible;

  fn attempt_from(value: U) -> Outcome<Self, Self::Mistake, Self::Failure> {
    Success(value.into())
  }
}

// Reflexive implementation for all [`TryInto`] implementations.
//
// # Notes
//
// If a [`TryInto`] implementation exists because of an [`Into`]
// implementation, the type returned by [`AttemptFrom`] will be an `Outcome<T,
// !, !>`. If the [`unstable` feature](crate#features) is enabled, users can
// then call [`Outcome::into_success`], which will never panic.
//
// ```compile_fail
// # use outcome::prelude::*;
// # use core::convert::Infallible;
// let x: Outcome<u16, Infallible, Infallible> = 1u8.attempt_into();
// assert_eq!(x.into_success(), 1);
// ```
//impl<T, U> AttemptFrom<U> for T
//where
//  U: TryInto<Self>,
//{
//  type Mistake = Infallible;
//  type Failure = <U as TryInto<Self>>::Error;
//
//  fn attempt_from(value: U) -> Outcome<Self, Self::Mistake, Self::Failure> {
//    match value.try_into() {
//      Ok(s) => Success(s),
//      Err(f) => Failure(f),
//    }
//  }
//}
