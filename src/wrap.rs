#[allow(unused_macros)]
macro_rules! r#trait {
  ($type:ty) => {
    /// This trait is meant to be the `outcome` analogue of *both*
    /// [`miette::WrapErr`] and [`eyre::WrapErr`]. Therefore, any type that
    /// implements `WrapErr` for either of these libraries will automatically work
    /// with `WrapFailure`.
    ///
    /// This trait is sealed and cannot be implemented for types outside of this
    /// `outcome`.
    ///
    /// [`Outcome`]: crate::prelude::Outcome
    pub trait WrapFailure: crate::private::Sealed {
      /// The expected return type for an `impl`.
      ///
      /// This will always be the same enumeration type, but with a [`Report`]
      /// in the error or failure position.
      type Return;

      /// Wrap the failure value with a new adhoc error that is evaluated lazily
      /// only once an error does occur.
      fn wrap_failure_with<D, F>(self, message: F) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D;

      /// Wrap the failure value with a new adhoc error.
      fn wrap_failure<D>(self, message: D) -> Self::Return
      where
        D: Display + Send + Sync + 'static;

      /// Compatibility re-export of [`wrap_failure_with`] for interop with
      /// [`anyhow`] and [`eyre`].
      ///
      /// [`wrap_failure_with`]: WrapFailure::wrap_failure_with
      /// [`anyhow`]: https://crates.io/crates/anyhow
      /// [`eyre`]: https://crates.io/crates/eyre
      fn with_context<D, F>(self, message: F) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D;
      /// Compatibility re-export of [`wrap_failure`] for interop with
      /// [`anyhow`] and [`eyre`].
      ///
      /// [`wrap_failure`]: WrapFailure::wrap_failure
      /// [`anyhow`]: https://crates.io/crates/anyhow
      /// [`eyre`]: https://crates.io/crates/eyre
      fn context<D>(self, message: D) -> Self::Return
      where
        D: Display + Send + Sync + 'static;
    }
  };
}

#[allow(unused_macros)]
macro_rules! r#impl {
  ($type:ident) => {
    impl<S, M, E> WrapFailure for Outcome<S, M, E>
    where
      E: $type + Send + Sync + 'static,
    {
      type Return = Outcome<S, M, Report>;

      #[track_caller]
      #[inline]
      fn wrap_failure_with<D, F>(self, message: F) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
      {
        self.map_failure(|f| Report::new(f).wrap_err(message()))
      }

      #[track_caller]
      #[inline]
      fn wrap_failure<D>(self, message: D) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
      {
        self.map_failure(|f| Report::new(f).wrap_err(message))
      }

      #[track_caller]
      #[inline]
      fn with_context<D, F>(self, message: F) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
      {
        self.wrap_failure_with(message)
      }

      #[track_caller]
      #[inline]
      fn context<D>(self, message: D) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
      {
        self.wrap_failure(message)
      }
    }

    impl<M, E> WrapFailure for Aberration<M, E>
    where
      E: $type + Send + Sync + 'static,
    {
      type Return = Aberration<M, Report>;

      #[track_caller]
      #[inline]
      fn wrap_failure_with<D, F>(self, message: F) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
      {
        self.map_failure(|f| Report::new(f).wrap_err(message()))
      }

      #[track_caller]
      #[inline]
      fn wrap_failure<D>(self, message: D) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
      {
        self.map_failure(|f| Report::new(f).wrap_err(message))
      }

      #[track_caller]
      #[inline]
      fn with_context<D, F>(self, message: F) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
      {
        self.wrap_failure_with(message)
      }

      #[track_caller]
      #[inline]
      fn context<D>(self, message: D) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
      {
        self.wrap_failure(message)
      }
    }
  };
}


#[allow(unused_macros)]
macro_rules! r#use {
  (miette) => { "use outcome::diagnostic::{WrapFailure, Result, Report};" };
  (eyre) => { "use outcome::report::{WrapFailure, Result, Report};" };
}

#[allow(unused_macros)]
macro_rules! r#result {
  ($module:ident) => {
    /// Implementation of [`WrapFailure`] for `Result<T, E>` for any
    /// implementations of [`WrapErr`].
    ///
    /// ```
    #[doc = crate::wrap::r#use!($module)]
    ///
    /// fn execute() -> Result<()> {
    ///   # Err(Report::msg("error here"))?;
    ///   # const IGNORE: &str = stringify! {
    ///   ...
    ///   # };
    ///   # unreachable!()
    /// }
    ///
    /// pub fn invoke() -> Result<Vec<u8>> {
    ///   execute().wrap_failure("Failed to execute correctly")?;
    ///   Ok(vec![])
    /// }
    /// ```
    #[doc = concat!("[`WrapErr`]:", stringify!($module), "::WrapErr")]
    impl<T, E> WrapFailure for Result<T, E>
    where
      Self: $module::WrapErr<T, E>,
    {
      type Return = Result<T, $module::Report>;

      #[track_caller]
      #[inline]
      fn wrap_failure_with<D, F>(self, message: F) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
      {
        $module::WrapErr::wrap_err_with(self, message)
      }

      #[track_caller]
      #[inline]
      fn wrap_failure<D>(self, message: D) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
      {
        $module::WrapErr::wrap_err(self, message)
      }

      #[track_caller]
      #[inline]
      fn with_context<D, F>(self, message: F) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
        F: FnOnce() -> D,
      {
        $module::WrapErr::with_context(self, message)
      }

      #[track_caller]
      #[inline]
      fn context<D>(self, message: D) -> Self::Return
      where
        D: Display + Send + Sync + 'static,
      {
        $module::WrapErr::context(self, message)
      }
    }
  };
}

#[allow(clippy::redundant_pub_crate)]
#[allow(unused_imports)]
pub(crate) use {r#impl, r#result, r#trait, r#use};
