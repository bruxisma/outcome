use crate::Outcome;

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
/// should prefer implementing the `AttemptFrom` trait, which offers more
/// flexibility and provides an equivalent `AttemptInto` implementation for
/// free, thanks to the blanket implementation provided by the `outcome` crate.
///
/// Unlike [`TryInto`], users are free to return a *retryable* error, which
/// *should* return the data consumed.
///
/// ```ignore
/// # use outcome::*;
/// let y = -1i8;
/// let x: Outcome<u128, i8, Box<dyn std::error::Error>> = y.attempt_into();
/// match x {
///   // y's original value, which we can retrieve if desired.
///   Mistake(y) => { return y; },
///   _ => { /* ... */ }
/// }
/// ```
///
/// For more information on this, see the documentation for [`Into`].
///
/// [`TryInto`]: core::convert::TryInto
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
