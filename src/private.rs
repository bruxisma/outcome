use core::fmt::Debug;

/* Much like the internal `unwrap_failed` function found in core::result, this
 * function helps reduce method code size. Given that we have several types
 * that all have `unwrap(_.+)?` names, this helps immensely for generated code.
 */
#[inline(never)]
#[track_caller]
#[cold]
pub fn panic(method: &str, variant: &str, error: &dyn Debug) -> ! {
  panic!("Called `{}` on a `{}` value: {:?}", method, variant, error);
}

pub trait Sealed {}

#[cfg(feature = "pretty-report")]
impl<T> Sealed for T where T: color_eyre::Section {}

#[cfg(all(feature = "report", not(feature = "pretty-report")))]
impl Sealed for eyre::Report {}

#[cfg(all(feature = "report", not(feature = "pretty-report")))]
impl<T, E> Sealed for Result<T, E> where E: Into<eyre::Report> {}

impl<S, M, F> Sealed for crate::outcome::Outcome<S, M, F> {}
impl<M, F> Sealed for crate::aberration::Aberration<M, F> {}
