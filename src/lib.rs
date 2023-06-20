//! # code-path
//!
//! A code path macro
//!
//! ## Usage
//! ```rust
//! use code_path::code_path;
//!
//! fn foo() -> &'static str {
//!     fn bar() -> &'static str {
//!         code_path!()
//!     }
//!     bar()
//! }
//!
//! assert_eq!(foo(), "rust_out::main::_doctest_main_src_lib_rs_6_0::foo::bar");
//! ```

#![warn(clippy::all, missing_docs, nonstandard_style, future_incompatible)]

/// Returns the current code path. It could be used in errors, logs, etc to avoid typos.
#[macro_export]
macro_rules! code_path {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            ::std::any::type_name::<T>()
        }
        let mut name = type_name_of(f);
        name = &name[..name.len() - 3];
        while name.ends_with("::{{closure}}") {
            name = &name[..name.len() - 13];
        }
        name
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nesting() {
        fn foo() -> &'static str {
            fn bar() -> &'static str {
                code_path!()
            }
            bar()
        }

        assert_eq!(foo(), "code_path::tests::nesting::foo::bar");
    }

    #[test]
    fn ending_cloures() {
        fn foo() -> &'static str {
            #[allow(clippy::redundant_closure_call)]
            (|| (|| code_path!())())()
        }
        assert_eq!(foo(), "code_path::tests::ending_cloures::foo");
    }
}
