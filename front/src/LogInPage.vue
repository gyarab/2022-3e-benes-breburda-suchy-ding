<script setup>
import { ref } from 'vue'
import { store } from './store'
import { EyeIcon, EyeSlashIcon } from '@heroicons/vue/20/solid'
import { useRouter } from 'vue-router'

const router = useRouter()
const username = ref('')
const showPassword = ref(false)
const whichIcon = ref(false);

function login() {
  store.user = {
    username: username.value,
  }
  store.loggedIn = true;
  router.push({ path: '/' })
}

function switchIcon() {
  whichIcon.value = !whichIcon.value;
}

function switchVisibility(){
  showPassword.value = !showPassword.value;
}

</script>

<template>
  <div class="flex h-screen items-center justify-center">
    <div>
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
      <div>Not a user yet? <RouterLink class="register" to="/register">Register here!</RouterLink></div>
    </div>
  </div>
</template> 

<style>

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

.register {
  text-decoration: underline;
}
</style>
