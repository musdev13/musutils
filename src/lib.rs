/* This Source Code Form is subject to the terms of the
Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed
with this file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
pub mod color;
#[cfg(any(feature = "fs-v1", feature = "fs-v2"))]
pub mod fs;
#[cfg(any(feature = "http-v1", feature = "http-v2"))]
pub mod http;
#[cfg(any(feature = "os-v1", feature = "os-v2"))]
pub mod os;
#[cfg(any(feature = "types-v1", feature = "types-v2"))]
pub mod types;
#[cfg(any(feature = "vec-v1", feature = "vec-v2"))]
pub mod vec;

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
mod types_macros;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

#[cfg(all(
    feature = "fs-v1",
    not(any(feature = "types-v1", feature = "types-v2"))
))]
compile_error!("feature `fs-v1` requires `types-v1` or `types-v2`");

#[cfg(all(
    feature = "types-v1",
    not(any(feature = "color-v1", feature = "color-v2"))
))]
compile_error!("feature `types-v1` requires `color-v1` or `color-v2`");

#[cfg(all(
    feature = "types-v2",
    not(any(feature = "color-v1", feature = "color-v2"))
))]
compile_error!("feature `types-v2` requires `color-v1` or `color-v2`");

#[cfg(all(feature = "http-v1", not(any(feature = "fs-v1", feature = "fs-v2"))))]
compile_error!("feature `http-v1` requires `fs-v1` or `fs-v2`");

#[cfg(all(
    feature = "http-v1",
    not(any(feature = "types-v1", feature = "types-v2"))
))]
compile_error!("feature `http-v1` requires `types-v1` or `types-v2`");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleInfo {
    pub name: &'static str,
    pub generation: u32,
}

#[allow(unused_mut, clippy::vec_init_then_push)]
pub fn available_modules() -> Vec<ModuleInfo> {
    let mut out = Vec::new();

    #[cfg(feature = "fs-v1")]
    out.push(ModuleInfo {
        name: "fs",
        generation: 1,
    });
    #[cfg(feature = "fs-v2")]
    out.push(ModuleInfo {
        name: "fs",
        generation: 2,
    });

    #[cfg(feature = "vec-v1")]
    out.push(ModuleInfo {
        name: "vec",
        generation: 1,
    });
    #[cfg(feature = "vec-v2")]
    out.push(ModuleInfo {
        name: "vec",
        generation: 2,
    });

    #[cfg(feature = "http-v1")]
    out.push(ModuleInfo {
        name: "http",
        generation: 1,
    });
    #[cfg(feature = "http-v2")]
    out.push(ModuleInfo {
        name: "http",
        generation: 2,
    });

    #[cfg(feature = "types-v1")]
    out.push(ModuleInfo {
        name: "types",
        generation: 1,
    });
    #[cfg(feature = "types-v2")]
    out.push(ModuleInfo {
        name: "types",
        generation: 2,
    });

    #[cfg(feature = "color-v1")]
    out.push(ModuleInfo {
        name: "color",
        generation: 1,
    });
    #[cfg(feature = "color-v2")]
    out.push(ModuleInfo {
        name: "color",
        generation: 2,
    });

    #[cfg(feature = "os-v1")]
    out.push(ModuleInfo {
        name: "os",
        generation: 1,
    });
    #[cfg(feature = "os-v2")]
    out.push(ModuleInfo {
        name: "os",
        generation: 2,
    });

    out
}

pub fn is_module_available(name: &str) -> bool {
    available_modules().iter().any(|m| m.name == name)
}

pub fn module_generation(name: &str) -> Option<u32> {
    available_modules()
        .iter()
        .find(|m| m.name == name)
        .map(|m| m.generation)
}

pub fn supported_generations(name: &str) -> &'static [u32] {
    match name {
        "fs" | "vec" | "http" | "types" | "color" | "os" => &[1, 2],
        _ => &[],
    }
}
