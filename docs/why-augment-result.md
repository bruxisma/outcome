# Why Augment `Result<T, E>`?

[`Outcome`] is *not* intended to fully replace [`Result`], especially at
the API boundary (i.e., the API used by clients) when there is a clear
success or failure state that can be transferred to users. Instead, it
provides the ability to quickly expand the surface area of consumed APIs
with finer grained control over errors so that library writers can write
*correct* behavior and then return at a later time to compose results,
expand error definitions, or to represent different error severities.

As an example, the section [making unhandled errors unrepresentable][1] in
the post *Error Handling in a Correctness-Critical Rust Project*, the
author states:

> this led me to go for what felt like the nuclear solution, but after
> seeing how many bugs it immediately rooted out by simply refactoring the
> codebase, I’m convinced that this is the only way to do error handling in
> systems where we have multiple error handling concerns in Rust today.

The solution, as they explain in the next paragraph, is

> make the global `Error` enum specifically only hold errors that should
> cause the overall system to halt - reserved for situations that require
> human intervention. Keep errors which relate to separate concerns in
> totally separate error types. By keeping errors that must be handled
> separately in their own types, we reduce the chance that the try `?`
> operator will accidentally push a local concern into a caller that can’t
> deal with it.

As the author of this post later shows, the `sled::Tree::compare_and_swap`
function returns a `Result<Result<(), CompareAndSwapError>, sled::Error>`.
They state this looks "*way less cute*", but will

> improve chances that users will properly handle their compare and
> swap-related errors properly\[sic]
>
> ```rust
> # const IGNORE: &str = stringify! {
> // we can actually use try `?` now
> let cas_result = sled.compare_and_swap(
>   "dogs",
>   "pickles",
>   "catfood"
> )?;
>
> if let Err(cas_error) = cas_result {
>     // handle expected issue
> }
> # };
> ```

The issue with this return type is that there is *technically nothing* to
stop a user from using what the creator of the `outcome` crate calls the
WTF operator (`??`) to ignore these intermediate errors.

```rust
# const IGNORE: &str = stringify! {
let cas = sled.compare_and_swap("dogs", "pickles", "catfood")??;
# };
```

It would be hard to *forbid* this kind of usage with tools like clippy due
to libraries such as [nom][2] relying on nested results and expecting
moderately complex pattern matching to extract relevant information.

Luckily, it *is* easier to prevent this issue in the first place if:

 - An explicit call to extract an inner `Result<T, E>`-like type must be made
 - The call of an easily greppable/searchable function before using the
    "WTF" (`??`) operator is permitted.
 - The [`Try`] trait returns a type that *must* be decomposed explicitly
    and *does not support* the try `?` operator itself.

Thanks to [clippy](https://github.com/rust-lang/rust-clippy)'s
`disallowed_method` lint, users can rely on the first two options until
[`Try`] has been stabilized.

`outcome` provides this in the form of its [`Concern`] type, whose variants
match the [`Success`] and [`Failure`] of [`Outcome`], as well as the associated
function [`Outcome::acclimate`], which returns a `Result<Concern<S, M>, F>`.

**NOTE**: This associated function will be deprecated once [`Try`] has been
stabilized.

[`Result`]: core::result::Result
[`Try`]: core::ops::Try

[`Outcome::acclimate`]: crate::prelude::Outcome::acclimate
[`Concern`]: crate::prelude::Concern
[`Success`]: crate::prelude::Success
[`Failure`]: crate::prelude::Failure
[`Outcome`]: crate::prelude::Outcome

[1]: https://sled.rs/errors.html#making-unhandled-errors-unrepresentable
[2]: https://crates.io/crates/nom
