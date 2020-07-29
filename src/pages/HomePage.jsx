import { Button } from 'antd'
import React, { useState } from 'react'

import { exampleCall } from '../api'

const HomePage = () => {
  const [content, setContent] = useState('nothing')
  return (
    <div>
      <Button onClick={() => {
        exampleCall().then(res => {
          setContent(res)
        })
      }}>Click me</Button>
      Content: {content}
    </div>
  )
}

export default HomePage
