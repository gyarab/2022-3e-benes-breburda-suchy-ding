<script setup>
import { ref } from 'vue'

function toHomePage(){
  window.location.hash = '#/'
}

function toRegisterPage(){
  window.location.hash = '#/register'
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
  <div class="grid grid-rows-8">
    <div> <p>Username: {{ message }}</p> </div>
    <div> <input v-model="message" placeholder="Your username here" class="input_box" /> </div>
    <div> <p>Password:</p> </div>
    <div> <input type="password" ref="passwordField" placeholder="Password"> </div>
    <div> <button @click="switchVisibility">show / hide</button> </div>
    <div> <button @click="toHomePage">Log in</button> </div>
    <div> <p>Not a user yet?</p> </div>
    <div> <button @click="toRegisterPage">Register</button> </div>
  </div>


  <!-- <p>Username: {{ message }}</p>
  <input v-model="message" placeholder="Your username here" class="input_box" />
  <p>Password:</p>
  <input type="password" ref="passwordField" placeholder="Password">
  <button @click="switchVisibility">show / hide</button>
  <button @click="toHomePage">Log in</button>
  <p>Not a user yet?</p>
  <button @click="toRegisterPage">Register</button> -->
</template> 
