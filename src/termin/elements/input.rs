use std::io::{stdout, Write};

use copypasta::ClipboardProvider;
use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
};

use crate::{
    sleep,
    termin::{
        crossterm_handler::CrosstermHandler,
        window::{Window, WindowRef},
    },
};

use super::Text;

#[derive(Debug)]
pub struct InputWindow {
    start_text: (u32, u32),
    max_len: i32,
    cursor_pos: usize,
    input_win: WindowRef,
    input_box: Text,
    abs_x: u32,
    abs_y: u32,
    rel_x: u32,
    rel_y: u32,
}

pub enum EventResult {
    Continue,
    Return(String),
}

impl InputWindow {
    pub fn from(parent: &mut WindowRef, win: Window) -> Self {
        let input_win = parent.new_child(win);
        let input_box = Text::default().size(input_win.width(), input_win.height());

        let (abs_y, abs_x) = input_win.abs_pos();

        let mut iw = InputWindow {
            start_text: (0, 0),
            max_len: 0,
            input_box,
            cursor_pos: 0,
            input_win,
            abs_x,
            abs_y,
            rel_x: 0,
            rel_y: 0,
        };
        iw.update_rel_xy();

        return iw;
    }

    pub fn clear(&mut self) {
        self.input_win.clear(); 
        self.input_box.set_text("");
        self.cursor_pos = 0;
        self.update_rel_xy();
    }

    pub fn show_cursor(&self) {
        execute!(stdout(), cursor::Show).unwrap();
    }

    pub fn get_text_clone(&self) -> String {
        self.get_text().into()
    }

    pub fn get_text(&self) -> &str {
        self.input_box.get_text()
    }

    pub fn hide_cursor(&self) {
        execute!(stdout(), cursor::Hide).unwrap();
    }

    pub fn start_text(mut self, start_text: (u32, u32)) -> Self {
        self.start_text = start_text;
        self
    }

    pub fn update_rel_xy(&mut self) {
        let width = self.input_win.width();
        let idx = self.cursor_pos as u32;
        self.rel_x = idx % width;
        self.rel_y = idx / width;
    }

    pub fn max_len(mut self, max_len: i32) -> Self {
        self.max_len = max_len;
        self
    }

    pub fn handle_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Paste(s) => {
                let len = s.len();
                self.input_box.push_string(s);
                if self.cursor_pos + len > self.max_len as usize {
                    self.input_box.chop_after(self.max_len as usize);
                    self.cursor_pos = self.max_len as usize;
                } else {
                    self.cursor_pos += len as usize;
                }
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('v'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            }) => {
                let s = copypasta::ClipboardContext::new()
                    .unwrap()
                    .get_contents()
                    .unwrap();
                let len = s.len();
                self.input_box.push_string(s);
                if self.cursor_pos + len as usize > self.max_len as usize {
                    self.input_box.chop_after(self.max_len as usize);
                    self.cursor_pos = self.max_len as usize;
                } else {
                    self.cursor_pos += len as usize;
                }
            }
            Event::Key(k) => match k.code {
                KeyCode::Esc => {
                    self.hide_cursor();
                    self.input_win.delete();
                    return EventResult::Return(self.input_box.get_text().to_string());
                }
                KeyCode::Enter => {
                    self.hide_cursor();
                    self.input_win.delete();
                    return EventResult::Return(self.input_box.get_text().to_string());
                }
                KeyCode::Backspace => {
                    if self.cursor_pos > 0 {
                        self.cursor_pos -= 1;
                        self.input_box.remove_char_at(self.cursor_pos);
                    }
                }
                KeyCode::Char(ch) => {
                    if (self.input_box.get_text().len() as i32) < self.max_len {
                        self.input_box.add_char_at(self.cursor_pos, ch);
                        self.cursor_pos += 1;
                    }
                }
                KeyCode::Left => {
                    self.cursor_pos -= if self.cursor_pos != 0 { 1 } else { 0 };
                }
                KeyCode::Right => {
                    self.cursor_pos += if self.cursor_pos == self.input_box.get_text().len() {
                        0
                    } else {
                        1
                    };
                }
                _ => (),
            },
            _ => (),
        }

        self.update_rel_xy();
        return EventResult::Continue;
    }

    pub fn update_cursor(&self) {
        queue!(
            stdout(),
            cursor::MoveTo((self.abs_x + self.rel_x) as u16, (self.abs_y + self.rel_y) as u16)
        )
        .unwrap();
    }

    pub fn render(&mut self) {
        self.input_win.clear();
        self.input_win.draw_element(&self.input_box);
    }

    pub fn render_to_parent(&mut self) {
        self.input_win.clear();
        self.input_win.draw_element(&self.input_box);
        self.input_win.render_to_parent();
    }

    pub fn input_win(&self) -> &WindowRef {
        &self.input_win
    }

    pub fn set_abs_xy(&mut self) {
        let (abs_y, abs_x) = self.input_win.abs_pos();
        self.abs_x = abs_x;
        self.abs_y = abs_y;
    }

    pub fn read_string<W: Write>(&mut self, handler: &mut CrosstermHandler<W>) -> String {
        self.set_abs_xy();
        self.show_cursor();
        self.update_rel_xy();
        self.update_cursor();
        handler.flush().unwrap();

        loop {
            match self.handle_event(handler.event()) {
                EventResult::Return(s) => return s,
                _ => (),
            }
            self.render();
            handler.draw_window(self.input_win()).unwrap();

            self.update_cursor();
            handler.flush().unwrap();
        }
    }
}
