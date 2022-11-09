use std::{io::Write, thread, time::Duration};

use crossterm::style::Color;

use crate::termin::{terminal_window::Terminal, elements::Text, window::Window};

pub struct Action <'a> {
  label: &'a str,
  action: &'a dyn Fn()
}

pub struct SubMenu <'a> {
  label: &'a str,
  menu: Menu<'a>
}

pub struct Menu <'a> {
  heading: &'a str,
  list: Vec<MenuItem<'a>>,
  cursor: u16
}

pub enum MenuItem <'a> {
  SubMenu(SubMenu<'a>),
  Action(Action<'a>)
}

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

impl <'a> Menu <'a> {
  pub fn new(heading: &'static str) -> Self {
    Self { heading, list: vec![], cursor: 0 }
  }

  pub fn action(mut self, label: &'a str, action: &'a dyn Fn()) -> Self {
    self.list.push(MenuItem::<'a>::Action(Action{label, action}));
    self
  }

  pub fn sub_menu(mut self, label: &'a str, sub_menu: Menu<'a>) -> Self {
    self.list.push(MenuItem::<'a>::SubMenu(SubMenu{menu: sub_menu, label}));
    self
  }

  pub fn back(mut self, label: &'a str) -> Self {
    self.list.push(MenuItem::<'a>::Action(Action{label, action: &|| {}}));
    self
  }

  pub fn run<W: Write>(&self, terminal: &mut Terminal<W>) {
    let heading = Text::default().text(self.heading).size(10, 1);
    let mut options_win = terminal.root.new_child(Window::default().size(50, (self.list.len() * 2) as u16).position(2, 2));
    let mut opt = Text::default();

    terminal.root.draw_element(&heading);

    for (idx, o) in self.list.iter().enumerate() {
      let label = match o {
        MenuItem::Action(a) => a.label,
        MenuItem::SubMenu(m) => m.label
      };

      opt.set_size(label.len() as u16, 1);
      opt.set_text(label);
      opt.set_pos(0, (idx * 2) as u16);
      options_win.draw_element(&opt);
    }
    options_win.render();
    terminal.refresh().unwrap();
    sleep(5000);
  }
}
