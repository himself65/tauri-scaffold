use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
  pub id: u32,
  pub name: String,
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  ExampleCommand {
    body: RequestBody,
    callback: String,
    error: String,
  },
}
