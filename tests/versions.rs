use musutils::{
    ModuleInfo, NAME, VERSION, available_modules, is_module_available, module_generation,
    supported_generations,
};

const KNOWN_MODULES: &[&str] = &["fs", "vec", "http", "types", "color", "os"];

#[test]
fn modules_reflect_enabled_features() {
    let mods = available_modules();
    let expected_count = [
        cfg!(any(feature = "fs-v1", feature = "fs-v2")),
        cfg!(any(feature = "vec-v1", feature = "vec-v2")),
        cfg!(any(feature = "http-v1", feature = "http-v2")),
        cfg!(any(feature = "types-v1", feature = "types-v2")),
        cfg!(any(feature = "color-v1", feature = "color-v2")),
        cfg!(any(feature = "os-v1", feature = "os-v2")),
    ]
    .into_iter()
    .filter(|x| *x)
    .count();
    assert_eq!(mods.len(), expected_count);
}

#[test]
fn versions_are_reexported() {
    assert_eq!(VERSION, env!("CARGO_PKG_VERSION"));
    assert_eq!(NAME, env!("CARGO_PKG_NAME"));
}

#[test]
fn module_info_debug_eq_and_fields() {
    let a = ModuleInfo {
        name: "x",
        generation: 1,
    };
    let b = ModuleInfo {
        name: "x",
        generation: 1,
    };
    assert_eq!(a, b);
    assert_eq!(a.generation, 1);
    assert_eq!(a.name, "x");
    let _ = format!("{:?}", a);
}

#[test]
fn no_duplicate_module_entries() {
    let mods = available_modules();
    for (i, m) in mods.iter().enumerate() {
        for other in mods.iter().skip(i + 1) {
            assert!(
                !(m.name == other.name),
                "duplicate entry for module {}",
                m.name
            );
        }
    }
}

#[test]
fn all_available_modules_are_known() {
    for m in available_modules() {
        assert!(
            KNOWN_MODULES.contains(&m.name),
            "unexpected module {}",
            m.name
        );
        assert!(
            supported_generations(m.name).contains(&m.generation),
            "module {} with unsupported generation {}",
            m.name,
            m.generation
        );
    }
}

#[test]
fn fs_generation_matches_features() {
    if cfg!(feature = "fs-v1") {
        assert_eq!(module_generation("fs"), Some(1));
    }
    if cfg!(feature = "fs-v2") && !cfg!(feature = "fs-v1") {
        assert_eq!(module_generation("fs"), Some(2));
    }
}

#[test]
fn every_known_module_generation_matches_features() {
    let pairs: &[(&str, bool, bool)] = &[
        ("fs", cfg!(feature = "fs-v1"), cfg!(feature = "fs-v2")),
        ("vec", cfg!(feature = "vec-v1"), cfg!(feature = "vec-v2")),
        ("http", cfg!(feature = "http-v1"), cfg!(feature = "http-v2")),
        (
            "types",
            cfg!(feature = "types-v1"),
            cfg!(feature = "types-v2"),
        ),
        (
            "color",
            cfg!(feature = "color-v1"),
            cfg!(feature = "color-v2"),
        ),
        ("os", cfg!(feature = "os-v1"), cfg!(feature = "os-v2")),
    ];
    for &(name, v1, v2) in pairs {
        let expected = if v1 {
            Some(1)
        } else if v2 {
            Some(2)
        } else {
            None
        };
        assert_eq!(module_generation(name), expected, "module {}", name);
        assert_eq!(
            is_module_available(name),
            expected.is_some(),
            "module {}",
            name
        );
    }
}

#[test]
fn unknown_module() {
    assert!(!is_module_available("nope"));
    assert!(!is_module_available(""));
    assert!(!is_module_available("FS"));
    assert_eq!(module_generation("nope"), None);
    assert_eq!(module_generation(""), None);
    assert!(supported_generations("nope").is_empty());
    assert!(supported_generations("").is_empty());
}

#[test]
fn supported_generations_known() {
    assert_eq!(supported_generations("fs"), &[1, 2]);
    assert_eq!(supported_generations("vec"), &[1, 2]);
    assert_eq!(supported_generations("http"), &[1, 2]);
    assert_eq!(supported_generations("types"), &[1, 2]);
    assert_eq!(supported_generations("color"), &[1, 2]);
    assert_eq!(supported_generations("os"), &[1, 2]);
}
