#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod cmd;

fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          Err(e.to_string())
        }
        Ok(command) => {
          match command {
            ExampleCommand { body, callback, error } => {
              tauri::execute_promise(
                _webview,
                move || {
                  println!("{:?}", body);
                  Ok("every is ok".to_string())
                },
                callback,
                error
              )
            }
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
