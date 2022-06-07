# Rust Exercises
Answers to basic Dapp problems using Rust

Like strumming a guitar or painting on Adobe Illustrator, I learn best through exercises. This collection is organized by theme and aims to explore how to develop a Dapp.

## 00 - Getting Started

Use [```rustup```](https://rustup.rs/) to install rust.
```
 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify your work
```
 which rustup
```

Create a new project using ```cargo```
```
 cargo new --bin <name_of_project>
```

## 01 - CLI with ```clap```
[```clap```](https://crates.io/crates/clap) is a command-line argument parser which we will use for our CLI. Our CLI will actually be a Rust Macro. 

In this example, we will capture the crate version similar to what you would present in cargo.

```
cargo run -- -- RUST
```
**Note:** The ```--``` marks the end of the arguments to cargo.

