<script setup>
import FeedVue from './components/FeedVue.vue'
import SideBar from './components/SideBar.vue'
import { store } from './store'
</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <SideBar />
    </div>

    <div class="flex flex-col w-1/2 items-center">
      <h1 class="text-3xl mt-6 font-bold">Saved Posts</h1>
      <p class="text-xl text-gray-500">{{ store.user.name }}</p>
      <FeedVue url="/api/posts/saved" />
    </div>
  </div>
</template>

<style>
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
