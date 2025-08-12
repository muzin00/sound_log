<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

interface Record {
  channels: number;
  samples: number[];
  sample_rate: number;
}

async function startRecording() {
  await invoke('start_recording');
}

async function stopRecording() {
  await invoke('stop_recording');
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
    <button @click="startRecording">録音</button>
    <button @click="stopRecording">停止</button>
    <button @click="playAudio">再生</button>
</template>
