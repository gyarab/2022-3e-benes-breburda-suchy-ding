<script setup>
import { store } from './store';
import { ref, onBeforeMount } from 'vue'
import rest from './rest'
import SideBar from './components/SideBar.vue'
import ProfilePic from './components/ProfilePic.vue'
import FeedVue from './components/FeedVue.vue'
import { capture_err } from './notify'

const props = defineProps({ user: String })
const user = ref({})
const isFound = ref(false)

const isListeningTo = ref(false)
const isMe = props.user == 'me' || props.user == store.user.user_id

async function listen() {
  if (isListeningTo.value) {
    capture_err(await rest.del(`/api/users/${props.user}/listen`))
    user.value.listeners.splice(user.value.listeners.findIndex(u => u.user_id == store.user.user_id), 1)
    isListeningTo.value = !isListeningTo.value
  } else {
    capture_err(await rest.post(`/api/users/${props.user}/listen`))
    user.value.listeners.push(store.user)
    isListeningTo.value = !isListeningTo.value
  }
}

onBeforeMount(async () => {
  const resp = await rest.get(`/api/users/${props.user}`)
  isFound.value = resp.status == 200
  if (!isFound.value) return;
  user.value = resp.body

  isListeningTo.value = user.value.listeners.some(l => l.user_id == store.user.user_id)
})

</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <SideBar />
    </div>

    <div v-if="isFound" class="flex flex-col w-1/2 items-center">
      <div class="w-full">
        <div class="flex flex-col w-full text-3xl font-bold justify-center items-center">
          <div class="flex flex-col mt-6 w-full justify-center items-center">
            <ProfilePic :user="user.user_id" class="h-20 w-20" />
            <div class="flex w-full justify-center">
              {{ user.name }}
            </div>
          </div>
          <p class="w-1/2 text-sm text-center text-[#828282] mt-3">
            {{ user.bio || `${user.name} wants to keep a cloud of mistery around them...` }}
          </p>
          <div class="flex justify-center mt-6">
            <div class="flex flex-col justify-center text-base">
              <p class="flex justify-center">{{ user.listeners.length }}</p>
              <p class="flex justify-center font-normal">Listeners</p>
            </div>
            <div>
              <button v-if="!isMe" @click="listen" class="flex mx-6 w-35 h-8 items-center justify-center newpostButton ">
                <p v-if="isListeningTo" class="text-sm">Listening</p>
                <p v-else class="text-sm">Listen</p>
              </button>
              <div v-else class="w-5"></div>
            </div>
            <div class="flex flex-col justify-center text-base">
              <p class="flex justify-center">{{ user.listening.length }}</p>
              <p class="flex justify-center font-normal">Listening</p>
            </div>
          </div>
        </div>
      </div>
      <FeedVue :url="`/api/users/${user.user_id}/posts`" />
    </div>
    <div v-else>
      <p>User was not found</p>
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
