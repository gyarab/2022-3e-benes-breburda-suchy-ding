<script setup>
import { store } from './store'
import { ArrowLeftCircleIcon } from '@heroicons/vue/24/outline'
import { UserCircleIcon } from '@heroicons/vue/24/outline'
</script>

<template>
    <h1 class="text-xl font-bold">Profile Page</h1>

    <p>{{ store.user.name }}</p>
    <p>Listener, Listening, PostAmount count</p>
    <p>Audio Bio?</p>
    <p>Listen button</p>
    <p>User posts</p>

    <div class="flex flex-col items-center justify-center">
        <div>
            <div class="flex mt-20 items-center text-3xl font-bold">
                <button @click="$router.push('/')" class="iconButton p-0 mr-2">
                    <ArrowLeftCircleIcon class="ml-2 mr-6 h-10" />
                </button>
                <UserCircleIcon class="h-10 mr-2" />
                <h1>
                    {{ store.user.name }}
                </h1>
            </div>
            <div>
            </div>
        </div>
    </div>
</template>
