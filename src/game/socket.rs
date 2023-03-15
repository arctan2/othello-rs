use serde::{Serialize, Deserialize};
use tokio_tungstenite::{WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
pub struct SocketMsg {
  pub name: String,
  pub data: String
}

impl SocketMsg {
  pub fn from(s: String) -> SocketMsg {
    serde_json::from_str(&s).unwrap()
  }

  pub fn to_string<T: Serialize>(ev_name: &str, data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(&SocketMsg{name: ev_name.to_string(), data: serde_json::to_string(data)? })
  }

  pub fn parse<'a, T: Deserialize<'a>>(&'a self) -> T {
    serde_json::from_str(&self.data).unwrap()
  }

  pub fn event_name(&self) -> &str {
    &self.name
  }
}

pub type WS = WebSocketStream<MaybeTlsStream<TcpStream>>;

macro_rules! emit {
  ($socket_writer:expr,$e:expr) => {
    match SocketMsg::to_string($e, &"".to_string()) {
      Ok(msg) => match $socket_writer.send(Message::Text(msg)).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
      },
      Err(e) => Err(e.to_string())
    }
  };
  ($socket_writer:expr,$e:expr,$data:expr) => {
    match SocketMsg::to_string($e, &$data) {
      Ok(msg) => match $socket_writer.send(Message::Text(msg)).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
      },
      Err(e) => Err(e.to_string())
    }
  };
}

// must be inside async
macro_rules! emit_json {
  ($socket_writer:expr,$e:expr,$data:expr) => {
    match SocketMsg::to_string($e, &$data) {
      Ok(msg) => match $socket_writer.send(Message::Text(msg)).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string())
      },
      Err(e) => Err(e.to_string())
    }
  };
}

pub(crate) use {emit_json, emit};