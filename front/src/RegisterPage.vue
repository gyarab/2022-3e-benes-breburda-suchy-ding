<script setup>
import { ref } from 'vue'
import { notify } from '@kyvg/vue3-notification'
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
function switchVisibility(){
  showPassword.value = !showPassword.value;
}
function passwordMatch(){
  showError.value = passwordField1.value && passwordField1.value === passwordField2.value
  if (!showError.value) {
    notify({title: "password mismatch", type: "error"})
  }
  return showError.value;
}
function switchIcon() {
  whichIcon.value = !whichIcon.value;
}
async function register() {
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
      router.push('/');
    }
  }
}
</script>

<template>
  <div class="container">
    <div> <input v-model="username" placeholder="Username" /> </div>
    <div> <input v-model="email" placeholder="Email" /> </div>
    <div class="password-field">
      <div> <input :type="showPassword ? 'text' : 'password'" v-model="passwordField1" placeholder="Password"> </div>
      <button v-if="whichIcon" @click="switchVisibility(); switchIcon();" class="border-transparent hover:bg-transparent">
      <EyeIcon class="eyeIcon" />
    </button>
    <button v-else @click="switchVisibility(); switchIcon();" class="border-transparent hover:bg-transparent">
      <EyeSlashIcon class="eyeIcon" />
    </button>
    </div>
    <div class="password-field">
      <div> <input :type="showPassword ? 'text' : 'password'" v-model="passwordField2" placeholder="Confirm Password"> </div>
      <button v-if="whichIcon" @click="switchVisibility(); switchIcon();" class="border-transparent hover:bg-transparent">
      <EyeIcon class="eyeIcon" />
    </button>
    <button v-else @click="switchVisibility(); switchIcon();" class="border-transparent hover:bg-transparent">
      <EyeSlashIcon class="eyeIcon" />
    </button>
    </div>
    <button class="submit-button" @click="register">Confirm</button>
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