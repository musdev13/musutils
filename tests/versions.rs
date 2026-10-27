use musutils::{available_modules, is_module_available, module_generation, supported_generations};

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
fn fs_generation_matches_features() {
    if cfg!(feature = "fs-v1") {
        assert_eq!(module_generation("fs"), Some(1));
    }
    if cfg!(feature = "fs-v2") && !cfg!(feature = "fs-v1") {
        assert_eq!(module_generation("fs"), Some(2));
    }
}

#[test]
fn unknown_module() {
    assert!(!is_module_available("nope"));
    assert_eq!(module_generation("nope"), None);
    assert!(supported_generations("nope").is_empty());
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
