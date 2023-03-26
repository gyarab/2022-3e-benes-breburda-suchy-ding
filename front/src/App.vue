<script setup>
import { ref, computed } from 'vue'
import Home from './HomePage.vue'
import LogIn from './LogInPage.vue'
import Register from './RegisterPage.vue'
import Profile from './ProfilePage.vue'
import Post from './PostPage.vue'
import ProfileSetUp from './ProfileSetUp.vue'



const routes = {
  '/': Home,
  '/login': LogIn,
  '/register': Register,
  '/profile': Profile,
  '/post': Post,
  '/profilesetup': ProfileSetUp


}

const currentPath = ref(window.location.hash)

window.addEventListener('hashchange', () => {
  currentPath.value = window.location.hash
})

const currentView = computed(() => {
  return routes[currentPath.value.slice(1) || '/']
})
</script>

<template>
  <!-- <NotificationsPage></NotificationsPage> -->
  <a href="#/">Home</a>
  <a href="#/login">Log in</a>
  <a href="#/register">Register</a>
  <a href="#/profile">Profile</a>
  <a href="#/post">Post</a>
  <a href="#/profilesetup">ProfileSetUp</a>


  <component :is="currentView" />
</template> 


<style>
@import url('https://fonts.googleapis.com/css?family=Manjari');


#app {
  font-family: Manjari;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: white;
  margin-top: 60px;
}

body {
  background: #13131b;
}

button {
  padding: 0.75rem 1.5rem;
  border: 2px solid #20201E;
  font-family: Manjari;

  @apply bg-[#222230] rounded-3xl text-white hover:bg-[#303044]
}

text {
  color: white;
}

input {
  box-sizing: border-box;
  padding: 1rem 2rem;
  margin: 8px 0;
  border-radius: 25px;
  -webkit-text-fill-color: rgb(255, 255, 255);
  font-family: Manjari;
  border: none;
  font-size: 1.25rem;

  @apply bg-[#222230]
}

</style>
