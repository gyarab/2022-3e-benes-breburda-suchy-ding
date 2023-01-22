<script setup>
import { ref } from 'vue'

const passwordField1 = ref(null)
const passwordField2 = ref(null)



function switchVisibility(){
  console.log(passwordField1.value.getAttribute('type'))
  console.log(passwordField2.value.getAttribute('type'))

  if (passwordField1.value.getAttribute('type') === 'password'){
    passwordField1.value.setAttribute('type', 'text')
  }
  else {
    passwordField1.value.setAttribute('type', 'password')
  }

  if (passwordField2.value.getAttribute('type') === 'password'){
    passwordField2.value.setAttribute('type', 'text')
  }
  else {
    passwordField2.value.setAttribute('type', 'password')
  }
}

function confirm(){
    if (passwordField1.value !== passwordField2.value){
        console.log('uwu')
    }
}
</script>

<template>

    <p>Username: </p>
    <input v-model="username" placeholder="Your username here" />
    <p>Email: </p>
    <input v-model="email" placeholder="Your email here" />
    <p>Password:</p>
    <input type="password" ref="passwordField1" placeholder="Password">
    <input type="password" ref="passwordField2" placeholder="Confirm Password">
    <button @click="switchVisibility">show / hide</button>
    <button @click="confirm">confirm</button>

    <vue-basic-alert
        :duration="300" 
        :closeIn="300"
       ref="alert" />
</template>