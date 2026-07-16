use crate::color::Colors;

pub fn color_str(text: &str, color: Colors) -> String {
    format!("{}{}{}", color.as_term_code(), text, Colors::Reset.as_term_code())
}
