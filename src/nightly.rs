#[cfg(feature = "std")]
extern crate std;
use core::{
  convert::Infallible,
  ops::{ControlFlow, FromResidual, Try},
};
#[cfg(feature = "std")]
use std::{
  eprintln,
  fmt::Debug,
  process::{ExitCode, Termination},
};

use crate::prelude::*;

/* feature(never_type) */
impl<S, M, F> Outcome<S, M, F> {
  /// **`TODO`**: write documentation
  pub fn escalate_with<C, T>(self, closure: C) -> Outcome<!, M, F>
  where
    T: Into<M>,
    C: FnOnce(S) -> T,
  {
    match self {
      Success(s) => Mistake(closure(s).into()),
      Mistake(m) => Mistake(m),
      Failure(f) => Failure(f),
    }
  }
}

impl<S: Into<!>, M: Into<F>, F> Outcome<S, M, F> {
  /// Escalates an [`Outcome`] from a [`Mistake`] to a [`Failure`]
  pub fn escalate_mistake(self) -> Outcome<!, !, F> {
    match self {
      Success(s) => s.into(),
      Mistake(m) => Failure(m.into()),
      Failure(f) => Failure(f),
    }
  }
}

impl<S: Into<!>, M, F> Outcome<S, M, F> {
  /// Escalates an [`Outcome`] from a [`Mistake`] to a [`Failure`] using the
  /// given closure.
  ///
  pub fn escalate_mistake_with<C, G>(self, closure: C) -> Outcome<!, !, F>
  where
    G: Into<F>,
    C: FnOnce(M) -> G,
  {
    match self {
      Success(s) => s.into(),
      Mistake(m) => Failure(closure(m).into()),
      Failure(f) => Failure(f),
    }
  }
}

impl<S, M: Into<!>, F: Into<!>> Outcome<S, M, F> {
  /// Returns the contained [`Success`] value, but never panics.
  ///
  /// Unlike [`unwrap`], this method is known to never panic on the outcome
  /// types it is implemented for. Therefore, it can be used instead of
  /// `unwrap` as a maintainability safeguard that will fail to compile if the
  /// mistake or failure type of the `Outcome` is later changed to mistake or
  /// failure that can actually occur.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![feature(never_type)]
  /// # use outcome::prelude::*;
  /// fn only_success() -> Outcome<String, !, !> {
  ///   Success("This is fine 🐶☕🔥".into())
  /// }
  ///
  /// let s: String = only_success().into_success();
  /// assert!(s.contains("This is fine"));
  /// ```
  ///
  /// [`unwrap`]: crate::prelude::Outcome::unwrap
  pub fn into_success(self) -> S {
    match self {
      Success(s) => s,
      Mistake(m) => m.into(),
      Failure(f) => f.into(),
    }
  }
}

impl<S: Into<!>, M, F: Into<!>> Outcome<S, M, F> {
  /// Returns the contained [`Mistake`] value, but never panics.
  ///
  /// Unlike [`unwrap_mistake`], this method is known to never panic on the
  /// outcome types it is implemented for. Therefore it can be used instead of
  /// `unwrap_mistake` as a maintainibility safeguard that will fail to compile
  /// if the success or failure type of the `Outcome` is later changed to a
  /// success or failure that can actually occur.
  ///
  /// # Examples
  ///
  /// ```
  /// # #![feature(never_type)]
  /// # use outcome::prelude::*;
  /// fn only_mistake() -> Outcome<!, String, !> {
  ///   Mistake("Try another! 🍾🔫🤠".into())
  /// }
  ///
  /// let s: String = only_mistake().into_mistake();
  /// assert!(s.contains("Try another!"));
  /// ```
  ///
  /// [`unwrap_mistake`]: crate::prelude::Outcome::unwrap_mistake
  pub fn into_mistake(self) -> M {
    match self {
      Success(s) => s.into(),
      Mistake(m) => m,
      Failure(f) => f.into(),
    }
  }
}

impl<S: Into<!>, M: Into<!>, F> Outcome<S, M, F> {
  /// Returns the contained [`Failure`] value, but never panics.
  ///
  /// Unlike [`unwrap_failure`], this method is known to never panic on the
  /// outcome types it is implemented for. Therefore, it can be used instead of
  /// `unwrap_failure` as a maintainibility safeguard that will fail to compile
  /// if the success or mistake type of the `Outcome` is later changed to a
  /// success or mistake that can actually occur.
  ///
  /// ```
  /// # #![feature(never_type)]
  /// # use outcome::prelude::*;
  /// fn only_failure() -> Outcome<!, !, String> {
  ///   Failure("Catarina! 👦🤚🪑👧".into())
  /// }
  ///
  /// let s: String = only_failure().into_failure();
  /// assert!(s.contains("Catarina!"));
  /// ```
  ///
  /// [`unwrap_failure`]: crate::prelude::Outcome::unwrap_failure
  pub fn into_failure(self) -> F {
    match self {
      Success(s) => s.into(),
      Mistake(m) => m.into(),
      Failure(f) => f,
    }
  }
}

