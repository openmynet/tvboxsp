<script setup lang="ts">
import { PropType, computed } from "vue";
import List from "../ui/list.vue";
import ListItem from "../ui/list-item.vue";
import HeightAuto from "../ui/height-auto.vue";
import WidthAuto from "../ui/width-auto.vue";
import { useTvBoxStore } from "../../store";
import { Modal } from "@arco-design/web-vue";
const stats = computed(() => {
  const items = store.source?.parses || [];
  const count = items.length;
  const checked = items.filter((i) => i.status).length;
  const vaild = items.filter((i) => i.status == 1).length;
  return {
    count,
    checked,
    vaild,
  };
});
const store = useTvBoxStore();
async function test() {
  await store.check_parses();
  const count = store.source?.parses?.filter((i) => i.status == -1).length || 0;
  Modal.success({
    title: "检测完成",
    content: `丢失：${count} 个解析器`,
  });
}
async function removeInvalid() {
  const items = (store.source?.parses || []).filter((i) => i.status == -1);
  if (!items.length) {
    return;
  }
  store.remove_parses(items);
}
defineProps({
  data: Array as PropType<TvBoxParse[]>,
});
</script>
<template>
  <div class="parses h-full">
    <HeightAuto class="h-full w-full">
      <template #up>
        <div class="hd mb-4 flex flex-row">
          <span class="text-sm flex-auto">
            总计：{{ stats.count }}, 已检查: {{ stats.checked }}, 有效:{{
              stats.vaild
            }}
          </span>
          <a-button
            size="mini"
            type="outline"
            class="mx-2"
            @click="removeInvalid">
            删除失效
          </a-button>
          <a-button size="mini" type="outline" @click="test">检测</a-button>
        </div>
      </template>
      <List class="h-full text-sm">
        <ListItem
          v-for="(item, key) in data"
          :aria-disabled="item.status == -1"
          :key="key">
          <WidthAuto>
            <div class="name">
              解析器名称：{{ item.name }}, 类型：{{ item.type }}
            </div>
            <WidthAuto class="url">解析器地址：{{ item.url }}</WidthAuto>
            <WidthAuto class="ext">
              <template #head>
                <span>附加参数：</span>
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
                @click="store.check_parses([item])"
                title="检测">
                <icon-bug size="16" class="" />
              </span>
              <span class="px-2" title="删除" @click="store.remove_parses(key)">
                <icon-close-circle-fill size="16" class="text-red-500" />
              </span>
            </template>
          </WidthAuto>
        </ListItem>
      </List>
    </HeightAuto>
  </div>
</template>
