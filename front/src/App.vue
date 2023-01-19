<script setup>
import { ref, computed } from 'vue'
import Home from './HomePage.vue'
import LogIn from './LogInPage.vue'


const routes = {
  '/': Home,
  '/login': LogIn
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
  <a href="#/">Home</a>
  <a href="#/login">Log in</a>
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
