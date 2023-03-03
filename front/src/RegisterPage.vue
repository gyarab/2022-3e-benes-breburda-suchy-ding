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
    <div class="grid grid-rows-9 gap-y-10">
      <div> <p class="">Username: </p> </div>
      <div> <input v-model="username" placeholder="Your username here" /> </div>
      <div> <p>Email: </p> </div>
      <div> <input v-model="email" placeholder="Your email here" /> </div>
      <div> <p>Password:</p> </div>
      <div> <input :type="showPassword ? 'text' : 'password'" v-model="passwordField1" placeholder="Password"> </div>
      <div> <input :type="showPassword ? 'text' : 'password'" v-model="passwordField2" placeholder="Confirm Password"> </div>
      <div> <button @click="switchVisibility">show / hide</button> </div>
      <div> <button @click="passwordMatch">confirm</button> </div>
    </div>



    <!-- <p class="">Username: </p>
    <input v-model="username" placeholder="Your username here" />
    <p>Email: </p>
    <input v-model="email" placeholder="Your email here" />
    <p>Password:</p>
    <input :type="showPassword ? 'text' : 'password'" v-model="passwordField1" placeholder="Password">
    <input :type="showPassword ? 'text' : 'password'" v-model="passwordField2" placeholder="Confirm Password">
    <button @click="switchVisibility">show / hide</button>
    <button @click="passwordMatch">confirm</button> -->
</template>