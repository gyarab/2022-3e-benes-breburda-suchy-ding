<script setup>
import { SignalIcon } from '@heroicons/vue/20/solid'
import { Cog8ToothIcon, ArrowLeftCircleIcon, HomeIcon, InboxArrowDownIcon } from '@heroicons/vue/24/outline'
import { store } from '../store'
import ProfilePic from './ProfilePic.vue'

function logout() {
  window.localStorage.removeItem('api-key');
  location.reload();
}
</script>

<template>

    <div>
        <button @click="$router.push('/user/me')" class="invisButton flex items-center w-64 m-2">
        <ProfilePic user="me" size="10"/>
        <div class="ml-2">
            {{ store.user.name }}
        </div>
        </button>

        <button @click="$router.push('/new-post')" class="newpostButton flex items-center justify-center w-64 m-2">
        <SignalIcon class="h-10 mr-2" />
        <div class="hero">
            New Post
        </div>
        </button>

        <button @click="$router.push('/')" class="invisButton flex w-64 items-center m-2">
        <HomeIcon class="h-6 mr-2" />
        <div>
            Home
        </div>
        </button>

        <button @click="$router.push('/saved')" class="invisButton flex w-64 items-center m-2">
        <InboxArrowDownIcon class="h-6 mr-2" />
        <div>
            Saved Posts
        </div>
        </button>

        <button @click="$router.push('/settings')" class="invisButton flex  w-64 items-center m-2">
        <Cog8ToothIcon class="h-6 mr-2" />
        <div>
            Settings
        </div>
        </button>

        <button @click="logout" class="invisButton flex w-64 items-center m-2">
        <ArrowLeftCircleIcon class="h-6 mr-2" />
        <div>
            Log Out
        </div>
        </button>
  </div>
</template>
<style scoped>
.newpostButton {
  background-color: #6b21a8;
  border: none;
  transition-duration: 1s;
}

.newpostButton:hover {
  background-color: #9333ea;
  border: none;
}
.invisButton {
  background: none;
  border: none;
  transition-duration: 0.3s;
}
.invisButton:hover {
  background-color: #1D1D2A;
  border: none;
}
</style>
