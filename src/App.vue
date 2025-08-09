<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { onBeforeMount, ref } from 'vue';

const inputDevices = ref<string[]>([]);

onBeforeMount(async () => {
  inputDevices.value = await invoke('get_input_devices');
});
</script>

<template>
    <select>
        <option v-for="device in inputDevices" :value="device" :key="device">{{ device }}</option>
    </select>
</template>
