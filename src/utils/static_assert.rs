//! Static assert.

/// Assert a condition at compile-time.
///
/// See [RFC 2790] for potential addition into Rust itself.
///
/// [RFC 2790]: https://github.com/rust-lang/rfcs/issues/2790
///
/// # Examples
///
/// ```
/// use seer::utils::static_assert;
///
/// static_assert!(42 > 0);
/// ```
#[macro_export]
macro_rules! static_assert {
    ($($tt:tt)*) => {
        #[allow(dead_code)]
        const _: () = assert!($($tt)*);
    };
}

// I want it namespaced, even though it is exported to the root of the crate by `#[macro_export]`.
pub use static_assert;
