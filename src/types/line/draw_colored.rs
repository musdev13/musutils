use crate::types::line;

pub fn draw_colored(c: char, count:usize, color: crate::color::Colors ) -> String{
    crate::color::color_str(&line::draw(c, count), color)
}
