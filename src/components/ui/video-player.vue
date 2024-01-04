<script setup lang="ts">
// import video_js from "video.js";
import { VideoPlayer } from "@videojs-player/vue";
import "video.js/dist/video-js.css";
import { computed, ref } from "vue";
// video_js.xhr.XMLHttpRequest = window.XMLHttpRequest;
const url = ref("");
const close = () => {};
const src = computed(() => {
  return {
    src: "",
    type: "application/x-mpegURL",
  };
});
</script>

<template>
  <div
    class="player-masker fixed flex justify-center items-center w-full h-full left-0 top-0 z-10"
    @click.self="close"
    v-if="url">
    <div class="player-container rounded overflow-hidden">
      <video-player
        class="video-player vjs-big-play-centered w-full h-full"
        playsinline
        controls
        ref="playerRefx"
        :options="{ sources: [src] }"
        :data-url="url"
        :playback-rates="[0.7, 1.0, 1.5, 2.0]" />
    </div>
  </div>
</template>
<style lang="scss" scoped>
.player-masker {
  background-color: #000a;
  .player-container {
    width: 60vw;
    height: 60vh;
  }
}
</style>
