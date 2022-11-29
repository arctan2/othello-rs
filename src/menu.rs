use std::{io::Stdout, thread, time::Duration};

use crossterm::{style::Color, event::{Event, KeyCode}};

use crate::termin::{terminal_window::Terminal, elements::{Text, Rectangle}, window::{Window, Position::*, WindowRef}};

pub struct Action <'a, T> {
  label: &'a str,
  action_fn: &'a dyn Fn(&mut Terminal<Stdout>, &mut T) -> Return
}

pub struct Menu <'a, T> {
  pub heading: Text,
  list: Vec<MenuItem<'a, T>>,
  cursor: u16,
  id: u16,
  routine_fn: Option<&'a dyn Fn(&mut Self, &mut T)>
}

pub struct SubMenu <'a, T> {
  label: &'a str,
  menu: Menu<'a, T>
}

pub enum MenuItem <'a, T> {
  SubMenu(SubMenu<'a, T>),
  Action(Action<'a, T>)
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

impl <'a, T> Menu <'a, T> {
  pub fn new(heading: &'static str) -> Self {
    Self { heading: Text::default().text(heading), list: vec![], cursor: 0, id: 0, routine_fn: None }
  }

  pub fn action(mut self, label: &'a str, action_fn: &'a dyn Fn(&mut Terminal<Stdout>, &mut T) -> Return) -> Self {
    self.list.push(MenuItem::<'a>::Action(Action{label, action_fn}));
    self
  }

  pub fn sub_menu(mut self, label: &'a str, mut sub_menu: Menu<'a, T>) -> Self {
    sub_menu.id = self.id + 1; 
    self.list.push(MenuItem::<'a>::SubMenu(SubMenu{menu: sub_menu, label}));
    self
  }

  pub fn back(mut self, label: &'a str) -> Self {
    self.list.push(MenuItem::<'a>::Action(Action{label, action_fn: &|_, _| -> Return { Return::Back }}));
    self
  }

  pub fn routine(mut self, func: &'a dyn Fn(&mut Self, &mut T)) -> Self {
    self.routine_fn = Some(func);
    self
  }

  pub fn run(&mut self, terminal: &mut Terminal<Stdout>, ctx: &mut T) -> Return {
    let mut menu_win = terminal.root.new_child(
      Window::default().size(terminal.root.width(), (self.list.len() * 2 + 4) as u16)
    );
    let mut options_win = menu_win.new_child(
      Window::default().size(40, (self.list.len() * 2 - 1) as u16)
    );

    let mut opt = Text::default().start_text((1, 0));
    let run_routine = |s: &mut Self, ctx: &mut T| {
      match s.routine_fn {
        Some(r) => r(s, ctx),
        None => ()
      }
    };

    terminal.root.clear();
    run_routine(self, ctx);
    options_win.set_position(CenterB);

    self.heading.set_position(menu_win.rect(), CenterH);
    menu_win.draw_element(&self.heading);

    menu_win.set_position(CenterV);
    menu_win.set_xy_rel(0, -2);
      
    let return_val: Return;

    macro_rules! RETURN {
      ($v:ident) => {
        return_val = Return::$v; break;
      };
    }

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
        opt.set_xy(0, (idx * 2) as u16);
        opt.set_position(options_win.rect(), CenterH);

        if idx == self.cursor as usize {
          let bg = Rectangle::from_rect(opt.get_rect()).bg(Color::Green);
          options_win.draw_element(&bg); 
          opt.set_fg(Color::Black);
        }

        options_win.draw_element(&opt);
      }

      terminal.root.clear();
      options_win.render_to_parent();
      menu_win.render();
      terminal.refresh().unwrap();

      match terminal.event() {
        Event::Key(k) => {
          match k.code {
            KeyCode::Esc => {
              RETURN!(All);
            },
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
                  match (a.action_fn)(terminal, ctx) {
                    Return::ToRoot => {
                      if self.id != 0 {
                        RETURN!(ToRoot);
                      }
                    },
                    Return::Back => { RETURN!(None); },
                    Return::All => { RETURN!(All); },
                    Return::None => ()
                  }
                },
                MenuItem::SubMenu(sm) => {
                  match sm.menu.run(terminal, ctx) {
                    Return::ToRoot => {
                      if self.id != 0 {
                        RETURN!(ToRoot);
                      }
                    },
                    Return::Back => { RETURN!(None); },
                    Return::All => { RETURN!(All); },
                    Return::None => ()
                  }
                }
              }
              menu_win.clear();
              run_routine(self, ctx);
              self.heading.set_position(menu_win.rect(), CenterH);
              menu_win.draw_element(&self.heading);
            },
            _ => ()
          }
        },
        _ => ()
      }
    }

    menu_win.delete();
    return_val
  }
}
