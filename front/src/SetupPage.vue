<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import rest from './rest'

const router = useRouter()
const firstPage = ref(true)
const selectedImage = ref(null);
const imageSrc = ref("/imgs/JanLanaPP.jpg")
const bioInput = ref("")
const onImageSelected = (event) => {
  selectedImage.value = event.target.files[0]
  imageSrc.value = URL.createObjectURL(event.target.files[0])
}

async function setupDone() {
    await rest.patch('/api/users/me', {
        bio: bioInput.value
    });
    await rest.put('/api/users/me/profile.jpg', new Blob([selectedImage.value]))
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
                    <div class="flex w-24 h-24 border-2 border-[#1D1D2A] rounded-full overflow-hidden items-center justify-center">
                        <img class="w-full h-full object-cover" :src="imageSrc" alt="Image description">
                    </div>
                    <div class="flex flex-grow justify-center items-center">
                        <label for="fileInput" class="continueButton text-base flex w-full h-1/2 ml-4 justify-center items-center rounded-full cursor-pointer">
                            Upload image
                        </label>
                        <input type="file" id="fileInput" @change="onImageSelected" class="custom-file-input">
                    </div>
                </div>
                <div class="flex w-1/2 border-2 rounded-3xl border-[#1D1D2A] mb-4">
                    <input class="flex bg-transparent text-base focus:outline-none placeholder:italic"
                        placeholder="This will be your Bio..." v-model="bioInput" style="color: #9fa5b3; font-style: italic;">
                </div>
                <button @click="setupDone" class="continueButton flex w-1/2 mt-2 justify-center items-center">
                    Continue
                </button>
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