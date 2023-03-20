<template>
    <h1>Home Page</h1>
    <a href="#/profile">Profile</a>
    <a href="#/post">Post</a>

    <button @click="() => ToggleNotif('buttonTrigger')">Open Popup</button>

    <NotificationsPage v-if="notificationsTriggers.buttonTrigger">
        
    </NotificationsPage>

    <p>{{store.user.username}}</p>
    
</template>

<script setup>
import { ref } from 'vue';
import NotificationsPage from './NotificationsPage.vue';
import store from './store';

const notificationsTriggers = ref({
    buttonTrigger: false
});

const ToggleNotif = (trigger) => {
    notificationsTriggers.value[trigger] = !notificationsTriggers.value[trigger]
}

</script>
