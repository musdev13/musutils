# Features

`musutils` is fully feature-gated: **no module is compiled unless a feature enables it** (`default = []`).

## Feature naming

Features follow the pattern `<module>-<generation>`:

- `color-v1`, `color-v2`
- `fs-v1`, `fs-v2`
- `http-v1`, `http-v2`
- `os-v1`, `os-v2`
- `types-v1`, `types-v2`
- `vec-v1`, `vec-v2`

Enabling `fs-v2`, for example, pulls in the `fs` module and re-exports the `v2` API at `musutils::fs::*`.

## Generations are mutually exclusive

The two generations of the same module cannot coexist. Enabling both is a compile-time error:

```text
error: features `fs-v1` and `fs-v2` are mutually exclusive; enable exactly one
```

The check lives in `src/<module>/mod.rs`:

```rust
#[cfg(all(feature = "fs-v1", feature = "fs-v2"))]
compile_error!("features `fs-v1` and `fs-v2` are mutually exclusive; enable exactly one");
```

This is why `cargo test --all-features` (and `cargo doc --all-features`) do not work with this crate. Test with one generation at a time — for example:

```bash
cargo test --features "color-v1 types-v1 fs-v1 vec-v1 http-v1 os-v1"
cargo test --features "color-v2 types-v2 fs-v2 vec-v2 http-v2 os-v2"
```

## Modules are optional

If no generation of a module is enabled, `musutils::<module>` simply does not exist — there is no `compile_error!` for "no version selected". This keeps small builds small: a crate that only uses `os-v2` compiles nothing else.

```toml
[dependencies]
musutils = { git = "https://github.com/musdev13/musutils", features = ["os-v2"] }
```

## Inter-module dependencies

Some features depend on others; these rules are enforced at compile time in `src/lib.rs`:

| Feature      | Requires                                    |
|--------------|---------------------------------------------|
| `fs-v1`      | `types-v1` or `types-v2`                    |
| `types-v1`   | `color-v1` or `color-v2`                    |
| `types-v2`   | `color-v1` or `color-v2`                    |
| `http-v1`    | `fs-v1` or `fs-v2`, and `types-v1` or `types-v2` |
| `http-v2`    | — (self-contained)                          |
| `color-*`    | —                                           |
| `os-*`       | —                                           |
| `vec-*`      | —                                           |

Example failure:

```text
error: feature `types-v1` requires `color-v1` or `color-v2`
```

The reason is that `types::Status::as_colored_str` calls into the color module, and `fs::v1` prints `Status`-styled error messages.

### Cross-generation combinations

Some modules can mix generations across module boundaries. For example, `types-v2` works with `color-v1`, and `types-v1` works with `color-v2` — the dependency rules only constrain the *color* module to be present, not its generation. `fs-v2` also builds on top of either `types-v1` or `types-v2` if a caller needs it, although it does not depend on `types` itself.

## Recommended feature sets

Minimal CLI that only detects OS and architecture:

```toml
features = ["os-v2"]
```

Typical CLI application that wants colored status labels and config files:

```toml
features = ["color-v2", "types-v2", "fs-v2"]
```

Downloader that fetches files with SHA-1 verification:

```toml
features = ["color-v2", "types-v2", "fs-v2", "http-v2"]
```

(`http-v2` needs no `fs` or `types` at compile time, but user code usually depends on them anyway.)

## Checking enabled modules at runtime

The crate exposes `available_modules()` and friends so an application can inspect which modules were compiled in. See [Core API](../lib.md).

## See also

- [Installation](installation.md)
- [Modules overview](../modules/index.md)