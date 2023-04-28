<script setup>
import { store } from './store';
import { ref } from 'vue'
import { SignalIcon, MagnifyingGlassIcon, PlayIcon, PauseIcon, MusicalNoteIcon } from '@heroicons/vue/20/solid'
import { Cog8ToothIcon, UserCircleIcon, ArrowLeftCircleIcon, HomeIcon, InboxArrowDownIcon } from '@heroicons/vue/24/outline'
import FeedVue from './components/FeedVue.vue'


const search = ref('')
const savedPosts = ref(false)
const profileView = ref(false)
const isListeningTo = ref(false)
const newPost = ref(false)

function logout() {
  window.localStorage.removeItem('api-key');
  location.reload();
}
function homeClick() {
  if (savedPosts.value === true || profileView.value === true || newPost.value === true) {
    savedPosts.value = false;
    profileView.value = false;
    newPost.value = false;
  }
}
function profileClick() {
  if (profileView.value === false) {
    profileView.value = true;
  }
  if (savedPosts.value === true) {
    savedPosts.value = false;
  }
  if (newPost.value === true) {
    newPost.value = false;
  }
}
function savedClick() {
  if (savedPosts.value === false) {
    savedPosts.value = true;
  }
  if (profileView.value === true) {
    profileView.value = false;
  }
  if (newPost.value === true) {
    newPost.value = false;
  }
}
function postClick() {
  if (newPost.value === false) {
    newPost.value = true;
  }
  if (savedPosts.value === true) {
    savedPosts.value = false;
  }
  if (profileView.value === true) {
    profileView.value = false;
  }
}



</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <div>
        <button @click="profileClick" class="invisButton flex items-center w-64 m-2">
          <UserCircleIcon class="h-10 mr-2" />
          <div>
            {{ store.user.name }}
          </div>
        </button>

        <button @click="postClick" class="newpostButton flex items-center justify-center w-64 m-2">
          <SignalIcon class="h-10 mr-2" />
          <div class="hero">
            New Post
          </div>
        </button>

        <button @click="homeClick" class="invisButton flex w-64 items-center m-2">
          <HomeIcon class="h-6 mr-2" />
          <div>
            Home
          </div>
        </button>

        <button @click="savedClick" class="invisButton flex w-64 items-center m-2">
          <InboxArrowDownIcon class="h-6 mr-2" />
          <div>
            Saved Posts
          </div>
        </button>

        <button @click="$router.push({ path: '/settings' })" class="invisButton flex  w-64 items-center m-2">
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
    </div>

    <div v-if="savedPosts" class="flex flex-col w-1/2 items-center">
      <div class="w-full">
        <div class="flex text-3xl font-bold h-16 justify-center">
          <h1 class="mt-6">Saved Posts</h1>
        </div>
      </div>
      <FeedVue url="/api/posts/saved"/>
    </div>

    <div v-else-if="profileView" class="flex flex-col w-1/2 items-center">
      <div class="w-full">
        <div class="flex flex-col w-full text-3xl font-bold justify-center">
          <div class="flex flex-col mt-6 w-full justify-center">
            <UserCircleIcon class="h-20" />
            <div class="flex w-full justify-center">
              {{ store.user.name }}
            </div>
          </div>
          <div class="flex w-full justify-center text-[#828282] mt-3">
            <p class="flex w-1/2 text-sm text-center">
              We've known each other for so long. Your heart's been aching, but you're too shy to say it.
            </p>
          </div>
          <div class="flex justify-center mt-6">
            <div class="flex flex-col justify-center text-base">
              <p class="flex justify-center">100</p>
              <p class="flex justify-center font-normal">Listeners</p>
            </div>
            <div>
              <button @click="isListeningTo = !isListeningTo"
                class="flex mx-6 w-35 h-8 items-center justify-center newpostButton ">
                <p v-if="isListeningTo" class="text-sm">Listening</p>
                <p v-else class="text-sm">Listen</p>
              </button>
            </div>
            <div class="flex flex-col justify-center text-base">
              <p class="flex justify-center">10</p>
              <p class="flex justify-center font-normal">Listening</p>
            </div>
          </div>
        </div>
      </div>
      <div class="my-4 w-full overflow-y-scroll hidescrollbar">
        <PostVue />
        <PostVue />
        <PostVue />
      </div>
    </div>

    <div v-else-if="newPost" class="flex flex-col w-1/2 items-center">
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

    <div v-else class="flex flex-col w-1/2 items-center">
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

<style>
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
