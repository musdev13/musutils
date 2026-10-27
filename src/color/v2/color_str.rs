use super::Colors;
use super::support::color_supported;

pub fn color_str(text: &str, color: Colors) -> String {
    format!(
        "{}{}{}",
        color.as_term_code(),
        text,
        Colors::Reset.as_term_code()
    )
}

pub fn color_str_auto(text: &str, color: Colors) -> String {
    if color_supported() {
        color_str(text, color)
    } else {
        text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_with_reset() {
        let s = color_str("x", Colors::Red);
        assert!(s.starts_with("\x1b[31m"));
        assert!(s.ends_with("\x1b[0m"));
        assert!(s.contains('x'));
    }
}
