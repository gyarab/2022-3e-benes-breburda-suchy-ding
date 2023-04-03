<script setup>
import store from './store';
import { ref } from 'vue'
import { SignalIcon, MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
import { BellIcon, Cog8ToothIcon, UserCircleIcon} from '@heroicons/vue/24/outline'

const search = ref('')

</script>

<template>
  <div class="flex">
    <div class="flex flex-col w-1/4 items-center">
      <div>
        <button class="userContainer flex block items-center w-64 m-2">
          <UserCircleIcon class="h-10 mr-2"/>
          <div>
            {{store.user.username}}
          </div>
        </button>

        <button class="newpostButton flex block items-center justify-center w-64 m-2">
          <SignalIcon class="h-10 mr-2"/>
          <div>
            New Post
          </div>
        </button>

        <button class="sidebarButton flex block w-64 items-center m-2">
          <BellIcon class="h-6 mr-2"/>
          <div>
            Notifications
          </div> 
        </button>

        <button @click="$router.push({ path: '/settings' })" class="sidebarButton flex block w-64 items-center m-2">
          <Cog8ToothIcon class="h-6 mr-2"/>
          <div>
            Settings
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
      <div class="flex border-2 rounded-3xl border-[#1D1D2A] mt-4 w-full items-center justify-center">
        Main Feed
      </div>
    </div>

    <div class="flex w-1/4 mt-20 items-center justify-center border-2 rounded-3xl border-[#1D1D2A]">
      Sidebar
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
.sidebarButton{
  background: none;
  border: none;
  transition-duration: 0.5s;
}
.userContainer{
  background: none;
  border: none;
  transition-duration: 0.5s;
}
</style>
