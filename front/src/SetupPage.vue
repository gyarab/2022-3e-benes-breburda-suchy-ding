<script setup>
import { ref } from 'vue'
import SetUp from './components/SetUp.vue'

const firstPage = ref(true)

</script>

<template>
    <div class="flex h-screen w-screen">
        <div v-if="firstPage" class="flex flex-col h-full w-full justify-center items-center">
            <h1 class="text-3xl font-bold">Hello, welcome to Ding!</h1>
            <h2 class="text-xl font-bold mt-2 mb-10 text-[#828282]">Let's get you sorted...</h2>
            <button @click="firstPage = !firstPage" class="continueButton flex w-64 justify-center items-center">
                Continue
            </button>
        </div>
        <div v-else class="flex h-full w-full justify-center">
            <div class="flex flex-col w-1/2 justify-center items-center">
                <h1 class="text-3xl font-bold">Set yourself up!</h1>
                <h2 class="text-xl font-bold mt-2 mb-10 text-[#828282]">it will only take a minute...</h2>
            </div>
            <div class="flex flex-col w-1/2 justify-center">
                <SetUp @submit="$router.push('/')" submitButtonText="Confirm" class="w-1/2" />
            </div>
        </div>
    </div>
</template>

<style>
.custom-file-input[type="file"] {
    display: none;
}

.continueButton {
    background-color: #1D1D2A;
    border: none;
    transition-duration: 1s;
}

.continueButton:hover {
    background-color: #6b21a8;
    border: none;
}
</style>