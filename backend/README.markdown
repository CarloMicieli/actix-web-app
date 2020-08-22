# Trenako (Rust backend)

## Setup

Rustup is the starting point to setup the Rust toolchain:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Useful components include:

* `rustfmt`: Rust code formatter
* `watch` to watch changes
* `clippy`: Rust linter code 

```
$ rustup component add rustfmt

$ rustup component add clippy

$ cargo install cargo-watch 
```

## Build & Run

The project requires Rust 1.45+.

To build the project

```
$ cargo check
```

To run the project 

```
$ cargo run
```

## Tests

To run the test suite

```
$ cargo test
```


