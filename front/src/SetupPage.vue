<script setup>
</script>

<template>
    <div class="flex h-screen w-screen">
        <div class="flex flex-col h-4/5 w-full justify-center items-center">
            <h1 class="text-3xl font-bold">Hello, welcome to Ding!</h1>
            <h2 class="text-xl font-bold mt-2 mb-10 text-[#828282]">Let's get you sorted...</h2>
            <button class="continueButton flex w-64 justify-center items-center">
                Continue
            </button>
        </div>
    </div>
</template>

<style>
.continueButton {
    background-color: #1D1D2A;
    border: none;
    transition-duration: 1s;
}

.continueButton:hover {
    background-color: #6b21a8;
    border: none;
}
</style>