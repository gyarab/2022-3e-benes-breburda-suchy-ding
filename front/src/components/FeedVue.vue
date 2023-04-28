<script setup>
import rest from '../rest';
import { ref, onMounted } from 'vue'
import PostVue from './PostVue.vue'

const props = defineProps({"url": String})
const posts = ref([])

onMounted(async () => {
   const lana = await rest.get(props.url)
   posts.value = lana.body
})

</script>

<template>
    <div class="my-4 w-full h-full overflow-y-scroll hidescrollbar">
        <PostVue v-for="i in posts" :post="i" :key="i.post_id" />
      </div>
</template>

<style>
.hidescrollbar {
  -ms-overflow-style: none;
  /* for Internet Explorer, Edge */
  scrollbar-width: none;
  /* for Firefox */
  overflow-y: scroll;
}
.hidescrollbar::-webkit-scrollbar {
  display: none;
  /* for Chrome, Safari, and Opera */
}
</style>