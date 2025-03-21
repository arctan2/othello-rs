use std::{
    cell::{Ref, RefCell, RefMut},
    fmt,
    io::Write,
    rc::{Rc, Weak},
};

use crossterm::style::Color;

use crate::sleep;

use super::{
    buffer::{Buffer, Cell, Rect},
    crossterm_handler::CrosstermHandler,
    elements::{Element, InputWindow, Text},
};

pub struct Window {
    buffer: Buffer,
    sub_windows: Vec<WindowRef>,
    parent: Weak<RefCell<Window>>,
    id: usize,
}

type WinRef = Rc<RefCell<Window>>;

impl fmt::Debug for Window {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Window {{\n  parent: {}, \n  id: {}, \n  buffer: {:#?}, \n  children: {}\n}}\n",
            match self.parent.upgrade() {
                None => "None",
                _ => "<WindowRef>",
            },
            self.id,
            self.buffer(),
            self.children().len()
        )
    }
}

#[derive(Debug)]
pub enum Position {
    CenterH,
    CenterV,
    CenterB,
    Coord(u32, u32),
}

impl Window {
    pub fn default() -> Self {
        Window::new(0, 0, 0, 0)
    }

    pub fn new(width: u32, height: u32, x: u32, y: u32) -> Self {
        Window {
            buffer: Buffer::empty(
                Rect::new(x, y, width, height),
                Rect::new(0, 0, width, height),
            ),
            sub_windows: vec![],
            parent: Weak::new(),
            id: 0,
        }
    }

    pub fn set_pos(&mut self, pos: Position) {
        match pos {
            Position::Coord(x, y) => self.buffer.set_pos(x, y),
            _ => match self.parent.upgrade() {
                Some(parent) => {
                    let self_rect = self.buffer.rect();
                    let (x, y) = parent
                        .borrow()
                        .buffer
                        .rect()
                        .get_center_start_pos(self.buffer.rect());

                    match pos {
                        Position::CenterH => self.buffer.set_pos(x, self_rect.y),
                        Position::CenterV => self.buffer.set_pos(self_rect.x, y),
                        Position::CenterB => self.buffer.set_pos(x, y),
                        _ => (),
                    }
                }
                None => (),
            },
        }
    }

    pub fn extend_scroll_height(&mut self, dy: u32) {
        self.buffer.extend_scroll_height(dy);
    }

    pub fn scoll_size(mut self, width: u32, height: u32) -> Self {
        self.set_scroll_size(width, height);
        self
    }

    pub fn set_scroll_size(&mut self, width: u32, height: u32) {
        self.buffer.set_scroll_size(width, height);
    }

    pub fn set_scroll_xy(&mut self, x: u32, y: u32) {
        self.buffer.set_scroll_xy(x, y);
    }

    pub fn set_scroll_xy_rel(&mut self, dx: i16, dy: i16) {
        let mut x = self.buffer.scroll().x as i16 + dx;
        let mut y = self.buffer.scroll().y as i16 + dy;

        if x < 0 {
            x = 0;
        }
        if y < 0 {
            y = 0;
        }

        self.set_scroll_xy(x as u32, y as u32);
    }

    pub fn scroll(&self) -> Rect {
        self.buffer.scroll()
    }

    pub fn width(&self) -> u32 {
        self.buffer.width()
    }

    pub fn height(&self) -> u32 {
        self.buffer.height()
    }

    pub fn xy(mut self, x: u32, y: u32) -> Self {
        self.set_pos(Position::Coord(x, y));
        self
    }

    pub fn position(mut self, pos: Position) -> Self {
        self.set_pos(pos);
        self
    }

    pub fn parent(mut self, parent: WindowRef) -> Self {
        self.parent = Rc::downgrade(&parent.0);
        self
    }

    pub fn bg(mut self, bg: Color) -> Self {
        self.buffer.set_bg(bg);
        self
    }

    pub fn get_parent(&self) -> Weak<RefCell<Window>> {
        self.parent.clone()
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.set_size(width, height);
        self
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        let mut c = Cell::default();
        c.bg = self.buffer.get_bg();
        let r = Rect::new(self.buffer.left(), self.buffer.top(), width, height);
        let sr = Rect::new(0, 0, width, height);
        self.buffer = Buffer::filled(r, sr, c);
    }

    pub fn draw_element(&mut self, el: &dyn Element) {
        el.draw(&mut self.buffer);
    }

