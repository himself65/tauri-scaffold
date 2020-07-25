import 'react-app-polyfill/ie11'
import 'react-app-polyfill/stable'
import React, { useState } from 'react'
import { Button } from 'antd'
import './App.css'
import { exampleCall } from './api'

import 'antd/dist/antd.min.css'

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
