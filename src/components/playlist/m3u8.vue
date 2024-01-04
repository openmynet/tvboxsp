<script setup lang="ts">
import { reactive } from "vue";
// TODO playlist.m3u8 playlist.txt 两种格式
const playlist = reactive({
  content: "Coming soon",
});
</script>

<template>
  <div class="playlist p-2 text-sm w-full">
    <h2 v-text="playlist.content"></h2>
  </div>
</template>
