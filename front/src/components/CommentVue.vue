<script setup>
import { ref, onBeforeMount } from 'vue'
import ProfilePic from './ProfilePic.vue'
import { getUser } from '../store'

const props = defineProps({ comment: Object })
const author = ref({})

onBeforeMount(async () => {
    author.value = getUser(props.comment.author_id)
})

</script>

<template>
    <div class="mt-1 ml-1 h-auto w-full">
        <div class="flex items-start">
            <div class="flex items-center" @click="$router.push(`/user/${props.comment.author_id}`)">
                <ProfilePic :user="props.comment.author_id" class="h-10 w-10 flex-none" />
                <p class="ml-2 mr-4 font-bold">{{ author.name }}</p>
            </div>
            <p class="mt-2">{{ props.comment.content }}</p>
        </div>
    </div>
</template>
