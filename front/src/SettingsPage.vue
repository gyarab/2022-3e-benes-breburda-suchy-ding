<script setup>
import { ref, reactive } from 'vue'
import { ChevronLeftIcon, CheckIcon, EyeIcon, EyeSlashIcon } from '@heroicons/vue/20/solid'
import { UserCircleIcon, AdjustmentsHorizontalIcon, ArrowLeftCircleIcon, BellAlertIcon, } from '@heroicons/vue/24/outline'
import SetUp from './components/SetUp.vue'
import { store } from './store'
import rest from './rest'
import { notify, capture_err } from './notify'

const userSettings = reactive({
    email: store.user.email,
    oldPassword: '',
    password: '',
    showOld: false,
    show: false,
});
const tab = ref("")

async function saveUserSettings() {
    if (userSettings.email != store.user.email) {
        capture_err(await rest.patch('/api/users/me', {
            email: userSettings.email
        }))

        notify('info', 'Email changed!')
    }
    if (userSettings.password) {
        if (userSettings.password.length < 8) {
            notify('error', 'New password must have at least 8 characters')
            return
        }
        const resp = await rest.put('/api/users/me/password', {
            old_password: userSettings.oldPassword,
            password: userSettings.password
        })
        if (resp.status != 200) {
            if (resp.body.code == 'incorrect_password') {
                notify('error', 'Old password is incorrect')
            } else {
                notify('error', 'Changing password failed')
            }
        } else {
            notify('info', 'Password changed!')
        }
    }
}

</script>

<template>
    <div class="flex flex-col items-center justify-center">
        <div>
            <div class="flex mt-20 items-center text-3xl font-bold">
                <button @click="$router.push('/')" class="iconButton p-0 mr-2">
                    <ArrowLeftCircleIcon class="ml-2 mr-6 h-10" />
                </button>
                <h1>Settings Page</h1>
            </div>
        </div>

        <div class="flex flex-col mt-6 items-center">
            <div v-if="tab == 'user-settings'" class="flex flex-col">
                <input v-model="userSettings.email" type="email" placeholder="Change Email">
                <div class="password-field">
                    <input :type="userSettings.showOld ? 'text' : 'password'" placeholder="Old password"
                        v-model="userSettings.oldPassword" @keyup.enter="login">
                    <button v-if="!userSettings.showOld" @click="userSettings.showOld = !userSettings.showOld"
                        class="border-transparent hover:bg-transparent">
                        <EyeIcon class="eyeIcon" />
                    </button>
                    <button v-else @click="userSettings.showOld = !userSettings.showOld"
                        class="border-transparent hover:bg-transparent">
                        <EyeSlashIcon class="eyeIcon" />
                    </button>
                </div>
                <div class="password-field">
                    <input :type="userSettings.show ? 'text' : 'password'" placeholder="Old password"
                        v-model="userSettings.password" @keyup.enter="login">
                    <button v-if="!userSettings.show" @click="userSettings.show = !userSettings.show"
                        class="border-transparent hover:bg-transparent">
                        <EyeIcon class="eyeIcon" />
                    </button>
                    <button v-else @click="userSettings.show = !userSettings.show"
                        class="border-transparent hover:bg-transparent">
                        <EyeSlashIcon class="eyeIcon" />
                    </button>
                </div>
                <div class="flex w-full mb-4 rounded-2xl">
                    <button @click="saveUserSettings" class="flex rounded-full w-fit items-center m-auto">
                        <CheckIcon class="h-5 m-0" />
                    </button>
                    <button @click="tab = ''" class="flex rounded-full w-fit items-center m-auto">
                        <ChevronLeftIcon class="h-5 m-0" />
                    </button>
                </div>
            </div>
            <div v-else-if="tab == 'profile-settings'" class="flex">
                <SetUp submit-button-text="Save">
                    <button @click="tab = ''" class="flex rounded-full items-center ml-2 mt-2">
                        <ChevronLeftIcon class="h-5 m-0" />
                    </button>
                </SetUp>
            </div>
            <div v-else-if="tab == 'notification-settings'" class="flex flex-col">
                <h1>list of notification settings</h1>
                <div class="flex w-full mb-4 rounded-2xl">
                    <button @click="confirmNotificationSettingsChanges" class="flex rounded-full w-fit items-center m-auto">
                        <CheckIcon class="h-5 m-0" />
                    </button>
                    <button @click="tab = ''" class="flex rounded-full w-fit items-center m-auto">
                        <ChevronLeftIcon class="h-5 m-0" />
                    </button>
                </div>
            </div>
            <div v-else>
                <button @click="tab = 'user-settings'" class="invisButton flex w-64 items-center m-2">
                    <UserCircleIcon class="h-6 mr-2" />
                    <div>
                        User Settings
                    </div>
                </button>
                <button @click="tab = 'profile-settings'" class="invisButton flex w-64 items-center m-2">
                    <AdjustmentsHorizontalIcon class="h-6 mr-2" />
                    <div>
                        Profile Settings
                    </div>
                </button>
                <button @click="tab = 'notification-settings'" class="invisButton flex w-64 items-center m-2">
                    <BellAlertIcon class="h-6 mr-2" />
                    <div>
                        Notification Settings
                    </div>
                </button>
            </div>
        </div>
    </div>
</template>

<style>
.iconButton {
    background: none;
    border: none;
    transition-duration: 0.5s;
}

.iconButton:hover {
    background: none;
    border: none;
}

.invisButton {
    background: none;
    border: none;
    transition-duration: 0.5s;
}

.invisButton:hover {
    background-color: #1D1D2A;
    border: none;
}
</style>