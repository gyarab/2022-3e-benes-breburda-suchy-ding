<template>

    <div class="userContainer">
        <button class="profilePic" @click="toProfile">
            <UserIcon class="userIcon" />
        </button>
        <p class="username">{{store.user.username}}</p>
    </div>

    <h1>Home Page</h1>


    <!-- UNFINISHED NOTIFICATION... -->

    <!-- <button @click="() => ToggleNotif('buttonTrigger')">Open Popup</button>

    <NotificationsPage v-if="notificationsTriggers.buttonTrigger">
        
    </NotificationsPage> -->
    
</template>

<script setup>
// import { ref } from 'vue';
// import NotificationsPage from './NotificationsPage.vue';
import store from './store';
import { UserIcon } from '@heroicons/vue/20/solid'
import { useRouter } from 'vue-router';
const router = useRouter();

function toProfile() {
  router.push({ path: '/profile' });
}

// const notificationsTriggers = ref({
//     buttonTrigger: false
// });

// const ToggleNotif = (trigger) => {
//     notificationsTriggers.value[trigger] = !notificationsTriggers.value[trigger]
// }

</script>

<style>


.userContainer{
  border-radius: 50px;
  display: inline-flex;
  text-align: center;
  width: 15rem;
  margin: auto;
  padding: 10px;
  background-color: #222230;
  position: relative;
}

.profilePic {
  background-color: #222230;
  color: white;
  text-align: center;
  display: inline-block;

  height: 55px;
  width: 55px;
  border-radius: 50%;
  border: 3px solid #13131b;
}

.username{
  display: inline-flex;
  text-align: center;
  width: 10rem;
  margin: auto;
  position: relative;
}

.userIcon {
  width: 45px;
  margin-left: -21px;
  margin-top: -4px;
}

</style>
