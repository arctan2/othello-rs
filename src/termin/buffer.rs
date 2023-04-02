use std::{fmt, vec};

use crossterm::style::{Attribute, Color};

use crate::sleep;

#[derive(Clone, Debug)]
pub struct Cell {
    pub bg: Color,
    pub fg: Color,
    pub attr: Attribute,
    pub symbol: String,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            bg: Color::Reset,
            fg: Color::Reset,
            attr: Attribute::Reset,
            symbol: " ".into(),
        }
    }
}

impl Cell {
    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
    }

    pub fn set_fg(&mut self, fg: Color) {
        self.fg = fg;
    }

    pub fn set_symbol(&mut self, sym: char) {
        self.symbol = sym.to_string();
    }

    pub fn set_attr(&mut self, attr: Attribute) {
        self.attr = attr;
    }

    pub fn reset(&mut self) {
        self.bg = Color::Reset;
        self.fg = Color::Reset;
        self.symbol = String::from(" ");
        self.attr = Attribute::Reset;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn get_xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    pub fn get_center_start_pos(&self, rect: Rect) -> (u32, u32) {
        let h = (self.width / 2) - (rect.width / 2);
        let v = (self.height / 2) - (rect.height / 2);
        (h, v)
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

pub struct Buffer {
    bg: Color,
    rect: Rect,
    scroll: Rect,
    content: Vec<Cell>,
}

impl fmt::Debug for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Buffer{{rect: {:?}, content: Vec<Cell, {}>}}",
            self.rect,
            self.content.len()
        )
    }
}

impl Buffer {
    pub fn empty(rect: Rect, scroll: Rect) -> Self {
        let a = scroll.area() as usize;
        Buffer {
            rect,
            scroll,
            content: vec![Cell::default(); a],
            bg: Color::Reset,
        }
    }

    pub fn filled(rect: Rect, scroll: Rect, fill: Cell) -> Buffer {
        let a = scroll.area() as usize;
        Buffer {
            rect,
            scroll,
            content: vec![fill.clone(); a],
            bg: fill.bg,
        }
    }

    fn resize_content(&mut self) {
        let area = self.scroll.area() as usize;
        let mut cell = self.content[0].clone();
        cell.bg = self.bg;
        cell.symbol = " ".to_string();
        self.content = vec![cell; area];
    }

    pub fn set_scroll_size(&mut self, width: u32, height: u32) {
        self.scroll.width = width;
        self.scroll.height = height;
        self.resize_content();
    }

    pub fn extend_scroll_height(&mut self, dy: u32) {
        self.scroll.height += dy;

        let area = self.scroll.area() as usize;
        let mut cell = self.content[0].clone();
        cell.bg = self.bg;
        cell.symbol = " ".to_string();
        self.content.append(&mut vec![cell; area]);
    }

    pub fn set_scroll_xy(&mut self, x: u32, y: u32) {
        if x + self.rect.width <= self.scroll.width {
            self.scroll.x = x;
        }
        if y + self.rect.height <= self.scroll.height {
            self.scroll.y = y;
        }
    }

    pub fn content_mut(&mut self) -> &mut Vec<Cell> {
        &mut self.content
    }

    pub fn content(&self) -> &Vec<Cell> {
        &self.content
    }

    pub fn reset(&mut self) {
        for c in &mut self.content {
            c.reset();
            c.set_bg(self.bg);
        }
    }

    pub fn width(&self) -> u32 {
        self.rect.width
    }

    pub fn height(&self) -> u32 {
        self.rect.height
    }

    pub fn set_pos(&mut self, x: u32, y: u32) {
        self.rect.x = x;
        self.rect.y = y;
    }

    pub fn size(&self) -> usize {
        self.content.len()
    }

    pub fn vir_index_of(&self, x: u32, y: u32) -> usize {
        ((self.scroll.width * y) + x) as usize
    }

    pub fn get_vir(&self, x: u32, y: u32) -> &Cell {
        let idx = self.vir_index_of(x, y);
        &self.content[idx]
    }

    pub fn get_vir_mut(&mut self, x: u32, y: u32) -> &mut Cell {
        let idx = self.vir_index_of(x, y);

        &mut self.content[idx]
    }

    pub fn index_of(&self, mut x: u32, mut y: u32) -> usize {
        y += self.scroll.y;
        x += self.scroll.x;
        ((self.scroll.width * y) + x) as usize
    }

    pub fn get(&self, x: u32, y: u32) -> &Cell {
        let idx = self.index_of(x, y);
        &self.content[idx]
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> &mut Cell {
        let idx = self.index_of(x, y);

        &mut self.content[idx]
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.bg = bg;
        for c in &mut self.content {
            c.set_bg(bg);
        }
    }

    pub fn get_bg(&self) -> Color {
        self.bg
    }

    pub fn top(&self) -> u32 {
        self.rect.y
    }

    pub fn left(&self) -> u32 {
        self.rect.x
    }

    pub fn bottom(&self) -> u32 {
        self.rect.y + self.rect.height
    }

    pub fn right(&self) -> u32 {
        self.rect.x + self.rect.width
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    pub fn scroll(&self) -> Rect {
        self.scroll
    }

    pub fn to_vec(&self, abs: (u32, u32)) -> Vec<(u16, u16, &Cell)> {
        let mut result: Vec<(u16, u16, &Cell)> = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                result.push(((x + abs.1) as u16, (y + abs.0) as u16, self.get(x, y)));
            }
        }
        result
    }
}
