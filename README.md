<!-- cargo-rdme start -->

Apply functions in method-position.

# The Problem

Much functionality in Rust is provided by methods, but occasionally we're
forced to use certain standalone functions. Examples are functions like
[`std::str::from_utf8`] or [`std::sync::Arc::new`]:

```rust
let user = fetch_user()
    .await
    .map_err(Error::UserIsLost)?;
let arc = Arc::new(user);
Ok(arc)
```

Many functions that we write are also effectful, and require us to wrap some
final value in an `Ok`. The combination of these factors means that we often
have to bind values to names, even when we don't want to. Naming is hard,
and bad names can cause confusion for later readers. In the above, who
benefits from seeing a symbol named `arc`? Names should reflect what
something _is_, not what it's wrapped in.

And what if the only appropriate name would be the one it already had, say
in the case of a "response" value, etc.? This encourages "variable
shadowing", as in:

```rust
let resp = make_request(req).await?;
let resp = foo(resp);
let resp = bar(resp);
```

This code is fragile to ownership and refactors. We shouldn't have to write
code that looks like this.

# The Solution

This library provides the trait [`Apply`], which exposes an `apply` method
that is injected into all types via a "blanket implementation". `apply`
allows you to call top-level functions in a chain with the rest of your
method calls. This way, the original example becomes:

```rust
fetch_user()
    .await
    .map_err(Error::UserIsLost)?
    .apply(Arc::new)
    .apply(Ok)
```

Ah, beautiful, consistent nesting. And no spurrious names to confuse the
peasantry.

<!-- cargo-rdme end -->
