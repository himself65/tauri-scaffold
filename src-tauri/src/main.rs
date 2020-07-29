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
use reqwest::Error;
use std::collections::HashMap;
use std::env;
use std::sync::RwLock;

lazy_static! {
  static ref ADDRESS: RwLock<String> = RwLock::new(String::from("http://replace.me"));
}

async fn post_id(body: &RequestBody) -> Result<String, Error> {
  let mut map: HashMap<&str, &str> = HashMap::new();
  let id = body.id.to_string();
  let name = &body.name;
  map.insert("id", id.as_str());
  map.insert("name", name.as_str());

  reqwest::Client::new()
    .post(&format!("{}/hello", ADDRESS.read().unwrap()))
    .json(&map)
    .send()
    .await?
    .text()
    .await
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
                let response = async_std::task::block_on(async { post_id(&body).await });
                if let Ok(string) = response {
                  Ok(format!("{}", string))
                } else {
                  Ok("".to_string())
                }
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
