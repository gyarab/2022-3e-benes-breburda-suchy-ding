import { createRouter, createWebHistory } from "vue-router";
import { store } from './store.js';

import Home from './HomePage.vue'
import LogIn from './LogInPage.vue'
import Register from './RegisterPage.vue'
import Profile from './ProfilePage.vue'
import Settings from './SettingsPage.vue'


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
    },
    beforeEnter: () => {
      if (loggedIn()) return { path: '/' };
    },
  }, {
    path: '/register',
    name: 'register',
    component: Register,
    meta: {
      title: 'Sign up',
    },
    beforeEnter: () => {
      if (loggedIn()) return { path: '/' };
    },
  }, {
    path: '/profile',
    name: 'profile',
    component: Profile,
    meta: {
      title: 'My profile',
      requiresLogin: true,
    }
  }, {
    path: '/settings',
    name: 'settings',
    component: Settings,
    meta: {
      title: 'Settings',
      requiresLogin: true,
    }
  }],
});

router.beforeResolve(async to => {
  if (to.meta.requiresLogin && !loggedIn()) {
    return { path: '/login' };
  }
  document.title = to.meta.title;
})

export default router;
