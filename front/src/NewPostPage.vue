<script setup>
import { PlayIcon, PauseIcon, MusicalNoteIcon } from '@heroicons/vue/20/solid'
import SideBar from './components/SideBar.vue'
    
</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <SideBar />
    </div>

    <div class="flex flex-col w-1/2 items-center">
      <div class="flex flex-col w-full h-full">
        <div class="flex text-3xl mb-4 font-bold h-16 justify-center">
          <h1 class="mt-6">New Post</h1>
        </div>
        <div class="w-full h-12 border-2 rounded-3xl border-[#1D1D2A] items-center justify-center">
          tady budou nejaky ty nuzky na trimming or sm, nejak zejtra vymyslim jak na to hah
        </div>
        <div class="w-full h-2/5 border-2 rounded-3xl border-[#1D1D2A]"></div>
          <div class="w-full flex h-13 mt-2">
            <div class="flex w-1/2 h-13 items-center justify-start">
              <button class="flex playPauseButton mr-1 items-center justify-center">
                <PlayIcon class="h-6 w-6 p-0" />
              </button>
              <button class="flex playPauseButton mr-1 items-center justify-center">
                <PauseIcon class="h-6 w-6 p-0" />
              </button>
              <button class="uploadAudioButton flex items-center justify-center">
                <MusicalNoteIcon class="h-6 mr-2 p-0" />
                <div>
                  Upload
                </div>
              </button>
            </div>
            <div class="flex w-1/2 h-13 items-center justify-end">
              <button class="newpostButton flex items-center justify-center">
                <div>
                  Post
                </div>
              </button>
            </div>
          </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.uploadAudioButton {
  background-color: #1D1D2A;
  border: none;
  transition-duration: 1s;
}
.uploadAudioButton:hover {
  background-color: #aa0000;
  border: none;
}
.playPauseButton {
  background-color: #1D1D2A;
  border: none;
  transition-duration: 1s;
}
.playPauseButton:hover {
  background-color: #6b21a8;
  border: none;
}
</style>