# Installation

`musutils` is not published on crates.io yet. Add it as a git dependency.

## As a git dependency

```toml
[dependencies]
musutils = { git = "https://github.com/musdev13/musutils", features = ["color-v2", "types-v2"] }
```

Pin to a specific revision or branch if you need reproducibility:

```toml
[dependencies]
musutils = { git = "https://github.com/musdev13/musutils", rev = "<commit-sha>", features = ["color-v2", "types-v2"] }
```

## As a path dependency

When working on `musutils` locally, point `Cargo.toml` at the checkout:

```toml
[dependencies]
musutils = { path = "../musutils", features = ["color-v2", "types-v2"] }
```

## Choosing features

No module is enabled by default (`default = []`). Every module must be turned on explicitly through a feature of the form `<module>-<generation>`:

- `color-v1` / `color-v2`
- `fs-v1` / `fs-v2`
- `http-v1` / `http-v2`
- `os-v1` / `os-v2`
- `types-v1` / `types-v2`
- `vec-v1` / `vec-v2`

The two generations of a single module are **mutually exclusive** — enabling both `fs-v1` and `fs-v2` fails the build with `compile_error!`. See [Features](features.md) for the full picture, including the inter-module dependencies enforced by the crate.

## Minimal setup

The smallest useful configuration is a single module:

```toml
[dependencies]
musutils = { git = "https://github.com/musdev13/musutils", features = ["os-v2"] }
```

```rust
use musutils::os::{get_arch, get_os};

fn main() {
    println!("os = {:?}, arch = {:?}", get_os(), get_arch());
}
```

## A typical application setup

A CLI that needs colors, status labels and filesystem helpers:

```toml
[dependencies]
musutils = { git = "https://github.com/musdev13/musutils", features = [
    "color-v2",
    "types-v2",
    "fs-v2",
] }
serde_json = "1"
```

> `fs-v1` requires `types-v1` or `types-v2`; `types-v1` / `types-v2` require `color-v1` or `color-v2`. `http-v1` additionally requires `fs-*` and `types-*`. `http-v2` has no inter-module dependencies. These rules are enforced at compile time — see [Features](features.md).

## Toolchain

- Rust `edition = "2024"`.
- The `http` modules pull in `reqwest` (with TLS) and `tokio`.
- The `fs` and `types` modules use `serde` / `serde_json`.

No additional system dependencies are needed beyond a working Rust toolchain.