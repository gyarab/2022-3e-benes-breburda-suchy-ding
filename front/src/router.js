import { createRouter, createWebHistory } from "vue-router";
import { store } from './store.js';

import Home from './HomePage.vue'
import LogIn from './LogInPage.vue'
import Register from './RegisterPage.vue'
import Settings from './SettingsPage.vue'
import Setup from './SetupPage.vue'
import SavedPostsPage from './SavedPostsPage.vue'
import ProfilePage from './ProfilePage.vue'
import NewPostPage from './NewPostPage.vue'

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
      title: 'Home',
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
    path: '/settings',
    name: 'settings',
    component: Settings,
    meta: {
      title: 'Settings',
      requiresLogin: true,
    }
  }, {
    path: '/setup',
    name: '/setup',
    component: Setup,
    meta: {
      title: 'Setup',
      requiresLogin: false,
    }
  }, {
    path: '/saved',
    name: 'saved-posts',
    component: SavedPostsPage,
    meta: {
      title: 'Saved posts',
      requiresLogin: true
    }
  }, {
    path: '/user/:user',
    name: 'profile',
    component: ProfilePage,
    props: true,
    meta: {
      title: 'User profile',
      requiresLogin: true
    }
  }, {
    path: '/new-post',
    name: 'new-post',
    component: NewPostPage,
    meta: {
      title: 'New post',
      requiresLogin: true
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
