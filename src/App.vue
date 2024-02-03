<script setup lang="ts">
import { reactive } from "vue";
import playlist from "./components/playlist.vue";
import tvbox from "./components/tvbox.vue";
const menu = reactive([
  { name: "直播源", checked: true },
  { name: "TvBox", checked: false },
]);
const onMenuClick = (item: any) => {
  menu.forEach((i) => (i.checked = false));
  item.checked = true;
};
</script>

<template>
  <div class="flex flex-row bg-white">
    <nav class="basis-36 text-sm drop-shadow-lg min-w-36">
      <ul class="h-screen pt-3 cursor-pointer bg-slate-50">
        <li
          class="pl-3 py-2 transition-colors duration-200 hover:bg-slate-200 aria-checked:bg-slate-300"
          v-for="item in menu"
          v-text="item.name"
          :aria-checked="item.checked"
          @click="onMenuClick(item)"></li>
      </ul>
    </nav>
    <div class="main flex-1 p-2">
      <div class="h-full rounded p-2 border border-slate-400">
        <playlist v-show="menu[0].checked" />
        <tvbox v-show="menu[1].checked" />
      </div>
    </div>
  </div>
</template>

<style scoped></style>
