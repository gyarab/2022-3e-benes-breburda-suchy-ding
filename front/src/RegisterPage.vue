<script setup>
import { ref } from 'vue'
import { notify } from './notify'
import { EyeIcon, EyeSlashIcon } from '@heroicons/vue/20/solid'
import rest from './rest'
import { loadUserData } from './store'
import { useRouter } from 'vue-router'

const router = useRouter()
const passwordField1 = ref("")
const passwordField2 = ref("")
const email = ref("")
const username = ref("")
let showError = ref(false)
const showPassword = ref(false)
const whichIcon = ref(false);

function switchVisibility() {
  showPassword.value = !showPassword.value;
}
function passwordMatch() {
  showError.value = passwordField1.value && passwordField1.value === passwordField2.value
  if (!showError.value) {
    notify('error', 'Passwords do not match')
  }
  return showError.value;
}
function switchIcon() {
  whichIcon.value = !whichIcon.value;
}
async function register() {
  if (username.value.length < 3) {
    notify('error', 'Username must be at least 3 characters long')
    return
  }
  if (passwordField1.value.length < 8) {
    notify('error', 'Password must be at least 8 characters long')
    return
  }
  if (!email.value || !/[^@]+@[^@]+\.[^@]+/.test(email.value)) {
    notify('error', 'Email is invalid')
    return
  }
  if (!passwordMatch()) return;
  const reg_response = await rest.post('/api/users', {
    name: username.value,
    email: email.value,
    password: passwordField1.value,
  });
  console.log(reg_response);
  if (reg_response.status == 200) {
    const login_response = await rest.post('/api/sessions', {
      email: email.value,
      password: passwordField1.value,
    });
    console.log(login_response);
    if (login_response.status == 200) {
      window.localStorage.setItem('api-key', login_response.body.token);
      await loadUserData();
      router.push('/setup');
    } else {
      notify('error', 'Registered, but failed to log in')
    }
  } else {
    notify('error', 'Failed to register')
  }
}
</script>

<template>
  <div class="flex h-screen items-center justify-center">
    <div>
      <div class="flex relative">
        <h1 class="feather-font text-6xl mb-4">ding</h1>
        <h2 class="text-3xl feather-font ml-4 mt-6 text-[#828282] right-0">register</h2>
      </div>
      <div> <input v-model="username" placeholder="Username" @keyup.enter="register" /> </div>
      <div> <input type="email" v-model="email" placeholder="Email" @keyup.enter="register" /> </div>
      <div class="password-field">
        <div> <input :type="showPassword ? 'text' : 'password'" v-model="passwordField1" placeholder="Password"
            @keyup.enter="login"> </div>
        <button v-if="whichIcon" @click="switchVisibility(); switchIcon();"
          class="border-transparent hover:bg-transparent">
          <EyeIcon class="eyeIcon" />
        </button>
        <button v-else @click=" switchVisibility(); switchIcon(); " class="border-transparent hover:bg-transparent">
          <EyeSlashIcon class="eyeIcon" />
        </button>
      </div>
      <div class="password-field">
        <div> <input :type=" showPassword ? 'text' : 'password' " v-model=" passwordField2 "
            placeholder="Confirm Password" @keyup.enter=" register ">
        </div>
        <button v-if=" whichIcon " @click=" switchVisibility(); switchIcon(); "
          class="border-transparent hover:bg-transparent">
          <EyeIcon class="eyeIcon" />
        </button>
        <button v-else @click=" switchVisibility(); switchIcon(); " class="border-transparent hover:bg-transparent">
          <EyeSlashIcon class="eyeIcon" />
        </button>
      </div>
      <button class="submit-button" @click=" register " @keyup.enter=" register ">Register</button>
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
  border: none;
  transition-duration: 0.5s;
  margin: 1rem 0;
}
</style>