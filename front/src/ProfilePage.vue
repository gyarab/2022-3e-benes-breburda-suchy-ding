<script setup>
import { store } from './store'
</script>

<template>
    <h1 class="text-xl font-bold">Profile Page</h1>
    
    <p>{{ store.user.username }}</p>
    <p>Listener, Listening, PostAmount count</p>
    <p>Audio Bio?</p>
    <p>Listen button</p>
    <p>User posts</p>

</template>
