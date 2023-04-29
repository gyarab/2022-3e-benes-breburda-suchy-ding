<script setup>
import { ref } from 'vue'
import rest from '../rest'
import { notify } from '../notify'
import { store } from '../store'
import ProfilePic from './ProfilePic.vue'

const props = defineProps({ submitButtonText: String })
const emit = defineEmits(['submit'])

const selectedImage = ref(null);
const imageSrc = ref(null)
const bioInput = ref(store.user.bio)


function onImageSelected(event) {
    selectedImage.value = event.target.files[0]
    imageSrc.value = URL.createObjectURL(event.target.files[0])
}

async function setupDone() {
    await rest.patch('/api/users/me', {
        bio: bioInput.value
    });
    store.user.bio = bioInput.value
    if (selectedImage.value) await rest.put('/api/users/me/profile.jpg', new Blob([selectedImage.value]))
    emit('submit')
    notify('info', 'Saved!')
}

</script>

<template>
    <div class="flex flex-col justify-center">
        <div class="flex mb-4">
            <div v-if="imageSrc"
                class="flex w-24 h-24 border-2 border-[#1D1D2A] rounded-full overflow-hidden items-center justify-center">
                <img class="w-full h-full object-cover" :src="imageSrc" alt="Image description">
            </div>
            <ProfilePic v-else :user="store.user.user_id" class="w-20 h-20" />
            <div class="flex flex-grow justify-center items-center">
                <label for="fileInput"
                    class="continueButton text-base flex w-full h-1/2 ml-4 justify-center items-center rounded-full cursor-pointer">
                    Upload image
                </label>
                <input type="file" id="fileInput" @change="onImageSelected" class="custom-file-input">
            </div>
        </div>
        <div class="flex border-2 rounded-3xl border-[#1D1D2A] mb-4">
            <input class="flex bg-transparent text-base focus:outline-none placeholder:italic"
                placeholder="This will be your Bio..." v-model="bioInput" style="color: #9fa5b3; font-style: italic;">
        </div>
        <div class="flex">
            <button @click="setupDone" class="continueButton flex mt-2 justify-center items-center flex-1">
                {{ props.submitButtonText }}
            </button>
            <slot />
        </div>
    </div>
</template>

<style></style>