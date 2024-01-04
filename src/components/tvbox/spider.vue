<script setup lang="ts">
import { computed } from "vue";
import { Message } from "@arco-design/web-vue";
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
const download = () => {
  Message.success("下载完成");
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
