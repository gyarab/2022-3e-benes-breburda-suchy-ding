<script setup>
import { PlayIcon, PauseIcon, MicrophoneIcon, ArrowUpTrayIcon } from '@heroicons/vue/20/solid'
import SideBar from './components/SideBar.vue'
import { ref, onMounted } from 'vue'
import Queue from './queue'
import { useRouter } from 'vue-router'
import rest from './rest'
import { notify, capture_err } from './notify'

const router = useRouter()

let mediaRecorder = null
let chunks = []
let audio = ref(null)
let isRecording = ref(false)
let stream = null
let audioCtx = null
let isPlaying = ref(false)
const canvas = ref(null)
const canvasCont = ref(null)
const vizData = ref(new Queue(4000))
const playingSource = ref(null)
const audioInput = ref(null)
const audioFileName = ref("")

async function startRecord() {
  try {
    stream = await navigator.mediaDevices.getUserMedia({ audio: true });
  } catch (e) {
    notify('error', 'Microphone access failed or denied')
    return
  }

  chunks = []

  mediaRecorder = new MediaRecorder(stream)
  mediaRecorder.start()

  mediaRecorder.ondataavailable = e => {
    chunks.push(e.data);
  }

  mediaRecorder.onstop = () => {
    audio.value = new Blob(chunks, { 'type': 'audio/ogg; codecs=opus' })
    console.log(audio.value)
  }
  isRecording.value = true

  if (audioCtx == null) {
    audioCtx = new AudioContext()
  }
  const source = audioCtx.createMediaStreamSource(stream)
  visualize(source)
}

function visualize(source) {
  vizData.value = new Queue(4000)
  const analyser = audioCtx.createAnalyser()
  analyser.fftSize = 32

  const dataArray = new Uint8Array(analyser.frequencyBinCount)

  source.connect(analyser)

  let ctx = canvas.value.getContext('2d')

  function draw() {
    if (isRecording.value || isPlaying.value) requestAnimationFrame(draw)

    analyser.getByteFrequencyData(dataArray)
    let sum = 0;
    for (let i = 0; i < dataArray.length; i++) {
      sum += dataArray[i];
    }
    vizData.value.enqueue(sum / 32 / 255)
    redrawCanvas(ctx)

  }

  draw()
  return analyser
}

async function stopRecord() {
  mediaRecorder.stop();
  stream.getTracks().forEach(t => t.stop());
  isRecording.value = false
}

async function postAudio() {
  if (audio.value == null) return

  capture_err(await rest.post('/api/posts', audio.value))

  notify('info', 'Post created!')

  router.push('/user/me')
}

async function uploadAudio() {
  audioInput.value.click()
}

function onFilePick(ev) {
  console.log(ev.target.files[0])
  audio.value = ev.target.files[0]
  audioFileName.value = ev.target.files[0].name
}

async function playAudio() {
  if (isPlaying.value) return
  if (audio.value == null) return
  if (audioCtx == null) {
    audioCtx = new AudioContext()
  }
  playingSource.value = audioCtx.createBufferSource()
  playingSource.value.buffer = await audioCtx.decodeAudioData(await audio.value.arrayBuffer());

  isPlaying.value = true
  visualize(playingSource.value).connect(audioCtx.destination)
  playingSource.value.loop = true

  playingSource.value.start(0)

}

function pauseAudio() {
  if (playingSource.value) {
    playingSource.value.stop()
    isPlaying.value = false
  }
}

function resizeCanvas() {
  const rect = canvasCont.value.getBoundingClientRect()
  canvas.value.width = rect.width
  canvas.value.height = rect.height
}

function redrawCanvas(ctx) {
  const WIDTH = canvas.value.width
  const HEIGHT = canvas.value.height

  ctx.fillStyle = '#13131b'
  ctx.fillRect(0, 0, WIDTH, HEIGHT)

  ctx.lineWidth = 3;
  ctx.strokeStyle = 'rgb(200, 200, 200)'
  ctx.beginPath()

  let queue_i = Math.max(vizData.value.head, vizData.value.tail - WIDTH / 4)
  ctx.moveTo(0, HEIGHT - vizData.value.elements[queue_i] * HEIGHT * 0.7 - HEIGHT * 0.3)
  for (let i = 4; i < WIDTH; i += 4) {
    ctx.lineTo(i, HEIGHT - vizData.value.elements[queue_i] * HEIGHT * 0.7 - HEIGHT * 0.3)
    queue_i++;
  }
  ctx.stroke()
}
window.addEventListener('resize', resizeCanvas)
onMounted(() => {
  resizeCanvas()
})

</script>

<template>
  <div class="flex h-screen">
    <div class="flex flex-col w-1/4 items-center">
      <SideBar />
    </div>

    <div class="flex flex-col w-1/2 items-center">
      <div class="flex flex-col w-full h-full">
        <div class="flex text-3xl mb-4 font-bold h-16 justify-center">
          <h1 class="mt-6">New Post</h1>
        </div>
        <div ref="canvasCont" class="w-full h-2/5 border-2 rounded-3xl border-[#1D1D2A] overflow-hidden">
          <canvas ref="canvas"></canvas>
        </div>
        <div class="w-full flex h-13 mt-2">
          <div class="flex w-1/2 h-13 items-center justify-start">
            <button :disabled="isRecording" class="flex playPauseButton mr-1 items-center justify-center"
              @click="playAudio">
              <PlayIcon class="h-6 w-6 p-0" />
            </button>
            <button :disabled="isRecording" class="flex playPauseButton mr-1 items-center justify-center"
              @click="pauseAudio">
              <PauseIcon class="h-6 w-6 p-0" />
            </button>
            <input type="file" ref="audioInput" accept="audio/*" hidden @change="onFilePick" />
            <button :disabled="isRecording" class="uploadAudioButton mr-1 flex items-center justify-center"
              @click="uploadAudio">
              <ArrowUpTrayIcon class="h-6 mr-2 p-0" />
              <div>
                Upload
              </div>
            </button>
            <button v-if="!isRecording" class="uploadAudioButton flex items-center justify-center" @click="startRecord()"
              :disabled="isPlaying">
              <MicrophoneIcon class="h-6 mr-2 p-0" />
              <div>
                Record
              </div>
            </button>
            <button v-else class="uploadAudioButton flex items-center justify-center " @click="stopRecord()">
              <MicrophoneIcon class="h-6 mr-2 p-0 text-[#6b21a8]" />
              <div>
                Recording
              </div>
            </button>
            <p class="ml-5">{{ audioFileName }}</p>
          </div>
          <div class="flex w-1/2 h-13 items-center justify-end">
            <button class="newpostButton flex items-center justify-center" @click="pauseAudio(); postAudio()"
              :disabled="audio == null || isRecording">
              <div>
                Post
              </div>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.uploadAudioButton {
  background-color: #1D1D2A;
  border: none;
  transition-duration: 1s;
}

.uploadAudioButton:hover {
  background-color: #aa0000;
  border: none;
}

.playPauseButton {
  background-color: #1D1D2A;
  border: none;
  transition-duration: 1s;
}

.playPauseButton:hover {
  background-color: #6b21a8;
  border: none;
}
</style>
