<script setup>
import { UserCircleIcon } from '@heroicons/vue/24/outline'
import { ref } from 'vue'
import { store } from '../store'
import rest from '../rest'
const props = defineProps({user: String})
const isDefault = ref(false)

function showDefault() {
    isDefault.value = true
}
</script>

<template>
    <div class="flex w-10 h-10 border-2 border-[#1D1D2A] rounded-full overflow-hidden items-center justify-center">
        <UserCircleIcon v-if="isDefault" class="h-full"></UserCircleIcon>
        <img v-else class="w-full h-full object-cover" :src="rest.get_url(`/api/users/${props.user === 'me' ? store.user.user_id : props.user}/profile.jpg`)" alt="user profile picture" @error="showDefault">
    </div>
</template>

<style>
</style>