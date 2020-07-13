import { promisified } from 'tauri/api/tauri'

export async function exampleCall () {
  return promisified<string>({
    cmd: 'exampleCommand',
    body: {
      id: 5,
      name: 'hello, world!'
    }
  })
}
