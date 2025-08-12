<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';

interface Record {
  channels: number;
  samples: number[];
  sample_rate: number;
}

const isRecording = ref(false);

async function startRecording() {
  await invoke('start_recording');
  isRecording.value = true;
}

async function stopRecording() {
  await invoke('stop_recording');
  isRecording.value = false;
}

async function playAudio() {
  const record = await invoke<Record>('play_audio');
  if (!record) return;
  const audioContext = new AudioContext();
  const audioBuffer = audioContext.createBuffer(
    record.channels,
    record.samples.length,
    record.sample_rate
  );
  audioBuffer.copyToChannel(new Float32Array(record.samples), 0);
  const source = audioContext.createBufferSource();
  source.buffer = audioBuffer;
  source.connect(audioContext.destination);
  source.start();
}
</script>

<template>
  <div v-if="isRecording">
    <button @click="stopRecording">停止</button>
  </div>
  <div v-else>
    <button @click="startRecording">録音</button>
    <button @click="playAudio">再生</button>
  </div>
</template>
