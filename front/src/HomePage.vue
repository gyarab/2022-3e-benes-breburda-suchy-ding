<script setup>
import store from './store';
import { UserIcon } from '@heroicons/vue/20/solid'
</script>

<template>

    <div class="flex">
        <div class="w-1/4">
            <div class="userContainer">
                <button class="profilePic" @click="$router.push({ path: '/profile' })">
                    <UserIcon class="userIcon" />
                </button>
                <p class="username">{{store.user.username}}</p>
            </div>
        </div>
        <div class="w-1/2">feed</div>
        <div class="w-1/4">sidebar</div>
    </div>
    <h1>Home Page</h1>
</template>

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
