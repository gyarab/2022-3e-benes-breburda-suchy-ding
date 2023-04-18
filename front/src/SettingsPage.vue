<script setup>
import { ref } from 'vue'
import { MagnifyingGlassIcon, ChevronLeftIcon } from '@heroicons/vue/20/solid'
import { UserCircleIcon, AdjustmentsHorizontalIcon, ArrowLeftCircleIcon, BellAlertIcon } from '@heroicons/vue/24/outline'

const userSettings = ref(false)
const profileSettings = ref(false)
const notificationsSettings = ref(false)

</script>

<template>
    <div class="flex flex-col items-center justify-center">
        <div>
            <div class="flex mt-20 items-center text-3xl font-bold">
                <button @click="$router.push('/')" class="iconButton p-0 mr-2">
                    <ArrowLeftCircleIcon class="ml-2 mr-6 h-10"/>
                </button>
                <h1>Settings Page</h1>
            </div>
            <div class="flex bg-[#1D1D2A] rounded-full h-12 mt-5 items-center">
                <MagnifyingGlassIcon class="h-7 ml-4 mr-4"/>
                <input v-model="search" placeholder="Search" class="h-12 bg-transparent -ml-8 focus:outline-none">
            </div>
        </div>

        <div class="flex flex-col mt-6 items-center">
            <div v-if="userSettings" class="flex">
                <h1>list of user settings</h1>
                <button @click="userSettings = !userSettings" class="flex rounded-full w-15 h-15 margin-center items-center">
                    <ChevronLeftIcon class="h-5 m-0"/>
                </button>
            </div>
            <div v-else-if="profileSettings" class="flex">
                <h1>list of profile settings</h1>
                <button @click="profileSettings = !profileSettings" class="flex rounded-full w-15 h-15 margin-center items-center">
                    <ChevronLeftIcon class="h-5 m-0"/>
                </button>
            </div>
            <div v-else-if="notificationsSettings" class="flex">
                <h1>list of notifications settings</h1>
                <button @click="notificationsSettings = !notificationsSettings" class="flex rounded-full w-15 h-15 margin-center items-center">
                    <ChevronLeftIcon class="h-5 m-0"/>
                </button>
            </div>
            <div v-else>
                <button @click="userSettings = !userSettings" class="invisButton flex block w-64 items-center m-2">
                    <UserCircleIcon class="h-6 mr-2"/>
                    <div>
                        User Settings
                    </div> 
                </button>
                <button @click="profileSettings = !profileSettings" class="invisButton flex block w-64 items-center m-2">
                    <AdjustmentsHorizontalIcon class="h-6 mr-2"/>
                    <div>   
                        Profile Settings
                    </div> 
                </button>
                <button @click="notificationsSettings = !notificationsSettings" class="invisButton flex block w-64 items-center m-2">
                    <BellAlertIcon class="h-6 mr-2"/>
                    <div>
                        Notifications Settings
                    </div> 
                </button>
            </div>
        </div>
    </div>
</template>

<style>
.iconButton {
  background: none;
  border: none;
  transition-duration: 0.5s;
}
.iconButton:hover{
  background: none;
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
</style>