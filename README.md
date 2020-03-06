Command Line Arguments in Rust
==============================
As with other languages like C and C++, Rust programmes can accept command line arguments. This provides a way for you to pass values from the command line to be used during execution of code.

This repo shows some options for handling command line arguments in Rust.

Separation of Concerns
----------------------
The `main` function should not ibe burdened with too many tasks. It cannot be tested directly, so it shouldn't contain programme logic.

There is a Rust community guideline for separation of concerns when `main` gets large:

- Split programme into `main.rs` and `lib.rs`, with programme logic in `lib.rs`.
- Only allow simple command line parsing logic in `main.rs`.
- Complicated command line logic should be held in `lib.rs`.

The responsibilities of the `main` function should be:

- Calling command line parsing logic with argument values.
- Setting up configuration values.
- Calling a `run` function in `lib.rs` - delegating programme logic.
- Handling errors that might be returned by `run`.

This setup lets the programme logic to be tested by testing `lib.rs` - the code in `main.rs` should be small enough to check for correctness by reading it manually.

References
----------
* [Article in the online Rust book][1]

[1]: https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html

