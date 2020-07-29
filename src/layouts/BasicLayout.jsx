import {
  AppBar,
  Button,
  Drawer,
  IconButton,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Toolbar,
  Typography
} from '@material-ui/core'
import Icon from '@material-ui/core/Icon'
import { createStyles, makeStyles } from '@material-ui/core/styles'
import { Menu as MenuIcon } from '@material-ui/icons'
import React, { useCallback, useState } from 'react'
import importedComponent from 'react-imported-component'
import { BrowserRouter, Route, Switch } from 'react-router-dom'

import globalConfig from '../config'

const asyncComponentFactory = resolve => importedComponent(resolve)

const useStyles = makeStyles(theme =>
  createStyles({
    root: {
      flexGrow: 1
    },
    menuButton: {
      marginRight: theme.spacing(2)
    },
    title: {
      flexGrow: 1
    },
    list: {
      width: 250
    }
  })
)

const BasicLayout = () => {
  const classes = useStyles()
  const [isOpen, setOpen] = useState(false)
  const onMenuIconClick = useCallback(() => {
    setOpen(true)
  }, [])
  const onDrawerClose = useCallback(() => {
    setOpen(false)
  }, [])

  return (
    <BrowserRouter>
      <AppBar position='static'>
        <Toolbar>
          <IconButton edge='start' className={classes.menuButton} color='inherit' aria-label='menu'>
            <MenuIcon onClick={onMenuIconClick}/>
          </IconButton>
          <Typography variant='h6' className={classes.title}>
            News
          </Typography>
          <Button color='inherit'>Login</Button>
        </Toolbar>
      </AppBar>
      <Drawer open={isOpen} onClose={onDrawerClose}>
        <List className={classes.list}>
          {
            globalConfig.routers.map(({ name, icon }) => (
              <ListItem button key={name}>
                <ListItemIcon><Icon>{icon}</Icon></ListItemIcon>
                <ListItemText primary={name}/>
              </ListItem>
            ))
          }
        </List>
      </Drawer>
      <Switch>
        {globalConfig.routers.map(({ component, name, to }) => (
          <Route to={to} exact={to === '/'} component={asyncComponentFactory(component)} key={name}/>
        ))}
      </Switch>
    </BrowserRouter>
  )
}

export default BasicLayout
