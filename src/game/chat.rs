use crossterm::event::Event;
use serde::Serialize;

use crate::termin::{window::{WindowRef, Window}, terminal_window::TerminalHandler};

#[derive(Debug)]
pub struct ChatMsg <'a> {
  msg: &'a str,
  name: &'a str
}

impl <'a> ChatMsg<'a> {
  pub fn new(name: &'a str, msg: &'a str) -> Self {
    Self{name, msg}
  }
}

#[derive(Debug)]
pub struct ChatSection<'a> {
  chat: Vec<ChatMsg<'a>>,
  chat_section: WindowRef,
  chat_msgs: WindowRef,
  recent_chat: WindowRef,
  input_win: WindowRef
}

pub enum ChatSectionEvent {
  ExitChatMode,
  EmitString(String),
  DrawInputBox,
  DrawChatMsgs
}

impl <'a> ChatSection<'a> {
  pub fn new(win: &mut WindowRef) -> Self {
    let mut chat_section = win.new_child(Window::default().size(50, 20));
    let chat_msgs = chat_section.new_child(Window::default().size(50, 18).scoll_size(50, 100));
    let recent_chat = win.new_child(Window::default().size(50, 1));
    let input_win = chat_section.new_child(Window::default().size(50, 1));

    Self {
      chat: vec![],
      chat_section,
      recent_chat,
      chat_msgs,
      input_win
    }
  }

  pub fn push_chat_msg(&mut self, msg: ChatMsg<'a>) {
    self.chat.push(msg);
  }

  pub fn handle_kbd(&mut self, e: Event) {
  }
}
