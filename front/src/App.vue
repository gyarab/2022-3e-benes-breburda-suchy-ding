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
@import url('https://fonts.googleapis.com/css?family=Manjari');

#app {
  font-family: Manjari;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: white;
  margin-top: 60px;
}

body {
  @apply bg-[#13131b]
}

button {
  width: 10%;
  padding: 6px 12px;
  border: 2px solid #20201E;

  @apply bg-[#222230] rounded-3xl text-white hover:bg-[#303044]
}
text {
  color: white;
  
}

input {
  width: 20%;
  padding: 12px 20px;
  margin: 8px 0;
  border-radius: 25px;
  -webkit-text-fill-color: rgb(255, 255, 255);

  @apply bg-[#222230] content-center
}

</style>
