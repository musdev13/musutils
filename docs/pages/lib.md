# Core API

These items live at the crate root and are available regardless of which module features are enabled.

```rust
use musutils::{
    available_modules, is_module_available, module_generation,
    supported_generations, ModuleInfo, NAME, VERSION,
};
```

## Constants

### `VERSION: &str`

Crate version as recorded in `Cargo.toml` (via `env!("CARGO_PKG_VERSION")`).

```rust
assert_eq!(musutils::VERSION, "0.5.0");
```

### `NAME: &str`

Crate name, `env!("CARGO_PKG_NAME")` — always `"musutils"`.

## `ModuleInfo`

```rust
pub struct ModuleInfo {
    pub name: &'static str,
    pub generation: u32,
}
```

Describes one module generation compiled into the current build. Derives `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`.

- `name` — module name (`"color"`, `"fs"`, `"http"`, `"os"`, `"types"`, `"vec"`).
- `generation` — generation number, `1` or `2`.

## `available_modules() -> Vec<ModuleInfo>`

Returns every module generation compiled into the current build. One entry per module, ordered: `fs`, `vec`, `http`, `types`, `color`, `os`.

```rust
// With features = ["os-v2"]:
let mods = musutils::available_modules();
assert_eq!(mods, vec![
    musutils::ModuleInfo { name: "os", generation: 2 },
]);
```

With no module features enabled, the function returns an empty `Vec`.

## `is_module_available(name: &str) -> bool`

`true` if the module with the given name is present in the current build.

```rust
assert!(musutils::is_module_available("os"));  // if os-v1 or os-v2 enabled
assert!(!musutils::is_module_available("nope"));
```

Matching is case-sensitive and exact.

## `module_generation(name: &str) -> Option<u32>`

Returns the enabled generation for the given module, or `None` if the module is not compiled in.

```rust
// With features = ["fs-v2"]:
assert_eq!(musutils::module_generation("fs"), Some(2));
assert_eq!(musutils::module_generation("nope"), None);
```

## `supported_generations(name: &str) -> &'static [u32]`

Returns the generations that the crate knows about for the given module — `&[1, 2]` for any of the six known modules, and `&[]` for unknown names. This is a **static** description of the crate's capabilities, not the current build.

```rust
assert_eq!(musutils::supported_generations("fs"), &[1, 2]);
assert!(musutils::supported_generations("nope").is_empty());
```

## The `log!` macro

Available whenever at least one `types-*` feature is enabled (it lives in `types_macros.rs`, gated behind `types-v1` / `types-v2`).

```rust
musutils::log!(is_json, "value = {}", 42);
```

Expands to:

```rust
if !is_json { println!("value = {}", 42); }
```

- `is_json` — any expression convertible to `bool`.
- Remaining arguments — anything `println!` accepts.

If `is_json` is `true`, nothing is printed. If `false`, the arguments are forwarded to `println!`. Useful for tools that support both human-readable and JSON output.

Because the `if` is part of the macro body (not a match arm), the formatted arguments are still evaluated when `is_json` is `true` — with the exception of values that are moved into the `println!` invocation. Keep this in mind for expensive formatting.

## Feature matrix

The crate root is always compiled. The module namespaces (`musutils::color`, `musutils::fs`, …) appear only when the corresponding `<module>-v1` or `<module>-v2` feature is enabled. See [Features](getting-started/features.md) for the full dependency graph.

## See also

- [Modules overview](modules/index.md)