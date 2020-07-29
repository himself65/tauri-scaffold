import 'react-app-polyfill/ie11'
import 'react-app-polyfill/stable'
import './App.css'

import React from 'react'

import BasicLayout from './layouts/BasicLayout'

const App = () => {
  return (
    <div className='App'>
      <BasicLayout/>
    </div>
  )
}

export default App
