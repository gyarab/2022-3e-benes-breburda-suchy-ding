<script setup>
import { ref } from 'vue'
import store from './store'

const username = ref('')

function login() {
  store.user = {
    username: username.value,
  }
  window.location.hash = '#/'
}

const passwordField = ref(null)

function switchVisibility(){
  console.log(passwordField.value.getAttribute('type'))
  if (passwordField.value.getAttribute('type') === 'password'){
    passwordField.value.setAttribute('type', 'text')
  }
  else {
    passwordField.value.setAttribute('type', 'password')
  }
}

</script>

<template>
  <div class="container">
    <div><input v-model="username" placeholder="Username"></div>
    <div class="password-field">
      <div><input type="password" ref="passwordField" placeholder="Password"></div>
      <button @click="switchVisibility">SH</button>
    </div>
    <button class="submit-button" @click="login">Log in</button>
    <div>Not a user yet? <a href="#/register">Register here!</a></div>
  </div>
</template> 

<style>

.container {
  text-align: center;
  width: 30rem;
  margin: auto;
}
input {
  width: 100%;
}
.password-field {
  position: relative;
}
.password-field button {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  right: 10px;
}
.submit-button {
  margin: 1rem 0;
}
</style>