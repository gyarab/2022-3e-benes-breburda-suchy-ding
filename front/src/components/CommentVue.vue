<script setup>
import { ref, onBeforeMount } from 'vue'
import rest from '../rest'
import ProfilePic from './ProfilePic.vue'

const props = defineProps({ comment: Object })
const author = ref({})

onBeforeMount(async () => {
    author.value = (await rest.get('/api/users/' + props.comment.author_id)).body
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