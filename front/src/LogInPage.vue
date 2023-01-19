<script setup>
import { ref } from 'vue'

function toHomePage(){
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
  <p>Username: {{ message }}</p>
  <input v-model="message" placeholder="Your username here" />
  <p>Password:</p>
  <input type="password" ref="passwordField" placeholder="Password">
  <button @click="switchVisibility">show / hide</button>
  <button @click="toHomePage">Log in</button>
</template> 
