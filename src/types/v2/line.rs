use crate::color::{Colors, color_str};

pub fn draw(c: char, count: usize) -> String {
    c.to_string().repeat(count)
}

pub fn draw_colored(c: char, count: usize, color: Colors) -> String {
    color_str(&draw(c, count), color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_repeats() {
        assert_eq!(draw('-', 5), "-----");
    }

    #[test]
    fn draw_colored_wraps() {
        let s = draw_colored('*', 3, Colors::Red);
        assert!(s.contains("***"));
    }
}
