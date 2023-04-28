<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const firstPage = ref(true)

function setupDone() {
    router.push({ path: '/' })
}

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
                <h2 class="text-xl font-bold mt-2 mb-10 text-[#828282]">you little wanking cunt...</h2>
            </div>

            <div class="flex flex-col w-1/2 justify-center">
                <div class="flex w-1/2 mb-4">
                    <div class="flex w-24 h-24 border-2 border-[#1D1D2A] rounded-full"></div>
                    <div class="flex flex-grow justify-center items-center">
                        <button class="continueButton flex flex-grow ml-4 justify-center items-center">
                            Upload Photo
                        </button>
                    </div>
                </div>
                <div class="flex w-1/2 border-2 rounded-3xl border-[#1D1D2A] mb-4">
                    <input class="flex bg-transparent text-base focus:outline-none placeholder:italic"
                        placeholder="This will be your Bio..." style="color: #9fa5b3; font-style: italic;">
                </div>
                <button @click="setupDone" class="continueButton flex w-1/2 mt-2 justify-center items-center">
                    Continue
                </button>

            </div>
        </div>
    </div>
</template>

<style>
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