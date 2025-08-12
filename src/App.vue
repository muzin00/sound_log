<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

const isRecording = ref(false);
const isPlaying = ref(false);

async function startRecording() {
  await invoke('start_recording');
  isRecording.value = true;
}

async function stopRecording() {
  await invoke('stop_recording');
  isRecording.value = false;
}

async function playAudio() {
  await invoke('play_audio');
  isPlaying.value = true;
}

async function stopAudio() {
  await invoke('stop_audio');
  isPlaying.value = false;
}
</script>

<template>
  <div v-if="isRecording">
    <button @click="stopRecording">停止</button>
  </div>
  <div v-else-if="!isPlaying">
    <button @click="startRecording">録音</button>
  </div>
  <div v-if="isPlaying">
    <button @click="stopAudio">停止</button>
  </div>
  <div v-else-if="!isRecording">
    <button @click="playAudio">再生</button>
  </div>
</template>
