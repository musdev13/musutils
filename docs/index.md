#

<div align="center" markdown="1">


<h1> musutils </h1>

<b>Rust utilities for your projects</b><br>

<a href="https://github.com/musdev13/musutils">GitHub</a>
<a href="https://github.com/musdev13/musutils/blob/dev/CONTRIBUTING.md">Contributing</a>
<a href="https://github.com/musdev13/musutils/blob/dev/LICENSE">License: MPL-2.0</a>


</div>

---

## What is musutils

`musutils` is a small Rust utility crate: colored terminal output, filesystem and configuration helpers, HTTP downloads, OS/architecture detection, vector pagination, JSON table formatting, and a handful of supporting types.

The crate is **modular**. Every module is optional and enabled through Cargo features, and modules are versioned independently — most ship in two generations, `v1` and `v2`.

## Quick start

```toml
[dependencies]
musutils = { git = "https://github.com/musdev13/musutils", features = [
    "color-v2",
    "types-v2",
    "fs-v2",
    "os-v2",
] }
```

```rust
use musutils::color::{Colors, color_str};
use musutils::types::Status;
use musutils::os::{get_os, get_arch};

fn main() {
    println!("{}", color_str("hello, musutils", Colors::Cyan));
    println!("{}", Status::Ok.as_colored_str());
    println!("os = {:?}, arch = {:?}", get_os(), get_arch());
}
```

## Where to go next

- **[Installation](pages/getting-started/installation.md)** — how to add `musutils` to a project.
- **[Features](pages/getting-started/features.md)** — how modules and generations are enabled.
- **[Modules overview](pages/modules/index.md)** — the full list of modules and their generations.
- **[Core API](pages/lib.md)** — `available_modules()`, `VERSION`, the `log!` macro, and other top-level items.

## Requirements

- Rust `edition = "2024"` (stable toolchain).

## License

`musutils` is distributed under the [Mozilla Public License 2.0](https://github.com/musdev13/musutils/blob/dev/LICENSE).