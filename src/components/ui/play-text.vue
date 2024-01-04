<script setup lang="ts">
import CopyText from "./copy-text.vue";
import { invoke } from "@tauri-apps/api/tauri";
const props = defineProps({
  text: String,
});
async function try_play() {
  if (props.text && /^https?:\/\//i.test(props.text)) {
    //   const url = "http://hw-m-l.cztv.com/channels/lantian/channel008/1080p.m3u8";
    const url = props.text;
    await invoke("exec", { args: `start mpv ${url}` });
  }
}
</script>
<template>
  <div class="try-play inline-flex flex-row max-w-full">
    <CopyText :text="text" class="flex-1"></CopyText>
    <span
      class="select-none inline-block ml-2 hover:text-blue-500 min-w-8"
      @click="try_play">
      ▶️
    </span>
  </div>
</template>
