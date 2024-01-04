<script setup lang="ts">
import List from "../ui/list.vue";
import ListItem from "../ui/list-item.vue";
import WidthAuto from "../ui/width-auto.vue";
import HeightAuto from "../ui/height-auto.vue";
import { useTvBoxStore } from "../../store";
import { Modal } from "@arco-design/web-vue";
import { computed, reactive } from "vue";
const state = reactive({
  onlyInvalid: false,
});
const store = useTvBoxStore();
const stats = computed(() => {
  const sites = store.source?.sites || [];
  const count = sites.length;
  const checked = sites.filter((i) => i.status).length;
  const vaild = sites.filter((i) => i.status == 1).length;
  return {
    count,
    checked,
    vaild,
  };
});
const items = computed(() => {
  if (state.onlyInvalid) {
    return (store.source?.sites || []).filter((i) => i.status == -1);
  }
  return store.source?.sites;
});
async function test() {
  await store.check();
  const count = store.source?.sites.filter((i) => i.status == -1).length || 0;
  Modal.success({
    title: "检测完成",
    content: `丢失：${count} 个点播源`,
  });
}
async function removeInvalid() {
  const items = (store.source?.sites || []).filter((i) => i.status == -1);
  if (!items.length) {
    return;
  }
  store.remove_by(items);
}
</script>
<template>
  <div class="vod h-full w-full">
    <HeightAuto class="h-full w-full">
      <template #up>
        <div class="hd flex flex-row justify-between items-center mb-4">
          <span class="text-sm">
            总计：{{ stats.count }}, 已检查: {{ stats.checked }}, 有效:{{
              stats.vaild
            }}
          </span>
          <div class="btns">
            <div class="inline-block mr-4">
              <span class="text-sm mr-2">仅显示失效:</span>
              <a-switch
                checked-color="#08f"
                unchecked-color="#0003"
                size="small"
                v-model="state.onlyInvalid"></a-switch>
            </div>
            <a-button size="mini" type="outline" class="mx-2" @click="test">
              检测
            </a-button>
            <a-button size="mini" type="outline" @click="removeInvalid">
              删除失效
            </a-button>
          </div>
        </div>
      </template>
      <List class="h-full w-full">
        <ListItem
          class="w-full text-sm aria-disabled:text-red-500 aria-required:text-green-600"
          :aria-disabled="item.status == -1"
          :aria-required="item.status == 1"
          v-for="item in items">
          <WidthAuto>
            <div class="name">接口名称：{{ item.name }}</div>
            <div class="type">接口类型：{{ item.type }}</div>
            <div class="api">点播接口：{{ item.api }}</div>
            <div class="feature" v-if="item.features">
              点播功能：{{ item.features }}
            </div>
            <div class="feature" v-if="item.jar">解析引擎：{{ item.jar }}</div>
            <WidthAuto v-if="item.ext">
              <template #head>
                <span style="min-width: 5em">附加参数：</span>
              </template>
              <pre
                class="whitespace-pre-wrap break-all"
                v-text="item.ext"></pre>
            </WidthAuto>
            <template #tail>
              <span
                class="px-2 aria-disabled:text-red-500 aria-required:text-green-500"
                :aria-disabled="item.status == -1"
                :aria-required="item.status == 1"
                @click="store.check_by([item])"
                title="检测">
                <icon-bug size="16" class="" />
              </span>
              <span class="px-2" title="删除" @click="store.remove_by([item])">
                <icon-close-circle-fill size="16" class="text-red-500" />
              </span>
            </template>
          </WidthAuto>
        </ListItem>
      </List>
    </HeightAuto>
  </div>
</template>
