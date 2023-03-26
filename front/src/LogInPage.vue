<script setup>
import { ref } from 'vue'
import store from './store'
import { EyeIcon, EyeSlashIcon } from '@heroicons/vue/20/solid'

const username = ref('')
const showPassword = ref(false)
const whichIcon = ref(false);

function login() {
  store.user = {
    username: username.value,
  }
  window.location.hash = '#/'
}

function switchIcon() {
  whichIcon.value = !whichIcon.value;
}

function switchVisibility(){
  showPassword.value = !showPassword.value;
}

</script>

<template>
  <div class="container">
    <input v-model="username" placeholder="Username">
    <div class="password-field">
      <input :type="showPassword ? 'text' : 'password'" placeholder="Password">
      <button v-if="whichIcon" @click="switchVisibility(); switchIcon();">
        <EyeIcon class="eyeIcon" />
      </button>
      <button v-else @click="switchVisibility(); switchIcon();">
        <EyeSlashIcon class="eyeIcon" />
      </button>
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

.eyeIcon {
  width: 20px;
  margin: -0.45rem 0;
}

</style>