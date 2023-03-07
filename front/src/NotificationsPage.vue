<script setup>

</script>

<template>
    <div class="fixed top-0 left-0 right-0 bottom-0 z-50 bg-gray-800 flex items-center justify-center"> <!-- WHY NO WORK CSS (ALLAH PLS HELP ME!) -->
        <div class="bg-white p-32">
            <slot />
            <button class="Notifications-close"> Close </button>
        </div>
    </div>

</template>

<style>
/* .Notifications {
	position: fixed;
	top: 0;
	left: 0;
	right: 0;
	bottom: 0;
	z-index: 99;
	background-color: rgb(80, 80, 80);
	
	display: flex;
	align-items: center;
	justify-content: center;
    
	.Notifications-inner {
		background: #FFF;
		padding: 32px;
	}
} */
</style>
