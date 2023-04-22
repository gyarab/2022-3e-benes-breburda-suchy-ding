<script setup>
import { ref } from 'vue'
import { BellAlertIcon, InboxArrowDownIcon, ChevronLeftIcon, ChevronRightIcon } from '@heroicons/vue/20/solid'
import { ChatBubbleBottomCenterTextIcon, PlayCircleIcon, UserCircleIcon, BellIcon, InboxIcon } from '@heroicons/vue/24/outline';

const openComments = ref(false)
const addCommentActive = ref(false)
const isDinged = ref(false)
const isSaved = ref(false)


</script>

<template>
    <div class="flex border-2 rounded-3xl border-[#1D1D2A] mb-2 h-1/2">
        <div class="flex w-1/2 m-2 items-center justify-center">
            <PlayCircleIcon class="w-1/2 h-1/2 text-[#1D1D2A]"/>
        </div>
        <div class="flex flex-col w-1/2">
            <div v-if="openComments" class="h-full">
                <div class="relative h-full">
                    <div class="flex mt-2 items-center">
                    <button class="iconButton w-8 h-8 p-0">
                        <ChevronLeftIcon @click="openComments = !openComments" class="h-7"/>
                    </button>
                    <div class="text-xl">
                        Comments
                    </div>
                    </div>
                    <div class="flex items-center border-2 border-[#1D1D2A]">
                        the comments section
                    </div>
                    <div class="flex w-full absolute bottom-0">
                        <div class="flex w-full bg-[#1D1D2A] mb-4 mr-2 rounded-2xl">
                            <input id="JanLana" @focus="addCommentActive = !addCommentActive" @blur="addCommentActive = !addCommentActive" class="flex w-5/6 bg-transparent text-base focus:outline-none placeholder:italic" placeholder="Add your thoughts...">
                            <div v-if="addCommentActive" class="flex items-center">
                                <ChevronRightIcon @click="openComments = !openComments" class="ml-3 h-7 p-0"/>
                            </div>
                        </div>
                    </div>
                </div>
                
            </div>
            <div v-else class="flex flex-col h-full">
                <div class="flex h-1/4 mt-4 items-center justify-center">
                <button class=" flex block invisButton w-full h-full items-center">
                  <UserCircleIcon class="h-14 mr-2"/>
                     <div>
                      Userboi
                     </div>
                    </button>
                </div>
                <div class="flex h-1/4 ml-8 mt-2 ">
                    <div class="text-[#828282]">
                        <p>//Top Comment</p>
                        <p>"Ur mum's a slag."</p>
                    </div>
                </div>
                <div class="flex h-1/6">
                    <button v-if="isDinged" class="iconButton ml-8 mr-1 w-8 h-8 p-0 text-[#6b21a8]">
                        <BellAlertIcon @click="isDinged = !isDinged" class="h-7 ml--4"/>
                    </button>
                    <button v-else class="iconButton ml-8 mr-1 w-8 h-8 p-0">
                        <BellIcon @click="isDinged = !isDinged" class="h-7 ml--4"/>
                    </button>
                    <button class="iconButton mr-1.5 w-8 h-8 p-0">
                        <ChatBubbleBottomCenterTextIcon  @click="openComments = !openComments" class="h-7"/>
                    </button>
                    <button v-if="isSaved" class="iconButton mr-1 w-8 h-8 p-0 text-[##c70a00]">
                        <InboxArrowDownIcon @click="isSaved = !isSaved" class="h-7 ml--4"/>
                    </button>
                    <button v-else class="iconButton mr-1 w-8 h-8 p-0">
                        <InboxIcon @click="isSaved = !isSaved" class="h-7 ml--4"/>
                    </button>
                </div>
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
</style>
