<script setup>
import store from './store';
import { UserIcon, SignalIcon } from '@heroicons/vue/20/solid'
import { BellIcon,Cog8ToothIcon } from '@heroicons/vue/24/outline'


</script>

<template>

    <div class="flex">
        <div class="flex flex-col w-1/4 items-center">
          <div>
            <div class="userContainer">
                <button class="profilePic" @click="$router.push({ path: '/profile' })">
                    <UserIcon class="userIcon" />
                </button>
                <p class="username">{{store.user.username}}</p>
            </div>
              <button class="newpostButton flex block items-center justify-center w-64 m-2">
                <SignalIcon class="h-10 mr-2"/>
                <div>
                  New Post
                </div>
              </button>
              <button class="sidebarButton flex block w-64 items-center m-2">
                <BellIcon class="h-6 mr-2"/>
                <div>
                  Notifications
                </div> 
              </button>
              <button class="sidebarButton flex block w-64 items-center m-2">
                <Cog8ToothIcon class="h-6 mr-2"/>
                <div>
                  Settings
                </div> 
              </button>
          </div>
        </div>
        <div class="w-1/2">feed</div>
        <div class="w-1/4">sidebar</div>
    </div>
</template>

<style>

.newpostButton{
  background-color: #028090;
  border: none;
}

.sidebarButton{
  background: none;
  border: none;
}

.userContainer{
  border-radius: 50px;
  display: flex;
  text-align: center;
  width: 15rem;
  margin: auto;
  padding: 10px;
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
