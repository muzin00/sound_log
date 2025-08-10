<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onBeforeMount, ref } from 'vue';

interface AudioData {
    channels: number;
    sample_rate: number;
    samples: number[];
}

const inputDevices = ref<string[]>([]);
const audioData = ref<AudioData | null>(null);

onBeforeMount(async () => {
  inputDevices.value = await invoke('get_input_devices');
});

async function startRecording() {
  const data = await invoke<AudioData>('start_recording');
  console.log(data);
  audioData.value = data;
}

async function playAudio() {
  if (!audioData.value) return;
  const audioContext = new AudioContext();
  const audioBuffer = audioContext.createBuffer(
    audioData.value.channels,
    audioData.value.samples.length,
    audioData.value.sample_rate
  );
  audioBuffer.copyToChannel(new Float32Array(audioData.value.samples), 0);
  const source = audioContext.createBufferSource();
  source.buffer = audioBuffer;
  source.connect(audioContext.destination);
  source.start();
}
</script>

<template>
    <select>
        <option v-for="device in inputDevices" :value="device" :key="device">{{ device }}</option>
    </select>
    <button @click="startRecording">録音</button>
    <button @click="playAudio">再生</button>
</template>