#[cfg(feature = "std")]
impl<M: Debug, F: Debug> Termination for Outcome<!, M, F> {
  fn report(self) -> ExitCode {
    #[allow(clippy::print_stderr)]
    match self {
      Mistake(m) => eprintln!("Mistake: {:?}", m),
      Failure(f) => eprintln!("Failure: {:?}", f),
    };
    ExitCode::FAILURE
  }
}

/* feature(try_trait_v2) */
impl<S, M, F> Try for Outcome<S, M, F> {
  type Output = Concern<S, M>;
  type Residual = Outcome<Infallible, Infallible, F>;

  #[inline]
  fn from_output(output: Self::Output) -> Self {
    match output {
      Concern::Success(s) => Success(s),
      Concern::Mistake(m) => Mistake(m),
    }
  }

  #[inline]
  fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
    match self {
      Success(s) => ControlFlow::Continue(Concern::Success(s)),
      Mistake(m) => ControlFlow::Continue(Concern::Mistake(m)),
      Failure(f) => ControlFlow::Break(Failure(f)),
    }
  }
}

impl<M, F> Try for Aberration<M, F> {
  type Output = M;
  type Residual = Result<Infallible, F>;

  #[inline]
  fn from_output(output: Self::Output) -> Self {
    Self::Mistake(output)
  }

  #[inline]
  fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
    match self {
      Self::Mistake(m) => ControlFlow::Continue(m),
      Self::Failure(f) => ControlFlow::Break(Err(f)),
    }
  }
}

impl<S, M, F, G: From<F>> FromResidual<Outcome<Infallible, Infallible, F>>
  for Outcome<S, M, G>
{
  #[inline]
  fn from_residual(residual: Outcome<Infallible, Infallible, F>) -> Self {
    match residual {
      Failure(f) => Failure(From::from(f)),
    }
  }
}

impl<S, M, F, N: From<M>, G: From<F>> FromResidual<Aberration<M, F>>
  for Outcome<S, N, G>
{
  #[inline]
  fn from_residual(residual: Aberration<M, F>) -> Self {
    match residual {
      Aberration::Mistake(m) => Mistake(From::from(m)),
      Aberration::Failure(f) => Failure(From::from(f)),
    }
  }
}

impl<T, F, E: From<F>> FromResidual<Outcome<Infallible, Infallible, F>>
  for Result<T, E>
{
  #[inline]
  fn from_residual(residual: Outcome<Infallible, Infallible, F>) -> Self {
    match residual {
      Failure(f) => Err(From::from(f)),
    }
  }
}

impl<S, M, E, F: From<E>> FromResidual<Result<Infallible, E>>
  for Outcome<S, M, F>
{
  #[inline]
  fn from_residual(residual: Result<Infallible, E>) -> Self {
    match residual {
      Err(e) => Failure(From::from(e)),
    }
  }
}

impl<M, E, F: From<E>> FromResidual<Result<Infallible, E>>
  for Aberration<M, F>
{
  #[inline]
  fn from_residual(residual: Result<Infallible, E>) -> Self {
    match residual {
      Err(e) => Self::Failure(From::from(e)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  mod try_trait {
    use super::*;

    #[test]
    fn aberration() -> Result<(), &'static str> {
      let aberration: Aberration<u32, &str> = Aberration::Mistake(0u32);
      let value = aberration?;
      assert_eq!(value, 0u32);
      Ok(())
    }

    #[test]
    fn outcome() -> Result<(), &'static str> {
      let outcome: Outcome<f32, u32, &str> = Mistake(0u32);
      let concern = outcome?;
      assert_eq!(concern, Concern::Mistake(0u32));
      Ok(())
    }
  }

  #[cfg(feature = "std")]
  mod termination {
    use super::*;

    #[test]
    fn aberration() -> Outcome<(), (), &'static str> {
      let aberration: Aberration<u32, &str> = Aberration::Mistake(0u32);
      let value = aberration?;
      assert_eq!(value, 0u32);
      Success(())
    }

    #[test]
    fn outcome() -> Outcome<(), (), &'static str> {
      let outcome: Outcome<f32, u32, &str> = Mistake(0u32);
      let concern = outcome?;
      assert_eq!(concern, Concern::Mistake(0u32));
      Success(())
    }

    #[test]
    fn result() -> Outcome<(), (), &'static str> {
      let result: Result<u32, &str> = Ok(0u32);
      let value = result?;
      assert_eq!(value, 0u32);
      Success(())
    }
  }
}
