[![License](https://img.shields.io/crates/l/code-path.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/code-path.svg)](https://crates.io/crates/code-path)
[![Docs.rs](https://docs.rs/code-path/badge.svg)](https://docs.rs/code-path)

<!-- cargo-sync-readme start -->

# code-path

A code path macro

## Usage
```rust

fn foo() {
    fn bar() {
        assert_eq!(
            code_path!(),
            "rust_out::main::_doctest_main_src_lib_rs_6_0::foo::bar, src/lib.rs:10:13".into(),
        );
    }
    bar()
}
foo()
```

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
