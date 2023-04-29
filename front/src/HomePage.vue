<script setup>
import { ref } from 'vue'
import { MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
import FeedVue from './components/FeedVue.vue'
import SideBar from './components/SideBar.vue'


const search = ref('')

</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <SideBar />
    </div>

    <div class="flex flex-col w-1/2 items-center">
      <div class="w-full">
        <div class="flex bg-[#1D1D2A] rounded-full h-12 mt-4 items-center">
          <MagnifyingGlassIcon class="h-7 ml-4 mr-4" />
          <input v-model="search" placeholder="Search" class="h-12 bg-transparent -ml-8 focus:outline-none">
        </div>
      </div>
      <FeedVue url="/api/posts/feed"/>
    </div>
  </div>
</template>

<style scoped>
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
