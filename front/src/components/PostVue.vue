<script setup>
import { ref, onBeforeMount } from 'vue'
import { BellAlertIcon, InboxArrowDownIcon, ChevronLeftIcon, ChevronRightIcon, TrashIcon } from '@heroicons/vue/20/solid'
import { ChatBubbleBottomCenterTextIcon, BellIcon, InboxIcon } from '@heroicons/vue/24/outline';
import CommentVue from './CommentVue.vue'
import AudioPlayer from './AudioPlayer.vue'
import ProfilePic from './ProfilePic.vue'
import rest from '../rest'
import { capture_err } from '../notify'
import { store, getUser } from '../store'

const props = defineProps({ post: Object })
const openComments = ref(false)
const addCommentActive = ref(false)
const isDinged = ref(props.post.liked)
const isSaved = ref(props.post.saved)
const author = ref({})
const comments = ref([])
const commentInput = ref("")
const dings = ref(props.post.likes)
const myPost = ref(props.post.author_id == store.user.user_id)
const emit = defineEmits(['delete'])

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
        dings.value--;
    }
    else {
        await rest.post('/api/posts/' + props.post.post_id + '/like')
        dings.value++;
    }
    isDinged.value = !isDinged.value
}

async function postComment() {
    if (commentInput.value.length > 0) {
        const comment = capture_err(await rest.post('/api/posts/' + props.post.post_id + '/comments', { content: commentInput.value }))
        commentInput.value = ""
        addCommentActive.value = false
        comments.value.unshift(comment.body)
    }
}

async function deletePost() {
    capture_err(await rest.del('/api/posts/' + props.post.post_id))
    emit('delete')
}

onBeforeMount(async () => {
    author.value = await getUser(props.post.author_id)
    comments.value = capture_err(await rest.get('/api/posts/' + props.post.post_id + '/comments')).body
})

</script>

<template>
    <div v-if="openComments" class="flex border-2 rounded-3xl border-[#1D1D2A] mb-2 h-1/2">
        <div class="flex w-1/2 m-2 items-center justify-center">
            <AudioPlayer :url="'/api/posts/' + props.post.post_id + '/content'" />
        </div>
        <div class="flex flex-col w-1/2">
            <div class="h-full">
                <div class="flex flex-col h-full m-2">
                    <div class="flex mt-2 items-center">
                        <button class="iconButton w-8 h-8 p-0">
                            <ChevronLeftIcon @click="openComments = !openComments" class="h-7" />
                        </button>
                        <div class="text-xl">
                            Comments
                        </div>
                    </div>
                    <div class="w-full flex-1 overflow-y-scroll mb-4 border-2 rounded-3xl border-[#1D1D2A] hidescrollbar">
                        <CommentVue v-for="i in comments" :comment="i" :key="i.comment_id" />
                    </div>
                    <div class="flex w-full">
                        <div class="flex w-full bg-[#1D1D2A] mb-4 rounded-2xl">
                            <input v-model="commentInput" @input="postChevron" @keyup.enter="postComment"
                                class="flex w-5/6 bg-transparent text-base focus:outline-none placeholder:italic"
                                placeholder="Add your thoughts...">
                            <div v-if="addCommentActive" class="flex items-center">
                                <ChevronRightIcon @click="postComment" class="ml-3 h-7 p-0" />
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
                <div class="flex h-1/4 mt-6 items-center justify-center"
                    @click="$router.push('/user/' + props.post.author_id)">
                    <button class="flex postUserButton w-full h-full mr-4 items-center">
                        <ProfilePic :user="props.post.author_id" class="h-14 w-14" />
                        <div class="ml-2">
                            {{ author.name }}
                        </div>
                    </button>
                </div>
                <div class="flex mt-4 h-1/6 items-center">
                    <button v-if="isDinged" class="iconButton ml-8 mr-1 w-8 h-8 p-0 text-[#6b21a8]">
                        <BellAlertIcon @click="dinging" class="h-7 ml--4" />
                    </button>
                    <button v-else class="iconButton ml-8 mr-1 w-8 h-8 p-0">
                        <BellIcon @click="dinging" class="h-7 ml--4" />
                    </button>
                    <p class="mr-6">{{ dings }}</p>
                    <button class="iconButton mr-1.5 w-8 h-8 p-0">
                        <ChatBubbleBottomCenterTextIcon @click="openComments = !openComments" class="h-7" />
                    </button>
                    <p class="mr-6">{{ comments.length }}</p>
                    <button v-if="isSaved" class="iconButton mr-1 w-8 h-8 p-0 text-[##c70a00]">
                        <InboxArrowDownIcon @click="saving" class="h-7 ml--4" />
                    </button>
                    <button v-else class="iconButton mr-1 w-8 h-8 p-0">
                        <InboxIcon @click="saving" class="h-7 ml--4" />
                    </button>
                </div>
            </div>
        </div>
        <TrashIcon v-if="myPost" class="relative top-0 right-0 m-4 w-8 h-8" @click="deletePost" />
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
