<script setup lang="ts">
import { reactive } from "vue";
import List from "../ui/list.vue";
import ListItem from "../ui/list-item.vue";
import WidthAuto from "../ui/width-auto.vue";
import { Message, Modal } from "@arco-design/web-vue";
import { useTvBoxStore } from "../../store";
const store = useTvBoxStore();
const input = reactive({
  uri: "https://jihulab.com/z-blog/xh2/-/raw/main/t.json",
  source: [] as string[],
});
const push = async () => {
  const uri = input.uri.trim();
  if (uri && /https?:\/\//.test(uri) && !input.source.includes(uri)) {
    input.source.push(uri);
  }
};
const remove = (i: number) => {
  input.source.splice(i, 1);
};
async function mergin() {
  if (!input.source.length) {
    return;
  }
  Modal.confirm({
    content: "确定要合并这些配置源",
    onOk(_e) {
      inlineMergin();
    },
  });
}
async function inlineMergin() {
  if (!input.source.length) {
    return;
  }

  for (let i = 0; i < input.source.length; i++) {
    const uri = input.source[i];
    await store.push(uri).catch((_) => {});
  }
  Message.success("合并完成");
}
</script>
<template>
  <div class="mergin">
    <div class="flex flex-row items-center">
      <a-input
        size="mini"
        class="flex-1"
        v-model="input.uri"
        :allow-clear="true"
        placeholder="点播源URL地址" />
      <a-button size="mini" class="ml-2" type="outline" @click="push">
        添加
      </a-button>
    </div>
    <hr class="mt-4" />
    <List>
      <ListItem v-for="(src, key) in input.source" :key="key">
        <WidthAuto>
          <small v-text="src"></small>
          <template #tail>
            <a-button
              status="danger"
              type="outline"
              size="mini"
              @click="remove(key)">
              删除
            </a-button>
          </template>
        </WidthAuto>
      </ListItem>
    </List>
    <div class="mt-4 text-sm bg-slate-100 py-3 px-3 rounded flex-auto">
      <div class="mb-2">
        合并项：点播源，直播源，解析器，vip标识，广告过滤，规则
      </div>

      <a-button type="primary" size="mini" @click="mergin">
        合并以上列表到当前配置
      </a-button>
    </div>
    <div class="mt-4 text-sm bg-slate-100 py-3 px-3 rounded">
      请注意，检测仅保证连接可访问，无法保证其内容有效
      在合并点播源时需要注意各个tvbox之间的版本差异，部分版本可能不支持python脚本
    </div>
  </div>
</template>
