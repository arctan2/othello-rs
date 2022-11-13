use std::{io::{Write, Stdout}, thread, time::Duration};

use crossterm::{style::Color, event::{Event, KeyCode}};

use crate::termin::{terminal_window::Terminal, elements::{Text, Rectangle}, window::Window};

pub struct Action <'a> {
  label: &'a str,
  action_fn: &'a dyn Fn(&mut Terminal<Stdout>) -> Return
}

pub struct Menu <'a> {
  heading: &'a str,
  list: Vec<MenuItem<'a>>,
  cursor: u16,
  id: u16,
}

pub struct SubMenu <'a> {
  label: &'a str,
  menu: Menu<'a>
}

pub enum MenuItem <'a> {
  SubMenu(SubMenu<'a>),
  Action(Action<'a>)
}

pub enum Return {
  ToRoot,
  Back,
  None,
  All
}

fn sleep(ms: u64) {
  thread::sleep(Duration::from_millis(ms));
}

impl <'a> Menu <'a> {
  pub fn new(heading: &'static str) -> Self {
    Self { heading, list: vec![], cursor: 0, id: 0 }
  }

  pub fn action(mut self, label: &'a str, action_fn: &'a dyn Fn(&mut Terminal<Stdout>) -> Return) -> Self {
    self.list.push(MenuItem::<'a>::Action(Action{label, action_fn}));
    self
  }

  pub fn sub_menu(mut self, label: &'a str, mut sub_menu: Menu<'a>) -> Self {
    sub_menu.id = self.id + 1; 
    self.list.push(MenuItem::<'a>::SubMenu(SubMenu{menu: sub_menu, label}));
    self
  }

  pub fn back(mut self, label: &'a str) -> Self {
    self.list.push(MenuItem::<'a>::Action(Action{label, action_fn: &|_| -> Return { Return::Back }}));
    self
  }

  pub fn run<T>(&mut self, terminal: &mut Terminal<Stdout>, ctx: &mut T) -> Return {
    terminal.root.clear();
    let heading = Text::default().text(self.heading).size(10, 1);
    let mut options_win = terminal.root.new_child(Window::default().size(50, (self.list.len() * 2) as u16).position(2, 2));
    let mut opt = Text::default().start_text((1, 0));

    terminal.root.draw_element(&heading);

    loop {
      options_win.clear();
      for (idx, o) in self.list.iter().enumerate() {
        let label = match o {
          MenuItem::Action(a) => a.label,
          MenuItem::SubMenu(m) => m.label
        };

        opt.set_size(label.len() as u16 + 2, 1);
        opt.set_text(label);
        opt.set_fg(Color::Reset);
        opt.set_pos(0, (idx * 2) as u16);

        if idx == self.cursor as usize {
          let bg = Rectangle::from_rect(opt.get_rect()).bg(Color::Green);
          options_win.draw_element(&bg); 
          opt.set_fg(Color::Black);
        }

        options_win.draw_element(&opt);
      }
      options_win.render();
      terminal.refresh().unwrap();

      match terminal.event() {
        Event::Key(k) => {
          match k.code {
            KeyCode::Esc => return Return::All,
            KeyCode::Down => {
              self.cursor += 1;
              if self.cursor == self.list.len() as u16 {
                self.cursor = 0;
              }
            },
            KeyCode::Up => {
              if self.cursor == 0 {
                self.cursor = (self.list.len() - 1) as u16;
              } else {
                self.cursor -= 1;
              }
            },
            KeyCode::Enter => {
              let menu_item = &mut self.list[self.cursor as usize];
              match menu_item {
                MenuItem::Action(a) => {
                  match (a.action_fn)(terminal) {
                    Return::ToRoot => {
                      if self.id != 0 {
                        return Return::ToRoot;
                      }
                    },
                    Return::Back => return Return::None,
                    Return::All => return Return::All,
                    Return::None => ()
                  }
                },
                MenuItem::SubMenu(sm) => {
                  match sm.menu.run(terminal, ctx) {
                    Return::ToRoot => {
                      if self.id != 0 {
                        return Return::ToRoot;
                      }
                    },
                    Return::Back => return Return::None,
                    Return::All => return Return::All,
                    Return::None => ()
                  }
                }
              }
              terminal.root.clear();
              terminal.root.draw_element(&heading);
            },
            _ => ()
          }
        },
        _ => ()
      }
    }
  }
}
