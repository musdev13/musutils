# Modules overview

`musutils` is split into six modules, each with up to two generations. Every module is optional and enabled through a Cargo feature; see [Features](../getting-started/features.md) for the exact rules.

| Module | Generations | Purpose |
|---|---|---|
| [`color`](color/v1.md) | [v1](color/v1.md) · [v2](color/v2.md) | ANSI colors and colored terminal text |
| [`fs`](fs/v1.md) | [v1](fs/v1.md) · [v2](fs/v2.md) | Files, directories, paths and JSON configuration |
| [`http`](http/v1.md) | [v1](http/v1.md) · [v2](http/v2.md) | HTTP downloads, JSON, SHA-1 verification, async downloader |
| [`os`](os/v1.md) | [v1](os/v1.md) · [v2](os/v2.md) | Operating system and architecture detection |
| [`types`](types/v1.md) | [v1](types/v1.md) · [v2](types/v2.md) | `Status`, `deoption`, line drawing |
| [`vec`](vec/v1.md) | [v1](vec/v1.md) · [v2](vec/v2.md) | Pagination and JSON table output |

## v1 vs v2 at a glance

The two generations of a module solve the same problem with different ergonomics. In general:

- **v1** keeps the original behavior with minimal typing. Many functions return plain values, `bool` or `io::Result`, and some helpers panic on failure rather than returning a typed error.
- **v2** introduces typed errors, richer traits (`Display`, `Eq`, `Hash`, `Debug`), atomic file operations, and helpers that respect environment variables or add explicit `Result`-returning variants.

The lists below are summaries. Each generation has its own page with the full API.

### `color`

- **v1** — `Colors` enum, `color_str`. Returns ANSI-escaped strings unconditionally.
- **v2** — adds `color_str_auto` (respects `NO_COLOR`, `CLICOLOR`, `TERM=dumb`, and `IsTerminal`), `color_supported()`, `Display` and `Eq`/`Hash` for `Colors`.

### `fs`

- **v1** — thin wrappers over `std::fs` plus `tilda_desir` (tilde expansion + normalization). `config` submodule stores files under the user's config directory.
- **v2** — `Result`-returning config subsystem (`Config`, `ConfigError`), atomic writes (`write_atomic`), expanded path helpers (`expand`, `normalize`, `current_dir`), and aliases keeping the v1 names.

### `http`

- **v1** — simple helpers (`get_file`, `get_json_str`, `get_github_json_str`, `download_to`, `download_to_sha1`) built on `reqwest::get`, plus an `AsyncDownloader` with unbounded retries.
- **v2** — shared `reqwest::Client` (with timeouts and connection pooling), typed `HttpError`, bounded retries with exponential backoff, and an `AsyncDownloader` that reports progress and collects per-task errors.

### `os`

- **v1** — `OS` / `Arch` enums without `Debug` / `Display`.
- **v2** — adds `Debug`, `Display`, `Eq`, `Hash` and `as_str()` on both enums.

### `types`

- **v1** — `Status` with `as_str` / `as_colored_str`, `deoption`, and `line::draw` / `line::draw_colored`.
- **v2** — adds `Display` and `Eq`/`Hash` on `Status`, exposes `draw` / `draw_colored` at the module root, and marks `deoption` as deprecated.

### `vec`

- **v1** — `get_page` (0-based) and `print_json_as_table` (prints to stdout).
- **v2** — adds 1-based `page` returning `Result<_, PageError>`, and `json_as_table` returning `Result<String, TableError>` with column widths that adapt to the data.

## Enabling modules

Modules are turned on with `<module>-<generation>` features. Because `default = []`, nothing is enabled out of the box:

```toml
[dependencies]
musutils = { git = "https://github.com/musdev13/musutils", features = ["color-v2", "types-v2", "os-v2"] }
```

The two generations of a single module are mutually exclusive.

## Runtime inspection

`available_modules()` and related helpers report which modules are compiled into the current build. See [Core API](../lib.md).

## See also

- [Installation](../getting-started/installation.md)
- [Features](../getting-started/features.md)