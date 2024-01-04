<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import { useTvBoxStore } from "../../store";
import { computed } from "vue";
const store = useTvBoxStore();
const update = () => {
  store.update_warningText(input.value);
  Message.success("保存成功!");
};
const input = computed({
  get: () => {
    return store.source?.warningText || "";
  },
  set: (i: string) => {
    store.update_warningText(i);
  },
});
</script>
<template>
  <div class="spider">
    <div class="flex flex-row items-center">
      <a-input
        size="mini"
        class="flex-1 mr-4"
        v-model="input"
        :default-value="store.source?.warningText"
        :allow-clear="true"
        placeholder="温馨提示" />
      <a-button size="mini" type="outline" @click="update">保存</a-button>
    </div>
  </div>
</template>
