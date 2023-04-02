use super::{impl_setters, Buffer, Element, Position, Rect};
use crossterm::style::{Attribute, Color};

#[derive(Debug)]
pub struct Text {
    rect: Rect,
    fg: Color,
    attr: Attribute,
    text: String,
    start_text: (u32, u32),
}

#[allow(dead_code)]
impl Text {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            rect: Rect::new(x, y, width, height),
            fg: Color::Reset,
            attr: Attribute::Reset,
            text: String::from(""),
            start_text: (0, 0),
        }
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn fg(mut self, fg: Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn set_fg(&mut self, fg: Color) {
        self.fg = fg;
    }

    pub fn text(mut self, text: &str) -> Self {
        self.set_text(text);
        self
    }

    pub fn start_text(mut self, start_text: (u32, u32)) -> Self {
        self.start_text = start_text;
        self
    }

    pub fn set_start_text(&mut self, start_text: (u32, u32)) {
        self.start_text = start_text
    }

    pub fn width_fit(&mut self) {
        self.rect.width = self.text.len() as u32;
        if self.rect.height == 0 {
            self.rect.height = 1;
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        if self.rect.width == 0 {
            self.rect.width = text.len() as u32;
        }
        if self.rect.height == 0 {
            self.rect.height = 1;
        }
    }

    pub fn push_string(&mut self, s: String) {
        self.text.push_str(&s);
    }

    pub fn chop_after(&mut self, idx: usize) {
        _ = self.text.split_off(idx);
    }

    pub fn add_char_at(&mut self, idx: usize, ch: char) {
        self.text.insert(idx, ch)
    }

    pub fn remove_char_at(&mut self, idx: usize) {
        if self.text.len() == 0 {
            return;
        }
        self.text.remove(idx);
    }

    pub fn attr(mut self, attr: Attribute) -> Self {
        self.set_attr(attr);
        self
    }

    pub fn set_attr(&mut self, attr: Attribute) {
        self.attr = attr;
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

impl_setters!(Text);

impl Default for Text {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Element for Text {
    fn draw(&self, buf: &mut Buffer) {
        let mut text = self.text.chars();
        let (start_x, start_y) = self.start_text;

        enum Char {
            NewLine,
            Other,
        }

        let mut set_cell = |x, y| -> Char {
            let c = buf.get_vir_mut(x, y);
            c.set_fg(self.fg);

            match text.next() {
                Some(sym) => {
                    if sym == '\n' {
                        return Char::NewLine;
                    }
                    c.set_symbol(sym);
                    c.set_attr(self.attr);
                    Char::Other
                }
                None => Char::Other,
            }
        };

        for x in start_x..self.rect.width {
            match set_cell(self.rect.x + x, self.rect.y) {
                Char::NewLine => break,
                _ => continue,
            }
        }

        for y in start_y + 1..self.rect.height {
            for x in 0..self.rect.width {
                match set_cell(self.rect.x + x, self.rect.y + y) {
                    Char::NewLine => break,
                    _ => continue,
                }
            }
        }
    }
}
