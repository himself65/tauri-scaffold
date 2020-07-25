#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;
use async_std::task;
use lazy_static::lazy_static;
use reqwest;
use std::env;
use std::sync::RwLock;

lazy_static! {
  static ref ADDRESS: RwLock<String> = RwLock::new(String::from("http://replace.me"));
}

async fn post_id() -> Result<String, reqwest::Error> {
  let client = reqwest::Client::new();
  let res = client
    .post(ADDRESS.read().unwrap().as_str())
    .body("{ id: \"himself65\" }")
    .send()
    .await?;
  let text = res.text().await?;
  Ok(text)
}

#[tokio::main]
async fn main() {
  let debug_mode: bool = env::var("NODE_ENV").map_or(false, |v| match v.as_str() {
    "DEBUG" => true,
    _ => false,
  });

  if debug_mode {
    let url = env::var("MOCK_PORT").map_or(String::from("http://localhost:3000"), |v| v);
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
                println!("{:?}", body);
                Ok(task::block_on(post_id()).unwrap_or("".to_string()))
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
