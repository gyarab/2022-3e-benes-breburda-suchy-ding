import { createRouter, createWebHistory } from "vue-router";
import store from './store.js';

import Home from './HomePage.vue'
import LogIn from './LogInPage.vue'
// import Register from './RegisterPage.vue'
// import Profile from './ProfilePage.vue'
// import Post from './PostPage.vue'
// import ProfileSetUp from './ProfileSetUp.vue'

function loggedIn() {
  return store.loggedIn === true;
}

const router = createRouter({
  history: createWebHistory('/'),
  routes: [{
    path: '/',
    name: 'home',
    component: Home,
    meta: {
      title: 'Home :))',
      requiresLogin: true,
    },
  }, {
    path: '/login',
    name: 'login',
    component: LogIn,
    meta: {
      title: 'Log in',
    }
  }],
});

router.beforeResolve(async to => {
  if (to.meta.requiresLogin && !loggedIn()) {
    return { path: '/login' };
  }
})

export default router;
