<script setup>
import store from './store';
import { ref } from 'vue'
import { SignalIcon, MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
import { BellIcon, Cog8ToothIcon, UserCircleIcon, PlayCircleIcon, ChatBubbleBottomCenterTextIcon } from '@heroicons/vue/24/outline'

const search = ref('')
const openComments = ref(false)

</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <div>
        <button class="invisButton flex block items-center w-64 m-2">
          <UserCircleIcon class="h-10 mr-2"/>
          <div>
            {{store.user.username}}
          </div>
        </button>

        <button class="newpostButton flex block items-center justify-center w-64 m-2">
          <SignalIcon class="h-10 mr-2"/>
          <div class="hero">
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
      <div class="flex flex-col border-2 rounded-3xl border-[#1D1D2A] my-4 w-full justify-center h-full">
        <div class="flex border-2 rounded-3xl border-[#1D1D2A] mx-2 mt-2 mb-1 h-1/2">
            <div class="flex w-1/2 h-full items-center justify-center border-2 rounded-3xl border-[#1D1D2A]">
              <PlayCircleIcon class="w-1/2 h-1/2 text-[#1D1D2A]"/>
            </div>
        </div>
        <div class="flex border-2 rounded-3xl border-[#1D1D2A] mx-2 mt-1 mb-2 h-1/2">
          <div class="flex w-1/2 h-full items-center justify-center border-2 rounded-3xl border-[#1D1D2A]">
            <PlayCircleIcon class="w-1/2 h-1/2 text-[#1D1D2A]"/>
          </div>
          <div class="flex flex-col w-1/2">
            <div v-if="openComments">
              <div>
              these are allll the comments...
              </div>
              <button @click="openComments = !openComments">lol</button>
            </div>
            <div v-else class="flex flex-col h-full">
              <div class="flex h-1/3 items-center justify-center">
                <button class=" flex block invisButton w-full h-full items-center">
                  <UserCircleIcon class="h-14 mr-2"/>
                  <div>
                    {{store.user.username}}
                  </div>
                </button>
              </div>
              <div class="flex block h-1/6">
                <button @click="openComments = !openComments" class="flex block invisButton w-full h-full items-center">
                  <ChatBubbleBottomCenterTextIcon class="h-7 mr-2 ml-2 mt-2"/>
                  <div>
                    Comments
                  </div>
                </button>
              </div>
              <div class="flex h-3/6 ml-8 mt-2 ">
                <div class="text-[#828282]">
                  //Top Comment
                </div>
              </div>
            </div>
          </div>
          
        </div>
        <!-- <div class="flex border-2 rounded-3xl border-[#1D1D2A] mx-2 mt-2 justify-center h-1/2">
          Third Visible Post
        </div> -->
      </div>
    </div>
    <div class="flex w-1/4 mt-20 mb-4 items-center justify-center border-2 rounded-3xl border-[#1D1D2A]">
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
.invisButton{
  background: none;
  border: none;
  transition-duration: 0.5s;
}
</style>
