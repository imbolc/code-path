# code-path

[![License](https://img.shields.io/crates/l/code-path.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/code-path.svg)](https://crates.io/crates/code-path)
[![Docs.rs](https://docs.rs/code-path/badge.svg)](https://docs.rs/code-path)

A code path macro

## Usage
```rust

fn foo() {
    fn bar() {
        assert_eq!(
            code_path::code_path!(),
            "rust_out::main::_doctest_main_src_lib_rs_10_0::foo::bar, src/lib.rs:9:13".into(),
        );
    }
    bar()
}
foo()
```

## Contributing

We appreciate all kinds of contributions, thank you!

If you have [rusty-hook] installed the changes will apply automatically on commit.

## License

This project is licensed under the [MIT license](LICENSE).

[rusty-hook]: https://github.com/swellaby/rusty-hook
