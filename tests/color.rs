mod common;

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
use musutils::color::{Colors, color_str};

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
const ALL_COLORS: &[Colors] = &[
    Colors::Green,
    Colors::Yellow,
    Colors::Blue,
    Colors::Lime,
    Colors::Gold,
    Colors::Cyan,
    Colors::Red,
    Colors::Orange,
    Colors::Purple,
    Colors::Rose,
    Colors::Gray,
    Colors::Reset,
];

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
#[test]
fn every_color_has_nonempty_term_code() {
    for c in ALL_COLORS {
        let code = c.as_term_code();
        assert!(!code.is_empty(), "empty term code: {}", code);
        assert!(
            code.starts_with('\x1b'),
            "term code should start with ESC: {}",
            code
        );
    }
}

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
#[test]
fn reset_is_reset_code() {
    assert_eq!(Colors::Reset.as_term_code(), "\x1b[0m");
}

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
#[test]
fn term_codes_are_distinct() {
    let mut seen: Vec<&'static str> = Vec::new();
    for c in ALL_COLORS {
        let code = c.as_term_code();
        if code != "\x1b[0m" {
            assert!(!seen.contains(&code), "duplicate term code {}", code);
        }
        seen.push(code);
    }
}

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
#[test]
fn color_str_wraps_text() {
    let s = color_str("hello", Colors::Red);
    assert_eq!(s, format!("{}{}{}", "\x1b[31m", "hello", "\x1b[0m"));
}

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
#[test]
fn color_str_empty_text() {
    let s = color_str("", Colors::Green);
    assert_eq!(s, format!("{}{}", "\x1b[32m", "\x1b[0m"));
}

#[cfg(any(feature = "color-v1", feature = "color-v2"))]
#[test]
fn color_str_unicode_preserved() {
    let s = color_str("привет 🦀", Colors::Cyan);
    assert!(s.contains("привет 🦀"));
    assert!(s.starts_with('\x1b'));
    assert!(s.ends_with("\x1b[0m"));
}

#[cfg(feature = "color-v2")]
#[test]
fn colors_display_matches_term_code() {
    for &c in ALL_COLORS {
        assert_eq!(c.to_string(), c.as_term_code());
    }
}

#[cfg(feature = "color-v2")]
#[test]
fn colors_eq_hash() {
    use std::collections::HashSet;
    let mut set: HashSet<Colors> = HashSet::new();
    for &c in ALL_COLORS {
        set.insert(c);
        set.insert(c);
    }
    assert_eq!(set.len(), ALL_COLORS.len());
}

#[cfg(feature = "color-v2")]
fn suppress_no_color() {
    unsafe {
        std::env::set_var("NO_COLOR", "1");
        std::env::remove_var("CLICOLOR");
        std::env::remove_var("TERM");
    }
}

#[cfg(feature = "color-v2")]
struct SavedEnv {
    no_color: Option<std::ffi::OsString>,
    clicolor: Option<std::ffi::OsString>,
    term: Option<std::ffi::OsString>,
}

#[cfg(feature = "color-v2")]
fn save_env() -> SavedEnv {
    SavedEnv {
        no_color: std::env::var_os("NO_COLOR"),
        clicolor: std::env::var_os("CLICOLOR"),
        term: std::env::var_os("TERM"),
    }
}

#[cfg(feature = "color-v2")]
fn restore_env(saved: SavedEnv) {
    unsafe {
        match saved.no_color {
            Some(v) => std::env::set_var("NO_COLOR", v),
            None => std::env::remove_var("NO_COLOR"),
        }
        match saved.clicolor {
            Some(v) => std::env::set_var("CLICOLOR", v),
            None => std::env::remove_var("CLICOLOR"),
        }
        match saved.term {
            Some(v) => std::env::set_var("TERM", v),
            None => std::env::remove_var("TERM"),
        }
    }
}

#[cfg(feature = "color-v2")]
#[test]
fn color_str_auto_disabled_by_no_color() {
    use musutils::color::color_str_auto;
    let _g = common::env_lock();
    let saved = save_env();
    suppress_no_color();
    assert_eq!(color_str_auto("x", Colors::Red), "x");
    restore_env(saved);
}

#[cfg(feature = "color-v2")]
#[test]
fn color_str_auto_disabled_by_clicolor_zero() {
    use musutils::color::color_str_auto;
    let _g = common::env_lock();
    let saved = save_env();
    unsafe {
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("TERM");
        std::env::set_var("CLICOLOR", "0");
    }
    assert_eq!(color_str_auto("y", Colors::Green), "y");
    restore_env(saved);
}

#[cfg(feature = "color-v2")]
#[test]
fn color_str_auto_disabled_by_term_dumb() {
    use musutils::color::color_str_auto;
    let _g = common::env_lock();
    let saved = save_env();
    unsafe {
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("CLICOLOR");
        std::env::set_var("TERM", "dumb");
    }
    assert_eq!(color_str_auto("z", Colors::Blue), "z");
    restore_env(saved);
}

#[cfg(feature = "color-v2")]
#[test]
fn color_str_auto_ignores_nonzero_clicolor() {
    use musutils::color::color_str_auto;
    let _g = common::env_lock();
    let saved = save_env();
    unsafe {
        std::env::remove_var("NO_COLOR");
        std::env::remove_var("TERM");
        std::env::set_var("CLICOLOR", "1");
    }
    let out = color_str_auto("q", Colors::Red);
    assert!(out == "q" || out == color_str("q", Colors::Red));
    restore_env(saved);
}
