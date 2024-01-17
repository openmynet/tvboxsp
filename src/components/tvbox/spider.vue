<script setup lang="ts">
import { computed } from "vue";
import { Message } from "@arco-design/web-vue";
import { save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
const props = defineProps({
  url: String,
});
const info = computed(() => {
  const items = (props.url || "").split(";");
  return {
    url: items[0],
    hash: items.slice(1).join(":"),
  };
});
const download = async () => {
  if (!info.value.url) {
    return;
  }
  const filePath = await save({
    filters: [
      {
        name: "custom_spider",
        extensions: ["jar"],
      },
    ],
  });
  const ok = await invoke("downlaod", { url: info.value.url, path: filePath });
  if (ok) {
    Message.success("下载成功!");
  } else {
    Message.error("下载失败!");
  }
};
</script>
<template>
  <div class="spider flex flex-row items-center">
    <div class="mr-4 text-sm bg-slate-100 p-2 rounded flex-auto">
      <div class="link" v-text="info.url"></div>
      <span v-text="info.hash" v-show="info.hash"></span>
    </div>
    <a-button size="mini" type="outline" v-show="info.url" @click="download">
      下载
    </a-button>
  </div>
</template>
