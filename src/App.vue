<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

async function startRecording() {
  await invoke('start_recording');
}

async function stopRecording() {
  await invoke('stop_recording');
}
</script>

<template>
    <button @click="startRecording">録音</button>
    <button @click="stopRecording">停止</button>
</template>
