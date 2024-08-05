#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]
#![deny(
    clippy::all,
    clippy::complexity,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    future_incompatible,
    keyword_idents,
    let_underscore,
    missing_docs,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unreachable_pub,
    unused
)]
#![warn(clippy::nursery)]
use std::{
    fmt,
    ops::{Deref, DerefMut},
};

/// Represents path in the code
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodePath(String);

impl fmt::Display for CodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for CodePath {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<String> for CodePath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<CodePath> for String {
    fn from(val: CodePath) -> Self {
        val.0
    }
}

impl Deref for CodePath {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CodePath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Returns the current code scope with location, e.g.
/// `code_path::tests::scope_path::foo::bar, src/lib.rs:80:17`
#[macro_export]
macro_rules! code_path {
    () => {
        $crate::CodePath::from(format!(
            "{}, {}",
            $crate::code_scope!(),
            $crate::code_loc!()
        ))
    };
}

/// Returns the current scope path, e.g. `my_crate::my_module::my_function`)
#[macro_export]
macro_rules! code_scope {
    () => {{
        const fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            ::std::any::type_name::<T>()
        }
        type_name_of(f)
            .strip_suffix("::f")
            .unwrap_or_default()
            .trim_end_matches("::{{closure}}")
    }};
}

/// Returns the code location: `file_name:line:column`
#[macro_export]
macro_rules! code_loc {
    () => {
        concat!(file!(), ":", line!(), ":", column!())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nesting() {
        fn foo() -> &'static str {
            fn bar() -> &'static str {
                code_scope!()
            }
            bar()
        }

        assert_eq!(foo(), "code_path::tests::nesting::foo::bar");
    }

    #[test]
    fn ending_cloures() {
        fn foo() -> &'static str {
            #[allow(clippy::redundant_closure_call)]
            (|| (|| code_scope!())())()
        }
        assert_eq!(foo(), "code_path::tests::ending_cloures::foo");
    }
}
