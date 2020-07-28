#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[macro_use]
extern crate log;

mod cmd;
use crate::cmd::RequestBody;
use lazy_static::lazy_static;
use reqwest;
use serde_json::json;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

lazy_static! {
  static ref ADDRESS: RwLock<String> = RwLock::new(String::from("http://replace.me"));
}

fn post_id(body: &RequestBody) -> Result<String, reqwest::Error> {
  let client = reqwest::blocking::Client::new();
  let url = format!("{}/hello", ADDRESS.read().unwrap());
  debug!("post_id({})", url);
  let mut map: HashMap<&str, &str> = HashMap::new();
  let id = body.id.to_string();
  let name = &body.name;
  map.insert("id", id.as_str());
  map.insert("name", name.as_str());
  let res = client.post(&url).json(&map).send()?;
  match res.text() {
    Ok(text) => Ok(text),
    Err(_) => Ok("".to_string()),
  }
}

fn main() {
  env_logger::init();
  let debug_mode: bool = env::var("NODE_ENV").map_or(false, |v| match v.as_str() {
    "DEBUG" => true,
    _ => false,
  });
  debug!("env.NODE_ENV: {}", debug_mode);

  if debug_mode {
    let url = env::var("MOCK_PORT").map_or(String::from("http://localhost:8081"), |v| {
      format!("http://localhost:{}", v)
    });
    debug!("env.MOCK_PORT: {}", url);
    *ADDRESS.write().unwrap() = url;
  }
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => Err(e.to_string()),
        Ok(command) => {
          match command {
            ExampleCommand {
              body,
              callback,
              error,
            } => tauri::execute_promise(
              _webview,
              move || {
                let res = post_id(&body);
                Ok(res.map_or("".to_string(), |v| v))
              },
              callback,
              error,
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
