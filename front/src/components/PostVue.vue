<script setup>
import { ref } from 'vue'
import { ChatBubbleBottomCenterTextIcon, PlayCircleIcon, UserCircleIcon } from '@heroicons/vue/24/outline';
import { store } from '../store';
const openComments = ref(false)
</script>

<template>
    <div class="flex border-2 rounded-3xl border-[#1D1D2A] mx-2 mt-1 mb-2 h-1/2">
        <div class="flex w-1/2 items-center justify-center border-2 rounded-3xl border-[#1D1D2A]">
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
</template>

<style>

</style>