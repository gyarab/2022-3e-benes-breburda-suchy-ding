<script setup>
import { ref } from 'vue'
import { loadUserData } from './store'
import { EyeIcon, EyeSlashIcon } from '@heroicons/vue/20/solid'

import { useRouter } from 'vue-router'
import rest from './rest';
import { notify } from './notify'

const router = useRouter()
const email = ref('')
const showPassword = ref(false)
const whichIcon = ref(false);
const password = ref('')

async function login() {
  const resp = await rest.post('/api/sessions', {
    email: email.value,
    password: password.value,
  });

  switch (resp.status) {
    case 200:
      window.localStorage.setItem('api-key', resp.body.token);
      await loadUserData();
      router.push({ path: '/' })
      break
    default:
      notify('error', 'Invalid password')
      break
    case 404:
      notify('error', 'Invalid email')
      break
  }

}

function switchIcon() {
  whichIcon.value = !whichIcon.value;
}

function switchVisibility() {
  showPassword.value = !showPassword.value;
}

</script>

<template>
  <div class="flex h-screen items-center justify-center">
    <div>
      <div class="flex relative">
        <h1 class="feather-font text-6xl mb-4">ding</h1>
        <h2 class="text-3xl feather-font ml-4 mt-6 text-[#828282] right-0">login</h2>
      </div>
      <input v-model="email" placeholder="Email" @keyup.enter="login">
      <div class="password-field">
        <input :type="showPassword ? 'text' : 'password'" placeholder="Password" v-model="password" @keyup.enter="login">
        <button v-if="whichIcon" @click="switchVisibility(); switchIcon();"
          class="border-transparent hover:bg-transparent">
          <EyeIcon class="eyeIcon" />
        </button>
        <button v-else @click=" switchVisibility(); switchIcon(); " class="border-transparent hover:bg-transparent">
          <EyeSlashIcon class="eyeIcon" />
        </button>
      </div>
      <button class="submit-button" @click=" login ">Log in</button>
      <div>Not a user yet? <RouterLink class="register" to="/register">Register here!</RouterLink>
      </div>
    </div>
  </div>
</template> 

<style>
.feather-font {
  font-family: 'FeatherBold', sans-serif;
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
  border: none;
  transition-duration: 0.5s;
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
