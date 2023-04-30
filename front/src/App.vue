<script setup>
import { notifications } from './notify'
</script>
<template>
  <div class="notification-block">
    <div v-for="i in notifications" :key="i.text" class="notification flex items-center"
      :class="{ error: i.type == 'error' }">
      <p>{{ i.text }}</p>
      <button @click="notifications.splice(notifications.findIndex(j => j.id == i.id), 1)"
        class="notificationCloseButton">X</button>
    </div>
  </div>
  <RouterView :key="$route.fullPath"></RouterView>

  <!-- pozustatek nasich nadeji ne lepsi zitrky :( -->
  <!-- <div class="testing">
    <RouterLink to="/">Home</RouterLink>
    <RouterLink to="/login">Log in</RouterLink>
    <RouterLink to="/register">Register</RouterLink>
    <RouterLink to="/settings">Settings</RouterLink>
    <RouterLink to="/setup">Setup</RouterLink>
  </div> -->
</template> 

<style>
@import url('https://fonts.googleapis.com/css?family=Inter');

#app {
  font-family: Inter;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  color: white;
}

body {
  background: #13131b;
}

button {
  padding: 0.75rem 1.5rem;
  border: 2px solid #20201E;

  @apply bg-[#222230] rounded-3xl text-white
}

button:hover {
  @apply bg-[#303044]
}

text {
  color: white;
}

input {
  box-sizing: border-box;
  padding: 1rem 2rem;
  margin: 8px 0;
  border-radius: 25px;
  border: none;
  font-size: 1.25rem;

  @apply bg-[#222230]
}

.testing {
  position: fixed;
  left: 50%;
  bottom: 20px;
  transform: translate(-50%, -50%);
  margin: 0 auto;
}

.notification-block {
  position: fixed;
  width: min(70ch, 95vw);
  top: 5px;
  left: 50vw;
  transform: translateX(-50%);
  font-weight: bold;
}

.notification {
  padding: 0.5rem 1rem;
  margin: 0.5rem;
  background: #2a2a3c;
  border-radius: 1rem;
}

.notification .error {
  background: #aa3333;
}

.notificationCloseButton {
  background: none;
  border: none;
  transition-duration: 0.3s;
  margin-left: auto;
}

.notificationCloseButton:hover {
  background: none;
  border: none;
}
</style>
