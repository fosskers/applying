//! Apply functions in method-position.
//!
//! # The Problem
//!
//! Much functionality in Rust is provided by methods, but occasionally we're
//! forced to use certain standalone functions. Examples are functions like
//! [`std::str::from_utf8`] or [`std::sync::Arc::new`]:
//!
//! ```ignore
//! let user = fetch_user()
//!     .await
//!     .map_err(Error::UserIsLost)?;
//! let arc = Arc::new(user);
//! Ok(arc)
//! ```
//!
//! Many functions that we write are also effectful, and require us to wrap some
//! final value in an `Ok`. The combination of these factors means that we often
//! have to bind values to names, even when we don't want to. Naming is hard,
//! and bad names can cause confusion for later readers. In the above, who
//! benefits from seeing a symbol named `arc`? Names should reflect what
//! something _is_, not what it's wrapped in.
//!
//! And what if the only appropriate name would be the one it already had, say
//! in the case of a "response" value, etc.? This encourages "variable
//! shadowing", as in:
//!
//! ```ignore
//! let resp = make_request(req).await?;
//! let resp = foo(resp);
//! let resp = bar(resp);
//! ```
//!
//! This code is fragile to ownership and refactors. We shouldn't have to write
//! code that looks like this.
//!
//! # The Solution
//!
//! This library provides the trait [`Apply`], which exposes an `apply` method
//! that is injected into all types via a "blanket implementation". `apply`
//! allows you to call top-level functions in a chain with the rest of your
//! method calls. This way, the original example becomes:
//!
//! ```ignore
//! fetch_user()
//!     .await
//!     .map_err(Error::UserIsLost)?
//!     .apply(Arc::new)
//!     .apply(Ok)
//! ```
//!
//! Ah, beautiful, consistent nesting. And no spurrious names to confuse the
//! peasantry.

#![deny(missing_docs)]

/// Apply functions in method-position.
///
/// See the module documentation for more information.
pub trait Apply {
    /// Apply a given function in method-position.
    fn apply<F, U>(self, f: F) -> U
    where
        F: FnOnce(Self) -> U,
        Self: Sized;
}

impl<T> Apply for T {
    fn apply<F, U>(self, f: F) -> U
    where
        F: FnOnce(Self) -> U,
        Self: Sized,
    {
        f(self)
    }
}
