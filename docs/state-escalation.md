# State Escalation

Using the `parse_version` example seen earlier in our documentation, we can see
that the `Version` enum can return either `V1` or `V2`. At some point, a tool
parsing some version number might choose to deprecate support for `V1`. In
these instances, a developer might choose to turn `V1` into a `Mistake`.
However, `parse_version` returns a `Success(V1)`. How, then, can we turn this
into a `Mistake` *in-place*?

This is where *state escalation* comes into play. It is one of the more
experimental features that `outcome` is trying to experiment with. The basic
idea of this concept is a successful outcome for *one function* does not imply
that the caller considers the successful value valid enough to continue.

With state escalation, we not only *move* the possible state of an `Outcome`
from `Success` to `Mistake` or `Mistake` to `Failure`, but we are also able to
eliminate possible states from the `Outcome`. That is, given an `Outcome<S, M,
F>`, the first state escalation would return an `Outcome<!, M, F>` (on
stable, this returns an `Outcome<Infallible, M, F>`. A second state
escalation would result in an `Outcome<!, !, F>`. This allows for *fast* state
transitions with little to no effort, while keeping this state transition
separate from mapping operations. The benefit of reducing the possible set of
states that the `Outcome` can represent is simply a *side effect* of Rust's
powerful type system with regards to `enum`s and their variants.

It is important to note that there is no *de-escalation* of state possible
without explicit operations from users. In other words, a new Outcome must be
generated when one or more fields are `Infallible`. This is difficult to
enforce without certain features missing from Rust, specifically partial
specialization and `impl` overloading (e.g., an `Outcome<!, M, F>` should *not*
be allowed to have `and_then` called on it). However this same limitation is
found in `Result` and other variants, and even Finite State Machines
implemented with Rust `enum`s will suffer the same fate. In an ideal world,
Outcome's state escalation would only permit unidirectional transitions, and
would require the creation of a completely *new* `Outcome` if a user wanted to
reset or *de-escalate* the `Outcome`. This might not ever be possible to do or
enforce in Rust, so in the meantime it is up to users to behave themselves with
regards to escalating state.
