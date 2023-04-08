use crossterm::{event::Event, style::Color, queue, execute, cursor::MoveTo};

use crate::{
    sleep,
    termin::{
        elements::{InputWindow, Text, Rectangle},
        terminal_window::TerminalHandler,
        window::{Position, Window, WindowRef},
    },
};

#[derive(Debug)]
pub struct ChatMsg<'a> {
    msg: &'a str,
    name: &'a str,
    name_fg: Color,
}

impl<'a> ChatMsg<'a> {
    pub fn new(name: &'a str, msg: &'a str, name_fg: Color) -> Self {
        Self { name, msg, name_fg }
    }
}

#[derive(Debug)]
pub struct ChatSection {
    pub chat_section: WindowRef,
    pub chat_msgs: WindowRef,
    pub recent_chat: WindowRef,
    pub input_win: InputWindow,
    is_online: bool,
    receiver_name: String,
    next_y_pos: u32,
}

enum MsgDir<'a> {
    Send(&'a str),
    Receive(&'a str)
}

impl<'a> MsgDir<'a> {
    fn to_chat_msg(&self, name: &'a str) -> ChatMsg {
        return match &self {
            MsgDir::Send(msg) => ChatMsg::new(name, msg, Color::Blue),
            MsgDir::Receive(msg) => ChatMsg::new(name, msg, Color::Red)
        }
    }
}

impl ChatSection {
    pub fn new(win: &mut WindowRef, receiver_name: String) -> Self {
        let height = 22;
        let width = 50;
        let mut chat_section = win.new_child(Window::default().size(width, height).xy(0, 2));
        let chat_msgs = chat_section.new_child(Window::default().size(width, height - 4).xy(0, 2).scoll_size(width, 100));
        let recent_chat = win.new_child(Window::default().size(width, 2).xy(1, 25));
        let input_win = InputWindow::from(
            &mut chat_section,
            Window::default().size(width - 2, 1).xy(2, height - 1)
        )
        .max_len((width - 3) as i32);

        chat_section.draw_text(">", Position::Coord(0, height - 1));
        
        let mut divider = String::new();
        for _ in 0..chat_section.width() {
            divider.push('_');
        }

        chat_section.draw_element(&Text::default().xy(3, 0).text(&receiver_name));
        chat_section.draw_element(&Text::default().text(&divider).xy(0, 1));

        Self {
            chat_section,
            recent_chat,
            chat_msgs,
            input_win,
            next_y_pos: 0,
            receiver_name,
            is_online: false
        }
    }

    fn draw_conn_status(&mut self, color: Color) {
        self.chat_section.draw_element(&Rectangle::default().xy(0, 0).size(2, 1).bg(color));
    }

    pub fn set_recvr_is_online(&mut self, is_online: bool) {
        self.is_online = is_online;
        self.draw_conn_status(if is_online { Color::Green } else { Color::Red });
    }

    pub fn render(&mut self) {
        self.chat_msgs.render_to_parent();
        self.input_win.render_to_parent();
        self.chat_section.render();
    }

    fn push_chat_msg(&mut self, msg_dir: MsgDir) {
        let msg = msg_dir.to_chat_msg(&self.receiver_name);
        let msg_text = ": ".to_string() + msg.msg;
        let msg_height = ((msg.name.len() + msg_text.len()) / self.chat_msgs.width() as usize + 1) as u32;

        if self.next_y_pos + msg_height > self.chat_msgs.height() {
            self.chat_msgs.extend_scroll_height(50);
        }

        let mut name = Text::default()
            .text(msg.name)
            .size(msg.name.len() as u32, 1)
            .xy(0, self.next_y_pos as u32)
            .fg(msg.name_fg);
        let mut msg = Text::default()
            .text(&msg_text)
            .start_text((msg.name.len() as u32, 0))
            .size(self.chat_msgs.width(), msg_height)
            .xy(0, self.next_y_pos as u32);

        self.chat_msgs.draw_element(&name);
        self.chat_msgs.draw_element(&msg);

        name.set_xy(0, 0);
        msg.set_xy(0, 0);
        self.recent_chat.clear();
        self.recent_chat.draw_element(&name);
        self.recent_chat.draw_element(&msg);

        self.next_y_pos += msg_height as u32 + 1;
    }

    pub fn send<'a>(&mut self, msg: &'a str) {
        self.push_chat_msg(MsgDir::Send(msg));
    }

    pub fn receive<'a>(&mut self, msg: &'a str) {
        self.push_chat_msg(MsgDir::Receive(msg));
    }

    pub fn scroll_up(&mut self) {
        self.chat_msgs.set_scroll_xy_rel(0, -1);
    }

    pub fn scroll_down(&mut self) {
        self.chat_msgs.set_scroll_xy_rel(0, 1);
    }

    pub fn clear_input_win(&mut self) {
        self.input_win.clear();
        self.input_win.update_rel_xy();
        self.input_win.update_cursor();
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
    use crate::termin::{self, crossterm_handler::CrosstermHandler};
    use crossterm::{
        cursor,
        event::KeyCode,
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::io::stdout;

    use super::*;

    #[test]
    fn chat_section() {
        enable_raw_mode().unwrap();
        execute!(stdout(), EnterAlternateScreen, cursor::Hide).unwrap();
        let mut terminal = termin::root(CrosstermHandler::new(stdout()));

        let mut chat_sec = ChatSection::new(&mut terminal.root, String::from("test jinga boy"));
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
                        chat_sec.send("obobobobobobobob");
                        terminal.draw_window(&chat_sec.chat_msgs).unwrap();
                    }
                    KeyCode::Down => {
                        chat_sec.scroll_down();
                        terminal.draw_window(&chat_sec.chat_msgs).unwrap();
                    }
                    KeyCode::Up => {
                        chat_sec.scroll_up();
                        terminal.draw_window(&chat_sec.chat_msgs).unwrap();
                    }
                    KeyCode::Esc => break,
                    _ => {
                        chat_sec.handle_kbd(e);
                        chat_sec.input_win.render();
                        terminal
                            .handler
                            .draw_window(chat_sec.input_win.input_win())
                            .unwrap();
                    }
                },
                _ => continue,
            }
            chat_sec.input_win.show_cursor();
            chat_sec.input_win.update_cursor();
            terminal.flush().unwrap();
        }

        execute!(stdout(), cursor::Show, LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
    }
}
