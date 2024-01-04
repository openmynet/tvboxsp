<script setup lang="ts">
import { ref, watch } from "vue";
import { useTvBoxStore } from "../../store";
const store = useTvBoxStore();
const input = ref("");
const props = defineProps({
  url: String,
});
const update = () => {
  const src = input.value.trim();
  if (src && /^https?:\/\//.test(src)) {
    store.update_wallpaper(src);
  }
};
watch(
  () => props.url,
  () => {
    input.value = props.url || "";
  }
);
</script>
<template>
  <div class="wallpaper">
    <div class="text-sm bg-slate-100 p-2 rounded">
      <div class="flex flex-row items-center justify-center w-full p-2">
        <img
          :src="url"
          class="cover object-cover max-w-full rounded bg-slate-200"
          :alt="url" />
      </div>
      <div class="text-center">{{ url }}</div>
      <div class="text-center">
        图片按16/9比例方式模拟电视屏幕最终的显示效果
      </div>
    </div>
    <br />
    <div class="flex flex-row">
      <a-input
        size="mini"
        class="flex-1 mr-4"
        v-model="input"
        default-value="url"
        :allow-clear="true"
        placeholder="壁纸" />
      <a-button size="mini" type="outline" @click="update">修改</a-button>
    </div>
  </div>
</template>
<style scoped lang="scss">
.cover {
  height: 18rem;
  width: 32rem;
}
</style>
