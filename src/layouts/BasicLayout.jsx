import 'antd/dist/antd.css'

import { Button, Layout, Menu } from 'antd'
import React from 'react'
import importedComponent from 'react-imported-component'
import { BrowserRouter, Link, Route, Switch } from 'react-router-dom'

import globalConfig from '../config'

const { Header, Content, Sider } = Layout
const asyncComponentFactory = resolve => importedComponent(resolve)

const DashboardView = () => {
  return (
    <BrowserRouter>
      <Layout className='winepot-layout'>
        <Header className='header'>
          <div className='logo' />
          <Button type='link'>
            更换节点
          </Button>
        </Header>
        <Layout>
          <Sider width={200} className='site-layout-background'>
            <Menu
              mode='inline'
              defaultSelectedKeys={['1']}
              defaultOpenKeys={['sub1']}
              style={{ height: '100%', borderRight: 0 }}
            >
              {
                globalConfig.routers.map(({ name, icon, to }) => (
                  <Menu.Item key={name}>
                    <Link to={to}>
                      {name}
                    </Link>
                  </Menu.Item>
                ))
              }
            </Menu>
          </Sider>
          <Layout style={{ padding: '0 24px 24px' }}>
            <Content
              className='site-layout-background'
              style={{
                padding: 24,
                margin: 0,
                minHeight: 280
              }}
            >
              <Switch>
                {globalConfig.routers.map(({ component, name, to }) => (
                  <Route to={to} exact={to === '/'} component={asyncComponentFactory(component)} key={name}/>
                ))}
              </Switch>
            </Content>
          </Layout>
        </Layout>
      </Layout>
    </BrowserRouter>
  )
}

export default DashboardView
