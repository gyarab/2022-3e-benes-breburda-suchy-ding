<script setup>
import { PlayIcon, PauseIcon } from '@heroicons/vue/24/outline'
import { ref, onBeforeUnmount } from 'vue'

const props = defineProps({ url: String })
const isPlaying = ref(false)
const audioEl = ref(null)
const iconEl = ref(null)


let analyser = null
let animationFrame = null

function setIconColor(v) {
    iconEl.value.style.color = `rgba(107, 33, 168, ${v})`
}

function visualize() {
    const dataArray = new Uint8Array(analyser.frequencyBinCount)

    function draw() {
        if (isPlaying.value) {
            animationFrame = requestAnimationFrame(draw)
        }

        analyser.getByteFrequencyData(dataArray)
        let sum = 0;
        for (let i = 0; i < dataArray.length; i++) {
            sum += dataArray[i];
        }
        setIconColor(sum / 32 / 255 * 0.80 + 0.2)
    }

    draw()
}

function togglePlay() {
    isPlaying.value = !isPlaying.value
    if (isPlaying.value) {
        audioEl.value.play()
    } else {
        audioEl.value.pause()
        cancelAnimationFrame(animationFrame)
        iconEl.value.style.color = ''
    }
}

function buildCtx() {
    if (!analyser) {
        const audioCtx = new AudioContext()
        analyser = audioCtx.createAnalyser()
        analyser.fftSize = 32
        const source = audioCtx.createMediaElementSource(audioEl.value)
        source.connect(analyser).connect(audioCtx.destination)
    }
    visualize()
}

onBeforeUnmount(() => {
    cancelAnimationFrame(animationFrame)
    isPlaying.value = false
})
</script>

<template>
    <div class="flex items-center justify-center">
        <audio :src="'https://ding.ecko.ga' + props.url" crossorigin="anonymous" style="display: none" ref="audioEl"
            @play="buildCtx" loop></audio>
        <PlayIcon v-if="!isPlaying" ref="iconEl" class="w-1/2 h-1/2 text-[#1D1D2A]" @click="togglePlay" />
        <PauseIcon v-else ref="iconEl" class="w-1/2 h-1/2 text-[#1D1D2A]" @click="togglePlay" />
    </div>
</template>

<style></style>