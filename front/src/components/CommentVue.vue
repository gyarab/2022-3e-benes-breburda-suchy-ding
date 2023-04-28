<script setup>
import { UserCircleIcon } from '@heroicons/vue/24/outline'
import { ref, onBeforeMount } from 'vue'
import rest from '../rest'


const props = defineProps({comment: Object})
const author = ref({})

onBeforeMount(async () => {
   author.value = (await rest.get('/api/users/' + props.comment.author_id)).body
})

</script>

<template>
    <div class="flex mt-1 ml-1 h-auto w-full">
        <div>
            <UserCircleIcon class="h-12"/>
        </div>
        <div class="flex items-center mr-12 m-2">
            {{ props.comment.content }}
        </div>
    </div>
</template>