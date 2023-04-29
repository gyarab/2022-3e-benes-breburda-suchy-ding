<script setup>
import FeedVue from './components/FeedVue.vue'
import SideBar from './components/SideBar.vue'
</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <SideBar />
    </div>

    <div class="flex flex-col w-1/2 items-center">
      <div class="w-full">
        <div class="flex text-3xl font-bold h-16 justify-center">
          <h1 class="mt-6">Saved Posts</h1>
        </div>
      </div>
      <FeedVue url="/api/posts/saved"/>
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
