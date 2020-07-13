import React, { useState } from 'react'
import { Button } from '@material-ui/core'
import './App.css'
import { exampleCall } from './api'

const App = () => {
  const [content, setContent] = useState<string>('nothing')
  return (
    <div className="App">
      <Button onClick={() => {
        exampleCall().then(res => {
          setContent(res)
        })
      }}>Click me</Button>
      Content: {content}
    </div>
  )
}

export default App
