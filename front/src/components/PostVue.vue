<script setup>
import { ref, onBeforeMount } from 'vue'
import { BellAlertIcon, InboxArrowDownIcon, ChevronLeftIcon, ChevronRightIcon } from '@heroicons/vue/20/solid'
import { ChatBubbleBottomCenterTextIcon, BellIcon, InboxIcon } from '@heroicons/vue/24/outline';
import CommentVue from './CommentVue.vue'
import AudioPlayer from './AudioPlayer.vue'
import rest from '../rest'

const props = defineProps({ post: Object })
const openComments = ref(false)
const addCommentActive = ref(false)
const isDinged = ref(props.post.liked)
const isSaved = ref(props.post.saved)
const author = ref({})
const comments = ref([])
const commentInput = ref("")

function postChevron() {
    addCommentActive.value = commentInput.value.length > 0
}

async function saving() {
    if (isSaved.value) {
        await rest.del('/api/posts/' + props.post.post_id + '/save')
    }
    else {
        await rest.post('/api/posts/' + props.post.post_id + '/save')
    }
    isSaved.value = !isSaved.value
}

async function dinging() {
    if (isDinged.value) {
        await rest.del('/api/posts/' + props.post.post_id + '/like')
    }
    else {
        await rest.post('/api/posts/' + props.post.post_id + '/like')
    }
    isDinged.value = !isDinged.value
}

onBeforeMount(async () => {
    author.value = (await rest.get('/api/users/' + props.post.author_id)).body
    comments.value = (await rest.get('/api/posts/' + props.post.post_id + '/comments')).body
})

</script>

<template>
    <div v-if="openComments" class="flex border-2 rounded-3xl border-[#1D1D2A] mb-2 h-1/2">
        <div class="flex w-1/2 m-2 items-center justify-center">
            <AudioPlayer :url="'/api/posts/' + props.post.post_id + '/content'" />
        </div>
        <div class="flex flex-col w-1/2">
            <div class="h-full">
                <div class="relative h-full">
                    <div class="flex mt-2 items-center">
                        <button class="iconButton w-8 h-8 p-0">
                            <ChevronLeftIcon @click="openComments = !openComments" class="h-7" />
                        </button>
                        <div class="text-xl">
                            Comments
                        </div>
                    </div>
                    <div class="w-full h-1/2">
                        <div class="overflow-y-scroll h-full mr-2 border-2 rounded-3xl border-[#1D1D2A] hidescrollbar">
                            <CommentVue v-for="i in comments" :comment="i" :key="i.comment_id" />
                        </div>
                    </div>
                    <div class="flex w-full absolute bottom-0">
                        <div class="flex w-full bg-[#1D1D2A] mb-4 mr-2 rounded-2xl">
                            <input v-model="commentInput" @input="postChevron"
                                class="flex w-5/6 bg-transparent text-base focus:outline-none placeholder:italic"
                                placeholder="Add your thoughts...">
                            <div v-if="addCommentActive" class="flex items-center">
                                <ChevronRightIcon @click="openComments = !openComments" class="ml-3 h-7 p-0" />
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>


    <div v-else class="flex border-2 rounded-3xl border-[#1D1D2A] mb-2 h-1/3">
        <div class="flex w-1/2 m-2 items-center justify-center">
            <AudioPlayer :url="'/api/posts/' + props.post.post_id + '/content'" />
        </div>
        <div class="flex flex-col w-1/2">
            <div class="flex flex-col h-full">
                <div class="flex h-1/4 mt-6 items-center justify-center">
                    <button class="flex postUserButton w-full h-full mr-4 items-center">
                        <div class="flex w-14 h-14 border-2 border-[#1D1D2A] rounded-full overflow-hidden items-center justify-center">
                            <img class="w-full h-full object-cover" src="../assets/VaclavChalupnicekPP.jpg" alt="Image description">
                        </div>
                        <div class="ml-2">
                            {{ author.name }}
                        </div>
                    </button>
                </div>
                <div class="flex mt-4 h-1/6">
                    <button v-if="isDinged" class="iconButton ml-8 mr-1 w-8 h-8 p-0 text-[#6b21a8]">
                        <BellAlertIcon @click="dinging" class="h-7 ml--4" />
                    </button>
                    <button v-else class="iconButton ml-8 mr-1 w-8 h-8 p-0">
                        <BellIcon @click="dinging" class="h-7 ml--4" />
                    </button>
                    <button class="iconButton mr-1.5 w-8 h-8 p-0">
                        <ChatBubbleBottomCenterTextIcon @click="openComments = !openComments" class="h-7" />
                    </button>
                    <button v-if="isSaved" class="iconButton mr-1 w-8 h-8 p-0 text-[##c70a00]">
                        <InboxArrowDownIcon @click="saving" class="h-7 ml--4" />
                    </button>
                    <button v-else class="iconButton mr-1 w-8 h-8 p-0">
                        <InboxIcon @click="saving" class="h-7 ml--4" />
                    </button>
                </div>
            </div>
        </div>
    </div>
</template>

<style>
.postUserButton {
    background: none;
    border: none;
}

.postUserButton:hover {
    background: none;
    border: none;
}

.iconButton {
    background: none;
    border: none;
    transition-duration: 0.5s;
}

.iconButton:hover {
    background: none;
    border: none;
}
</style>
