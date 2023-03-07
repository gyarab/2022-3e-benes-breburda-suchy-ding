<template>
    <h1>Home Page</h1>
    <a href="#/profile">Profile</a>
    <a href="#/post">Post</a>

    <button @click="() => ToggleNotif('buttonTrigger')">Open Popup</button>

    <NotificationsPage v-if="notificationsTriggers.buttonTrigger">
        <h2>Notif</h2>
    </NotificationsPage>
</template>

<script setup>
import { ref } from 'vue';

const notificationsTriggers = ref({
    buttonTrigger: false
});

const ToggleNotif = (trigger) => {
    notificationsTriggers.value[trigger] = !notificationsTriggers.value[trigger]
}

</script>
