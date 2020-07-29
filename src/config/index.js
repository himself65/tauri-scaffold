export default {
  title: 'my-project',
  routers: [
    {
      name: 'Home',
      icon: 'home',
      to: '/',
      component: () => import('../pages/HomePage')
    },
    {
      name: 'About',
      icon: 'info',
      to: '/about',
      component: () => import('../pages/HomePage')
    }
  ]
}
