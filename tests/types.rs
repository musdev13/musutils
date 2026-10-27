mod common;

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
use musutils::color::{Colors, color_str};

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
use musutils::types::Status;

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
const ALL_STATUSES: &[Status] = &[
    Status::Ok,
    Status::Err,
    Status::Inf,
    Status::Warn,
    Status::Task,
    Status::Note,
    Status::Quote,
    Status::No,
    Status::Yes,
    Status::Question,
    Status::Option,
    Status::Select,
    Status::Status,
];

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
fn status_as_str_values() {
    let cases = [
        (Status::Ok, "!Ok"),
        (Status::Err, "!Err"),
        (Status::Inf, ":Info"),
        (Status::Warn, "!Warn"),
        (Status::Task, ":Task"),
        (Status::Note, ":!Note"),
        (Status::Quote, ":>Quote"),
        (Status::No, "!No"),
        (Status::Yes, "!Yes"),
        (Status::Question, "?Question"),
        (Status::Option, ":?Option"),
        (Status::Select, ":?Select"),
        (Status::Status, ":Status"),
    ];
    for (s, want) in cases {
        assert_eq!(s.as_str(), want);
    }
}

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
fn status_as_str_unique() {
    let mut seen: Vec<&'static str> = Vec::new();
    for s in ALL_STATUSES {
        let v = s.as_str();
        assert!(!seen.contains(&v), "duplicate status str: {}", v);
        seen.push(v);
    }
}

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
fn status_colored_wraps_with_esc_and_text() {
    for s in ALL_STATUSES {
        let colored = s.as_colored_str();
        assert!(
            colored.contains(s.as_str()),
            "missing text in {:?}",
            colored
        );
        assert!(colored.starts_with('\x1b'), "missing esc in {:?}", colored);
        assert!(
            colored.ends_with("\x1b[0m"),
            "missing reset in {:?}",
            colored
        );
    }
}

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
fn status_colored_matches_expected_color() {
    assert_eq!(Status::Ok.as_colored_str(), color_str("!Ok", Colors::Lime));
    assert_eq!(
        Status::Yes.as_colored_str(),
        color_str("!Yes", Colors::Lime)
    );
    assert_eq!(
        Status::Err.as_colored_str(),
        color_str("!Err", Colors::Rose)
    );
    assert_eq!(Status::No.as_colored_str(), color_str("!No", Colors::Rose));
    assert_eq!(
        Status::Warn.as_colored_str(),
        color_str("!Warn", Colors::Orange)
    );
    assert_eq!(
        Status::Inf.as_colored_str(),
        color_str(":Info", Colors::Gold)
    );
    assert_eq!(
        Status::Status.as_colored_str(),
        color_str(":Status", Colors::Gold)
    );
    assert_eq!(
        Status::Task.as_colored_str(),
        color_str(":Task", Colors::Gold)
    );
    assert_eq!(
        Status::Note.as_colored_str(),
        color_str(":!Note", Colors::Yellow)
    );
    assert_eq!(
        Status::Select.as_colored_str(),
        color_str(":?Select", Colors::Yellow)
    );
    assert_eq!(
        Status::Option.as_colored_str(),
        color_str(":?Option", Colors::Yellow)
    );
    assert_eq!(
        Status::Quote.as_colored_str(),
        color_str(":>Quote", Colors::Gray)
    );
    assert_eq!(
        Status::Question.as_colored_str(),
        color_str("?Question", Colors::Purple)
    );
}

#[cfg(feature = "types-v2")]
#[test]
fn status_plain_str_alias() {
    for s in ALL_STATUSES {
        assert_eq!(s.as_plain_str(), s.as_str());
    }
}

#[cfg(feature = "types-v2")]
#[test]
fn status_display_matches_as_str() {
    for s in ALL_STATUSES {
        assert_eq!(s.to_string(), s.as_str());
    }
}

#[cfg(feature = "types-v2")]
#[test]
fn status_eq_hash() {
    use std::collections::HashSet;
    let mut set: HashSet<Status> = HashSet::new();
    for s in ALL_STATUSES {
        set.insert(*s);
        set.insert(*s);
    }
    assert_eq!(set.len(), ALL_STATUSES.len());
}

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
#[allow(deprecated)]
fn deoption_some_sets_found_true() {
    let mut found = false;
    let v = musutils::types::deoption(Some(42), || 0, &mut found);
    assert_eq!(v, 42);
    assert!(found);
}

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
#[allow(deprecated)]
fn deoption_none_uses_default_and_sets_found_false() {
    let mut found = true;
    let v = musutils::types::deoption::<i32, _>(None, || 7, &mut found);
    assert_eq!(v, 7);
    assert!(!found);
}

#[cfg(any(feature = "types-v1", feature = "types-v2"))]
#[test]
#[allow(deprecated)]
fn deoption_default_fn_not_called_on_some() {
    let mut called = false;
    let mut found = false;
    let v = musutils::types::deoption(
        Some("x"),
        || {
            called = true;
            "y"
        },
        &mut found,
    );
    assert_eq!(v, "x");
    assert!(found);
    assert!(!called);
}

#[cfg(feature = "types-v1")]
#[test]
fn v1_draw_repeats() {
    use musutils::types::line::draw;
    assert_eq!(draw('-', 0), "");
    assert_eq!(draw('-', 1), "-");
    assert_eq!(draw('x', 5), "xxxxx");
    assert_eq!(draw('🦀', 3), "🦀🦀🦀");
}

#[cfg(feature = "types-v1")]
#[test]
fn v1_draw_colored_wraps() {
    use musutils::types::line::draw_colored;
    let s = draw_colored('*', 3, Colors::Red);
    assert_eq!(s, color_str("***", Colors::Red));
    let e = draw_colored('-', 0, Colors::Green);
    assert_eq!(e, color_str("", Colors::Green));
}

#[cfg(feature = "types-v2")]
#[test]
fn v2_draw_repeats() {
    use musutils::types::draw;
    use musutils::types::line;
    assert_eq!(draw('-', 0), "");
    assert_eq!(draw('x', 5), "xxxxx");
    assert_eq!(line::draw('a', 3), "aaa");
    assert_eq!(draw('🦀', 2), "🦀🦀");
}

#[cfg(feature = "types-v2")]
#[test]
fn v2_draw_colored_wraps() {
    use musutils::types::draw_colored;
    use musutils::types::line;
    let s = draw_colored('*', 3, Colors::Red);
    assert_eq!(s, color_str("***", Colors::Red));
    let s2 = line::draw_colored('#', 2, Colors::Blue);
    assert_eq!(s2, color_str("##", Colors::Blue));
}
