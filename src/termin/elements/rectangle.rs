use super::{impl_setters, Buffer, Element, Position, Rect};
use crossterm::style::Color;

#[derive(Debug)]
pub struct Rectangle {
    rect: Rect,
    bg: Color,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Rectangle {
            rect: Rect::new(x, y, width, height),
            bg: Color::Reset,
        }
    }

    pub fn from_rect(rect: Rect) -> Self {
        let mut r = Self::default();
        r.rect = rect;
        r
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = bg;
        self
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }
}

impl_setters!(Rectangle);

impl Default for Rectangle {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Element for Rectangle {
    fn draw(&self, buf: &mut Buffer) {
        for y in 0..self.rect.height {
            for x in 0..self.rect.width {
                let c = buf.get_vir_mut(self.rect.x + x, self.rect.y + y);
                c.set_bg(self.bg);
            }
        }
    }
}
