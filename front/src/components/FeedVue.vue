<script setup>
import rest from '../rest';
import { ref, onMounted } from 'vue'
import PostVue from './PostVue.vue'
import { capture_err } from '../notify'

const props = defineProps({ "url": String, "endless": Boolean })
const posts = ref([])
let loading = false

onMounted(async () => {
  const lana = await rest.get(props.url)
  posts.value = lana.body
})

async function onScroll(ev) {
  const cont = ev.target
  if (props.endless && !loading && cont.scrollTop + cont.clientHeight - cont.scrollHeight > -50) {
    loading = true
    const lana = capture_err(await rest.get(props.url))
    lana.body.forEach(p => posts.value.push(p))
    loading = false
  }
}
</script>

<template>
  <div class="my-4 w-full h-full overflow-y-scroll hidescrollbar" @scroll.passive="onScroll">
    <PostVue v-for="i in posts" :post="i" :key="i.post_id"
      @delete="posts.splice(posts.findIndex(p => p.post_id == i.post_id), 1)" />
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