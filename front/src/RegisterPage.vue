<script setup>
import { ref } from 'vue'
import { notify } from '@kyvg/vue3-notification'
import { EyeIcon, EyeSlashIcon } from '@heroicons/vue/20/solid'


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
    showError.value = passwordField1.value !== passwordField2.value
    if (passwordField1.value !== "") {
      if (passwordField1.value !== passwordField2.value) {
        notify({title: "password mismatch", type: "error"})
      } else {window.location.hash = '#/profilesetup'}
    }
  }

  function switchIcon() {
  whichIcon.value = !whichIcon.value;
}

</script>
<template>
    <div class="container">
      <div> <input v-model="username" placeholder="Username" /> </div>
      <div> <input v-model="email" placeholder="Email" /> </div>
      <div class="password-field">
        <div> <input :type="showPassword ? 'text' : 'password'" v-model="passwordField1" placeholder="Password"> </div>
        <button v-if="whichIcon" @click="switchVisibility(); switchIcon();">
        <EyeIcon class="icon" />
      </button>
      <button v-else @click="switchVisibility(); switchIcon();">
        <EyeSlashIcon class="icon" />
      </button>
      </div>
      <div class="password-field">
        <div> <input :type="showPassword ? 'text' : 'password'" v-model="passwordField2" placeholder="Confirm Password"> </div>
        <button v-if="whichIcon" @click="switchVisibility(); switchIcon();">
        <EyeIcon class="icon" />
      </button>
      <button v-else @click="switchVisibility(); switchIcon();">
        <EyeSlashIcon class="icon" />
      </button>
      </div>
      <button class="submit-button" @click="passwordMatch">Confirm</button>
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