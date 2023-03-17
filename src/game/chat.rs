use crossterm::{event::Event, style::Color};

use crate::{termin::{window::{WindowRef, Window, Position}, terminal_window::TerminalHandler, elements::{InputWindow, Text}}, sleep};

#[derive(Debug)]
pub struct ChatMsg <'a> {
  msg: &'a str,
  name: &'a str,
  name_fg: Color
}

impl <'a> ChatMsg<'a> {
  pub fn new(name: &'a str, msg: &'a str, name_fg: Color) -> Self {
    Self{name, msg, name_fg}
  }
}

#[derive(Debug)]
pub struct ChatSection {
  pub chat_section: WindowRef,
  pub chat_msgs: WindowRef,
  pub recent_chat: WindowRef,
  pub input_win: InputWindow,
  next_y_pos: u32
}

pub enum ChatSectionEvent {
  ExitChatMode,
  EmitString(String),
  DrawInputBox,
  DrawChatMsgs
}

impl ChatSection {
  pub fn new(win: &mut WindowRef) -> Self {
    let mut chat_section = win.new_child(Window::default().size(50, 20));
    let chat_msgs = chat_section.new_child(Window::default().size(50, 18).scoll_size(50, 100));
    let recent_chat = win.new_child(Window::default().size(50, 1));
    let input_win = InputWindow::from(&mut chat_section, Window::default().size(48, 1).xy(2, 19)).max_len(47);

    chat_section.draw_text(">", Position::Coord(0, 19));

    Self {
      chat_section,
      recent_chat,
      chat_msgs,
      input_win,
      next_y_pos: 0
    }
  }

  pub fn render(&mut self) {
    self.chat_msgs.render_to_parent();
    self.input_win.render_to_parent();
    self.chat_section.render();
  }

  pub fn push_chat_msg(&mut self, msg: ChatMsg) {
    let msg_text = ": ".to_string() + msg.msg;
    let msg_height = ((msg.name.len() + msg_text.len()) / self.chat_msgs.width() as usize + 1) as u16;

    if self.next_y_pos + msg_height as u32 > self.chat_msgs.height() as u32 {
      self.chat_msgs.extend_scroll_height(50);
    }

    let name = Text::default()
                .text(msg.name)
                .size(msg.name.len() as u16, 1)
                .xy(0, self.next_y_pos as u16)
                .fg(msg.name_fg);
    let msg = Text::default().text(&msg_text)
                .start_text((msg.name.len() as u16, 0))
                .size(self.chat_msgs.width(), msg_height)
                .xy(0, self.next_y_pos as u16);

    self.chat_msgs.draw_element(&name);
    self.chat_msgs.draw_element(&msg);

    self.next_y_pos += msg_height as u32 + 1;
  }

  pub fn scroll_up(&mut self) {
    self.chat_msgs.set_scroll_xy_rel(0, -1);
  }

  pub fn scroll_down(&mut self) {
    self.chat_msgs.set_scroll_xy_rel(0, 1);
  }

  pub fn enable_cursor(&mut self) {
    self.input_win.set_abs_xy();
    self.input_win.show_cursor();
    self.input_win.update_rel_xy();
    self.input_win.update_cursor();
  }

  pub fn disable_cursor(&mut self) {
    self.input_win.hide_cursor();
  }

  pub fn handle_kbd(&mut self, e: Event) {
    self.input_win.handle_event(e);
  }
}

#[cfg(test)]
mod tests {
  use std::io::stdout;
  use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode}, execute, cursor, event::KeyCode};
  use crate::termin::{self, crossterm_handler::CrosstermHandler};

  use super::*;

  #[test]
  fn chat_section() {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();
    let mut terminal = termin::root(CrosstermHandler::new(stdout()));

    let mut chat_sec = ChatSection::new(&mut terminal.root);
    chat_sec.render();
    terminal.refresh().unwrap();

    chat_sec.enable_cursor();
    terminal.flush().unwrap();

    loop {
      let e = terminal.event();
      chat_sec.input_win.hide_cursor();
      match e {
        Event::Key(k) => match k.code {
          KeyCode::Enter => {
            chat_sec.push_chat_msg(ChatMsg::new("hehe", "obobobobobobobob", Color::Red));
            terminal.draw_window(&chat_sec.chat_msgs).unwrap();
          },
          KeyCode::Down => {
            chat_sec.scroll_down();
            terminal.draw_window(&chat_sec.chat_msgs).unwrap();
          },
          KeyCode::Up => {
            chat_sec.scroll_up();
            terminal.draw_window(&chat_sec.chat_msgs).unwrap();
          },
          KeyCode::Esc => break,
          _ => {
            chat_sec.handle_kbd(e);
            chat_sec.input_win.render();
            terminal.handler.draw_window(chat_sec.input_win.input_win()).unwrap();

          }
        },
        _ => continue
      }
      chat_sec.input_win.show_cursor();
      chat_sec.input_win.update_cursor();
      terminal.flush().unwrap();
    }

    execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
  }
}
