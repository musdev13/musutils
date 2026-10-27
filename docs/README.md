# musutils

**Rust utilities for your projects**

[![License: MPL-2.0](https://img.shields.io/badge/license-MPL--2.0-blue.svg)](https://github.com/musdev13/musutils/blob/dev/LICENSE)

`musutils` is a small Rust utility crate: colored terminal output, filesystem and configuration helpers, HTTP downloads, OS/architecture detection, vector pagination, JSON table formatting, and a handful of supporting types.

The crate is **modular**. Every module is optional and enabled through Cargo features, and modules are versioned independently — most ship in two generations, `v1` and `v2`.

## Features

- **`color`** — ANSI palette (`Colors`) and `color_str`, plus environment-aware `color_str_auto` (respects `NO_COLOR`, `CLICOLOR`, `TERM=dumb`, and whether stdout is a terminal) in v2.
- **`fs`** — copy/write/create/remove helpers, tilde expansion, path normalization, and JSON configuration files. v2 adds atomic writes and a typed `Config` with `ConfigError`.
- **`http`** — `get_file`, `get_json_str`, `get_github_json_str`, `download_to`, SHA-1-verified `download_to_sha1`, and an `AsyncDownloader` with bounded concurrency. v2 adds a shared client, typed `HttpError`, and bounded retries.
- **`os`** — `get_os()` / `get_arch()` returning `OS` / `Arch` enums. v2 adds `Debug`, `Display`, `Eq`, `Hash`, and `as_str()`.
- **`types`** — `Status` labels with optional ANSI coloring, `deoption`, and `draw` / `draw_colored`. v2 adds `Display` and `Eq`/`Hash` on `Status`.
- **`vec`** — pagination (`get_page`, `page`) and JSON table rendering (`json_as_table`, `print_json_as_table`).

## Installation

`musutils` is not published on crates.io yet. Add it as a git dependency, enabling the modules you need:

```toml
[dependencies]
musutils = { git = "https://github.com/musdev13/musutils", features = [
    "color-v2",
    "types-v2",
    "fs-v2",
    "os-v2",
] }
```

No module is enabled by default (`default = []`). Feature names follow `<module>-<generation>`: `color-v1`/`color-v2`, `fs-v1`/`fs-v2`, `http-v1`/`http-v2`, `os-v1`/`os-v2`, `types-v1`/`types-v2`, `vec-v1`/`vec-v2`.

The two generations of a module are **mutually exclusive** — enabling both fails the build with `compile_error!`. Some modules have inter-module dependencies (`fs-v1 → types`, `types → color`, `http-v1 → fs + types`); these are enforced at compile time. See [Features](pages/getting-started/features.md) in the documentation.

## Quick start

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

### Configuration files (v2)

```rust
use musutils::fs::config::Config;
use serde_json::json;

let mut cfg = Config::from_value(json!({}));
cfg.set("window.width", json!(1280)).unwrap();
cfg.save_to("config.json").unwrap();
```

### Pagination (v2)

```rust
use musutils::vec::page;

let items = vec![1, 2, 3, 4, 5];
let second = page(&items, 2, 2).unwrap(); // 1-based: [3, 4]
```

## Documentation

Full documentation lives in the [`docs/`](docs/) directory of this repository:

- [Getting started](docs/pages/getting-started/installation.md) — installation and features.
- [Core API](docs/pages/lib.md) — `available_modules()`, `VERSION`, the `log!` macro, and other top-level items.
- [Modules overview](docs/pages/modules/index.md) — the full list of modules and their generations.

The same content is served as a [static site](https://musdev13.github.io/musutils/) when the MkDocs site is built from `mkdocs.yml`.

## Building and testing

The crate is tested with one generation at a time. A convenience `Makefile` is provided:

```bash
make test     # cargo test for both v1 and v2
make lint     # cargo fmt --check + cargo clippy -D warnings for both generations
make check    # test + lint
```

`cargo test --all-features` is **not supported** — v1 and v2 are mutually exclusive.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for the structure of the crate, feature conventions, and the checklist before submitting a PR.

Pull requests target the `dev` branch.

## License

`musutils` is distributed under the [Mozilla Public License 2.0](LICENSE).