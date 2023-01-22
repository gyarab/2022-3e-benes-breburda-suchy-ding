<script setup>
import { ref } from 'vue'
import { notify } from '@kyvg/vue3-notification'

const passwordField1 = ref("")
const passwordField2 = ref("")
const email = ref("")
const username = ref("")
let showError = ref(false)
const showPassword = ref(false)



function switchVisibility(){
  showPassword.value = !showPassword.value;
}

function passwordMatch(){
  showError.value = passwordField1.value !== passwordField2.value
  if (passwordField1.value !== passwordField2.value) {
    notify({title: "password mismatch", type: "error"})
  }
}
</script>
<template>

    <p class="">Username: </p>
    <input v-model="username" placeholder="Your username here" />
    <p>Email: </p>
    <input v-model="email" placeholder="Your email here" />
    <p>Password:</p>
    <input :type="showPassword ? 'text' : 'password'" v-model="passwordField1" placeholder="Password">
    <input :type="showPassword ? 'text' : 'password'" v-model="passwordField2" placeholder="Confirm Password">
    <button @click="switchVisibility">show / hide</button>
    <button @click="passwordMatch">confirm</button>
</template>