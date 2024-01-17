<script setup lang="ts">
import { ref, computed } from "vue";
import { useTvBoxStore } from "../../store";
const store = useTvBoxStore();
const defaultUri = ref("https://jihulab.com/z-blog/xh2/-/raw/main/t.json");
const source = computed({
  get: () => {
    return store.merginSource || defaultUri.value;
  },
  set: (value: string) => {
    store.update_merginSource(value);
  },
});
</script>
<template>
  <div class="mergin min-w-96">
    <div class="flex flex-col">
      <a-textarea
        size="mini"
        v-model="source"
        :allow-clear="true"
        :auto-size="{ maxRows: 12, minRows: 7 }"
        placeholder="点播源URL地址, 每个地址一行，或者使用,;将url隔开" />
      <small class="mt-1">
        每个URL点播源地址一行，或者使用
        <b class="text-red-500 px-1 bg-slate-100 rounded">，;</b>
        将URL点播源地址
      </small>
    </div>
    <hr class="mt-4" />
    <div class="mt-4 text-sm bg-slate-100 py-3 px-3 rounded flex-auto">
      <div class="mb-2">
        合并项：点播源，直播源，解析器，vip标识，广告过滤，规则
        <hr class="my-2" />
        请注意：检测仅保证连接可访问，无法保证其内容有效
        在合并点播源时需要注意各个tvbox之间的版本差异，部分版本可能不支持python脚本
      </div>
    </div>
  </div>
</template>