    pub fn draw_text(&mut self, text: &str, pos: Position) {
        let mut t = Text::default().text(text);
        let r = Rect::new(0, 0, self.width(), self.height());
        t.set_position(r, pos);
        self.draw_element(&t);
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.buffer.set_bg(bg);
    }

    pub fn clear(&mut self) {
        self.buffer.reset();
    }

    fn children(&self) -> Vec<WindowRef> {
        self.sub_windows.clone()
    }

    fn delete_child_by_id(&mut self, id: usize) {
        self.sub_windows.remove(id);
    }

    pub fn top(&self) -> u32 {
        self.buffer.top()
    }

    pub fn left(&self) -> u32 {
        self.buffer.left()
    }

    pub fn bottom(&self) -> u32 {
        self.buffer.bottom()
    }

    pub fn right(&self) -> u32 {
        self.buffer.right()
    }

    pub fn abs_pos(&self) -> (u32, u32) {
        let mut top = self.buffer.top();
        let mut left = self.buffer.left();

        let mut parent = self.get_parent();

        loop {
            match parent.upgrade() {
                Some(p) => {
                    let p = p.borrow();
                    top += p.top();
                    left += p.left();
                    parent = p.get_parent();
                }
                None => return (top, left),
            }
        }
    }

    pub fn rel_pos(&self) -> (u32, u32) {
        (self.buffer.top(), self.buffer.left())
    }

    fn render_window_at(&mut self, buf: &Buffer, top: u32, left: u32) {
        let (maxx, maxy) = (self.buffer.right(), self.buffer.bottom());
        let mut endx = buf.width() as i16;
        let mut endy = buf.height() as i16;

        if left + buf.width() > maxx {
            endx -= ((left + buf.width()) - maxx) as i16;
        }
        if top + buf.height() > maxy {
            endy -= ((top + buf.height()) - maxy) as i16;
        }

        if endx < 0 || endy < 0 {
            return;
        }

        for y in 0..(endy as u32) {
            for x in 0..(endx as u32) {
                let a = self.buffer.get_mut(x + left, y + top);
                let b = buf.get(x, y);

                a.bg = b.bg;
                a.fg = b.fg;
                a.attr = b.attr;
                a.symbol = b.symbol.clone();
            }
        }
    }

    pub fn render_window(&mut self, win: &Window) {
        let (top, left) = win.rel_pos();
        self.render_window_at(&win.buffer, top, left);
    }

    pub fn render_to_parent(&mut self) {
        match self.parent.upgrade() {
            Some(parent) => parent.borrow_mut().render_window(&self),
            None => panic!("cannot call window.render_to_parent() on root window."),
        }
    }

    pub fn render(&mut self) {
        let mut parent = self.parent.clone();
        let buf = &self.buffer;
        let mut top = buf.top();
        let mut left = buf.left();

        loop {
            match parent.upgrade() {
                Some(p) => {
                    let mut p = p.borrow_mut();
                    parent = p.get_parent();
                    if parent.upgrade().is_none() {
                        p.render_window_at(&buf, top, left);
                        return;
                    }
                    top += p.top();
                    left += p.left();
                }
                None => panic!(
                    "cannot call window.render() on root window, instead call terminal.flush()."
                ),
            }
        }
    }

    pub fn is_root(&self) -> bool {
        match self.parent.upgrade() {
            Some(_) => false,
            None => true,
        }
    }

    // can be optimized
    pub fn render_children(&mut self) {
        for child in &mut self.sub_windows {
            child.render();
        }
    }

    // can be optimized
    pub fn render_deep(&mut self) {
        for child in &mut self.sub_windows {
            child.render();
            child.render_deep();
        }
    }

    pub fn render_element(&mut self, el: &dyn Element) {
        self.draw_element(el);
        self.render();
    }
}

#[derive(Clone, Debug)]
pub struct WindowRef(WinRef);

impl WindowRef {
    pub fn new(width: u32, height: u32, x: u32, y: u32) -> Self {
        WindowRef(Rc::new(RefCell::new(Window {
            buffer: Buffer::empty(
                Rect::new(x, y, width, height),
                Rect::new(x, y, width, height),
            ),
            sub_windows: vec![],
            parent: Weak::new(),
            id: 0,
        })))
    }

    pub fn from_window(win: Window) -> Self {
        WindowRef(Rc::new(RefCell::new(win)))
    }

    pub fn id(&self) -> usize {
        self.inner().id
    }

