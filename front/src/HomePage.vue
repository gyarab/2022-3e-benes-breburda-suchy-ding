<script setup>
import { ref } from 'vue'
import { MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
import FeedVue from './components/FeedVue.vue'
import SideBar from './components/SideBar.vue'
import ProfilePic from './components/ProfilePic.vue'
import rest from './rest'
import { capture_err } from './notify'


const search = ref('')

const searchResults = ref([])
let timeout = null

async function doSearch() {
  clearTimeout(timeout)
  if (search.value.length < 2) {
    searchResults.value = []
    return
  }
  timeout = setTimeout(async () => {
    const results = capture_err(await rest.get(`/api/users/search?search=${encodeURI(search.value)}`))
    searchResults.value = results.body
  }, 500)
}

</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <SideBar />
    </div>

    <div class="flex flex-col w-1/2 items-center">
      <div class="w-full rounded-xl bg-[#1D1D2A] mt-4">
        <div class="flex h-12 items-center">
          <MagnifyingGlassIcon class="h-7 ml-4 mr-4" />
          <input v-model="search" @input="doSearch" placeholder="Search"
            class="block h-12 bg-transparent -ml-8 focus:outline-none">
        </div>
        <div class="m-4" v-if="searchResults.length > 0">
          <div v-for="i in searchResults" :key="i.user_id"
            class="flex items-center hover:bg-[#2D2D3A] p-2 rounded-xl cursor-default"
            @click="$router.push(`/user/${i.user_id}`)">
            <ProfilePic :user="i.user_id" class="w-10 h-10" />
            <p class="ml-4">{{ i.name }}</p>
          </div>
        </div>
      </div>
      <FeedVue url="/api/posts/feed" :endless=true />
    </div>
  </div>
</template>

<style scoped></style>
