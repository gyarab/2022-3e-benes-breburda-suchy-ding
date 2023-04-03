<script setup>
import { MagnifyingGlassIcon } from '@heroicons/vue/20/solid'
import { QuestionMarkCircleIcon, AdjustmentsHorizontalIcon, ArrowRightCircleIcon } from '@heroicons/vue/24/outline'

</script>

<template>
    <div class="flex flex-col items-center justify-center">
        <div class="flex w-1/2 mt-20 justify-center text-3xl font-bold">
            <h1>Settings Page</h1>
        </div>
        <div class="flex bg-[#1D1D2A] rounded-full h-12 mt-5 items-center">
          <MagnifyingGlassIcon class="h-7 ml-4 mr-4"/>
          <input v-model="search" placeholder="Search Settings" class="h-12 bg-transparent -ml-8 focus:outline-none">
        </div>
        <div class="flex flex-col mt-8 items-center">
            <button class="sidebarButton flex block w-64 items-center m-2">
                <QuestionMarkCircleIcon class="h-6 mr-2"/>
                <div>
                    Nějakej setting
                </div> 
            </button>
            <button class="sidebarButton flex block w-64 items-center m-2">
                <AdjustmentsHorizontalIcon class="h-6 mr-2"/>
                <div>
                    Nějakej jinej setting
                </div> 
            </button>
            <button class="sidebarButton flex block w-64 items-center m-2">
                <ArrowRightCircleIcon class="h-6 mr-2"/>
                <div>
                    Bude jich tu spoustu
                </div> 
            </button>
        </div>
    </div>

</template>

<style>

</style>