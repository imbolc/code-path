#![doc = include_str!("../README.md")]
use std::fmt;

/// Represents path in the code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CodePath {
    context: &'static str,
    location: &'static str,
    scope: &'static str,
}

impl CodePath {
    /// Creates a new code path value.
    #[must_use]
    pub const fn new(context: &'static str, location: &'static str, scope: &'static str) -> Self {
        Self {
            context,
            location,
            scope,
        }
    }
}

impl fmt::Display for CodePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.context.is_empty() {
            write!(f, "{} in ", self.context)?;
        }
        write!(f, "{} at {}", self.scope, self.location)
    }
}

impl From<CodePath> for String {
    fn from(val: CodePath) -> Self {
        val.to_string()
    }
}

/// Returns the current code scope with location, e.g.
/// `foo::bar at src/lib.rs:80:17`
///
/// Optionally accepts one or more tokens that `concat!` can combine into a
/// string literal.
#[macro_export]
macro_rules! code_path {
    ($($context:expr_2021),* $(,)?) => {
        $crate::CodePath::new(
            concat!($($context),*),
            $crate::code_loc!(),
            $crate::code_scope!(),
        )
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
        concat!(file!(), ":", line!())
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

    #[test]
    fn literal_context() {
        let CodePath { context, .. } = code_path!(42);

        assert_eq!(context, "42");

        let CodePath {
            context: multi_context,
            ..
        } = code_path!("answer: ", 42);
        assert_eq!(multi_context, "answer: 42");
    }
}
