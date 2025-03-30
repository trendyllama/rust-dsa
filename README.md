# rust-dsa


## Build Instructions

```sh
cargo build

```

## Overview

- My first time using rust and cargo.
- Easy to setup additional libraries.

```sh
cargo new --lib dsa
```

This command makes a library target - any additional source files have to be referenced in the `lib.rs` file like...
```rs
pub mod node;
```

assuming a structure like

dsa
--src
-----lib.rs
-----node.rs

Then reference the library in the main binary target package.
```toml
[dependencies]
dsa = { path = "./dsa" }
```


## Testing

you will need to add preprocessor directives to mark your tests, the compiler will then be able to find them and execute them.

```sh

cd dsa

cargo test
```
