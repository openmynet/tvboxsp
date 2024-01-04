<script setup lang="ts">
import { PropType, reactive, computed } from "vue";
import CopyText from "../ui/copy-text.vue";
import List from "../ui/list.vue";
import ListItem from "../ui/list-item.vue";
import HeightAuto from "../ui/height-auto.vue";
import WidthAuto from "../ui/width-auto.vue";
import { useTvBoxStore } from "../../store";
import { Modal, Message } from "@arco-design/web-vue";
defineProps({
  data: Array as PropType<TvBoxLive[]>,
});
const state = reactive({
  name: "",
  url: "",
  show: false,
});
const stats = computed(() => {
  const items = store.source?.lives || [];
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
  await store.check_live();
  const count = store.source?.lives.filter((i) => i.status == -1).length || 0;
  if (!count) {
    Message.success("检测完成！");
    return;
  }
  Modal.success({
    title: "检测完成",
    content: `丢失：${count} 个直播源`,
  });
}
async function add() {
  const name = state.name.trim();
  const url = state.url.trim();
  if (!name) {
    Message.error("请输入直播源名称");
    return;
  }
  if (!url) {
    Message.error("请输入直播源地址");
    return;
  }
  if (!/^https?:\/\//.test(url)) {
    Message.error("请输入有效的直播源地址");
    return;
  }
  store.add_live({
    name,
    url,
  });
}
</script>
<template>
  <div class="live h-full">
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
            type="primary"
            @click="state.show = true"
            class="mx-2">
            添加
          </a-button>
          <a-button size="mini" type="outline" @click="test">检测</a-button>
        </div>
      </template>
      <list class="h-full text-sm">
        <ListItem
          v-for="(item, key) in data"
          :aria-disabled="item.status == -1"
          :key="key">
          <WidthAuto>
            <div class="name" v-if="item.name">名称：{{ item.name }}</div>
            <div class="group" v-if="item.group">分组：{{ item.group }}</div>
            <div class="hosts" v-if="item.type != null">
              类型：{{ item.type }}
            </div>
            <div class="hosts" v-if="item.url">
              地址：
              <CopyText :text="item.url"></CopyText>
            </div>
            <div class="hosts" v-if="item.epg">EPG：{{ item.epg }}</div>
            <WidthAuto class="regex" v-if="item.channels">
              <template #head>
                <span class="min-w-7">频道</span>
              </template>
              <div class="chns">
                <div class="chn" v-for="chn in item.channels">
                  <div class="chn-name">
                    <span class="min-w-11">名称：</span>
                    {{ chn.name }}
                  </div>
                  <div class="chn-url flex flex-row">
                    <span class="min-w-11">地址：</span>
                    <div class="urls">
                      <div
                        class="whitespace-pre-wrap break-all"
                        v-for="url in chn.urls">
                        {{ url }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </WidthAuto>
            <template #tail>
              <span
                class="px-2 aria-disabled:text-red-500 aria-required:text-green-500"
                :aria-disabled="item.status == -1"
                :aria-required="item.status == 1"
                @click="store.check_live([item])"
                title="检测">
                <icon-bug size="16" class="" />
              </span>
              <span class="px-2" title="删除" @click="store.remove_live(key)">
                <icon-close-circle-fill size="16" class="text-red-500" />
              </span>
            </template>
          </WidthAuto>
        </ListItem>
      </list>
    </HeightAuto>

    <a-modal
      v-model:visible="state.show"
      title="添加直播源"
      @cancel="state.show = false"
      @ok="add">
      <a-form :model="state" size="small" auto-label-width>
        <a-form-item field="addSource" label="直播名称: ">
          <a-input v-model="state.name" placeholder="输入直播源名称"></a-input>
        </a-form-item>
        <a-form-item field="addSource" label="直播地址: ">
          <a-input v-model="state.url" placeholder="输入直播源地址"></a-input>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>
