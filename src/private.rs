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
