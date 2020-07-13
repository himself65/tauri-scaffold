use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RequestBody {
  id: u32,
  name: String
}

#[derive(Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  ExampleCommand { body: RequestBody, callback: String, error: String  },
}
