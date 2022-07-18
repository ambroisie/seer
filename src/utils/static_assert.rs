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
    ($condition:expr) => {
        // Based on the latest one in `rustc`'s one before it was [removed].
        //
        // [removed]: https://github.com/rust-lang/rust/commit/c2dad1c6b9f9636198d7c561b47a2974f5103f6d
        #[allow(dead_code)]
        const _: () = [()][!($condition) as usize];
    };
}

// I want it namespaced, even though it is exported to the root of the crate by `#[macro_export]`.
pub use static_assert;
