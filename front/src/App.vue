<script setup>
import { ref, computed } from 'vue'
import Home from './HomePage.vue'
import LogIn from './LogInPage.vue'
import Register from './RegisterPage.vue'


const routes = {
  '/': Home,
  '/login': LogIn,
  '/register': Register
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
  <notifications position="top center" />
  <a href="#/">Home</a>
  <a href="#/login">Log in</a>
  <a href="#/register">Register</a>
  <component :is="currentView" />
</template> 


<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
