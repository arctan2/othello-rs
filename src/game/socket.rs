use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketMsg {
  pub name: String,
  pub data: String
}

impl SocketMsg {
  pub fn from(s: String) -> SocketMsg {
    serde_json::from_str(&s).unwrap()
  }

  pub fn parse<'a, T: Deserialize<'a>>(&'a self) -> T {
    serde_json::from_str(&self.data).unwrap()
  }

  pub fn event_name(&self) -> &str {
    &self.name
  }
}