    pub fn new_child(&mut self, mut win: Window) -> Self {
        win.id = self.0.borrow_mut().sub_windows.len();
        let child = WindowRef(Rc::new(RefCell::new(win.parent(self.clone()))));

        self.0.borrow_mut().sub_windows.push(child.clone());
        child
    }

    pub fn inner_mut(&mut self) -> RefMut<'_, Window> {
        self.0.borrow_mut()
    }

    pub fn inner(&self) -> Ref<'_, Window> {
        self.0.borrow()
    }

    pub fn xy_rel(mut self, dx: i16, dy: i16) -> Self {
        self.set_xy_rel(dx, dy);
        self
    }

    pub fn set_position(&mut self, pos: Position) {
        self.inner_mut().set_pos(pos);
    }

    pub fn set_xy_rel(&mut self, mut dx: i16, mut dy: i16) {
        let (x, y) = self.inner().buffer.rect().get_xy();
        dx += x as i16;
        dy += y as i16;
        if dx < 0 {
            dx = 0;
        }
        if dy < 0 {
            dy = 0;
        }
        self.set_position(Position::Coord(dx as u32, dy as u32));
    }

    pub fn draw_element(&mut self, el: &dyn Element) {
        self.inner_mut().draw_element(el);
    }

    pub fn clear(&mut self) {
        self.inner_mut().clear();
    }

    pub fn width(&self) -> u32 {
        self.inner().buffer.width()
    }

    pub fn height(&self) -> u32 {
        self.inner().buffer.height()
    }

    pub fn top(&self) -> u32 {
        self.inner().top()
    }

    pub fn left(&self) -> u32 {
        self.inner().left()
    }

    pub fn bottom(&self) -> u32 {
        self.inner().bottom()
    }

    pub fn right(&self) -> u32 {
        self.inner().right()
    }

    pub fn set_scroll_size(&mut self, width: u32, height: u32) {
        self.inner_mut().set_scroll_size(width, height);
    }

    pub fn set_scroll_xy(&mut self, x: u32, y: u32) {
        self.inner_mut().set_scroll_xy(x, y);
    }

    pub fn set_scroll_xy_rel(&mut self, dx: i16, dy: i16) {
        self.inner_mut().set_scroll_xy_rel(dx, dy);
    }

    pub fn scroll(&self) -> Rect {
        self.inner().scroll()
    }

    pub fn parent(&self) -> Weak<RefCell<Window>> {
        self.inner().get_parent()
    }

    pub fn abs_pos(&self) -> (u32, u32) {
        self.inner().abs_pos()
    }

    pub fn extend_scroll_height(&mut self, dy: u32) {
        self.inner_mut().extend_scroll_height(dy);
    }

    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.inner_mut().set_size(width, height);
        self
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.inner_mut().set_size(width, height);
    }

    pub fn set_bg(&mut self, bg: Color) {
        self.inner_mut().set_bg(bg);
    }

    pub fn rect(&self) -> Rect {
        self.inner().buffer.rect()
    }

    pub fn delete(&mut self) {
        let id = self.id();
        match self.parent().upgrade() {
            Some(p) => {
                let mut p = p.borrow_mut();
                p.parent = Weak::new();
                p.delete_child_by_id(id);
            }
            None => (),
        }
    }

    fn render_window_at(&mut self, buf: &Buffer, top: u32, left: u32) {
        self.inner_mut().render_window_at(buf, top, left);
    }

    pub fn render_window(&mut self, win: &WindowRef) {
        self.inner_mut().render_window(&win.inner());
    }

    pub fn render_to_parent(&mut self) {
        self.inner_mut().render_to_parent();
    }

    pub fn is_root(&self) -> bool {
        self.inner().is_root()
    }

    pub fn render(&mut self) {
        self.inner_mut().render();
    }

    pub fn render_children(&mut self) {
        self.inner_mut().render_children();
    }

    pub fn render_deep(&mut self) {
        self.inner_mut().render_deep();
    }

    pub fn render_element(&mut self, el: &dyn Element) {
        self.inner_mut().render_element(el);
    }

    pub fn draw_text(&mut self, text: &str, pos: Position) {
        self.inner_mut().draw_text(text, pos);
    }
}

macro_rules! draw_elements {
  ($win:ident,$($el:expr),+) => {
    $($win.draw_element(&$el);)+
  };
}

pub(crate) use draw_elements;
