//! # code-path
//!
//! A code path macro
//!
//! ## Usage
//! ```rust
//! fn foo() -> String {
//!     fn bar() -> String {
//!         code_path::with_loc!()
//!     }
//!     bar()
//! }
//! assert_eq!(foo(), "rust_out::main::_doctest_main_src_lib_rs_6_0::foo::bar, src/lib.rs:7:9");
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

/// Returns the code location
#[macro_export]
macro_rules! code_loc {
    () => {
        concat!(file!(), ":", line!(), ":", column!())
    };
}

/// Returns the code location
#[macro_export]
macro_rules! with_loc {
    () => {
        format!("{}, {}", $crate::code_path!(), $crate::code_loc!())
    };
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

    #[test]
    fn with_loc() {
        fn foo() -> String {
            fn bar() -> String {
                with_loc!()
            }
            bar()
        }

        assert_eq!(
            foo(),
            "code_path::tests::with_loc::foo::bar, src/lib.rs:80:17"
        );
    }
}
