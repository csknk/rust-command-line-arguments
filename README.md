Command Line Arguments in Rust
==============================

Separation of Concerns
----------------------
The `main` function should not have too many tasks. Because it cannot be tested directly, it shouldn't contain programme logic.

There is a Rust community guideline for splitting separate concerns when `main` gets large:

- Split programme into `main.rs` and `lib.rs`, moving logic into `lib.rs`.
- If command line parsing logic is small, keep it in `main.rs`.
- If command line logic is complicated, move it into lib.rs.

After this process, the `main` function responsibilities should be:

- Calling command line parsing logic with argument values.
- Setting up configuration values.
- Calling a `run` function in `lib.rs`.
- Handling the error if `run` returns an error.

This setup lets the programme logic to be tested by testing `lib.rs` - the code in `main.rs` should be small enough to check for correctness by reading it manually.
