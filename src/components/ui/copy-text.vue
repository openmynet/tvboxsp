<script setup lang="ts">
import { writeText } from "@tauri-apps/api/clipboard";
import { Message } from "@arco-design/web-vue";

const props = defineProps({
  text: String,
});
async function copy() {
  if (props.text) {
    await writeText(props.text);
    Message.success("复制完成!");
  }
}
</script>

<template>
  <span
    v-text="text"
    class="select-none truncate"
    @dblclick="copy"
    :title="`双击复制: ${text}`"></span>
</template>
