<script setup>
import { store } from './store';
import { ref } from 'vue'
import { SignalIcon, MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
import { BellIcon, Cog8ToothIcon, UserCircleIcon, ArrowLeftCircleIcon } from '@heroicons/vue/24/outline'
import PostVue from './components/PostVue.vue'

const search = ref('')

function logout() {
  window.localStorage.removeItem('api-key');
  location.reload();
}

</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <div>
        <button class="invisButton flex block items-center w-64 m-2">
          <UserCircleIcon class="h-10 mr-2"/>
          <div>
            {{store.user.name}}
          </div>
        </button>

        <button class="newpostButton flex block items-center justify-center w-64 m-2">
          <SignalIcon class="h-10 mr-2"/>
          <div class="hero">
            New Post
          </div>
        </button>

        <button class="invisButton flex block w-64 items-center m-2">
          <BellIcon class="h-6 mr-2"/>
          <div>
            Notifications
          </div> 
        </button>

        <button @click="$router.push({ path: '/settings' })" class="invisButton flex block w-64 items-center m-2">
          <Cog8ToothIcon class="h-6 mr-2"/>
          <div>
            Settings
          </div> 
        </button>

        <button @click="logout" class="invisButton flex block w-64 items-center m-2">
          <ArrowLeftCircleIcon class="h-6 mr-2"/>
          <div>
            Log Out
          </div> 
        </button>
      </div>
    </div>

    <div class="flex flex-col w-1/2 items-center">
      <div class="w-full">
        <div class="flex bg-[#1D1D2A] rounded-full h-12 mt-4 items-center">
          <MagnifyingGlassIcon class="h-7 ml-4 mr-4"/>
          <input v-model="search" placeholder="Search" class="h-12 bg-transparent -ml-8 focus:outline-none">
        </div>
      </div>
      <div class="my-4 w-full h-full overflow-y-scroll hidescrollbar">
        <PostVue />
        <PostVue />
        <PostVue />
        <PostVue />
        <PostVue />
        <PostVue />
        <PostVue />
        <PostVue />
        <PostVue />
      </div>
    </div>
    <div class="flex w-1/4 mt-20 mb-4 items-center justify-center">
    </div>
  </div>
</template>

<style>
.newpostButton{
  background-color: #6b21a8;
  border: none;
  transition-duration: 0.5s;
}
.newpostButton:hover{
  background-color: #9333ea;
  border: none;
}
.invisButton{
  background: none;
  border: none;
  transition-duration: 0.5s;
}
.invisButton:hover{
  background-color: #1D1D2A;
  border: none;
}
.hidescrollbar {
    -ms-overflow-style: none; /* for Internet Explorer, Edge */
    scrollbar-width: none; /* for Firefox */
    overflow-y: scroll; 
}
.hidescrollbar::-webkit-scrollbar {
    display: none; /* for Chrome, Safari, and Opera */
}
</style>
