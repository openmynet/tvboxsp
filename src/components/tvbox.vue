<script setup lang="ts">
import { reactive, onMounted } from "vue";
import { Message } from "@arco-design/web-vue";
import WidthAuto from "./ui/width-auto.vue";
import Rules from "./tvbox/rules.vue";
import Ijk from "./tvbox/ijk.vue";
import Ads from "./tvbox/ads.vue";
import Flags from "./tvbox/flags.vue";
import Parses from "./tvbox/parses.vue";
import Live from "./tvbox/live.vue";
import Vod from "./tvbox/vod.vue";
import Spider from "./tvbox/spider.vue";
import Wallpaper from "./tvbox/wallpaper.vue";
import Warning from "./tvbox/warning.vue";
import Mergin from "./tvbox/mergin.vue";
import Preview from "./tvbox/preview.vue";
import { useTvBoxStore } from "../store";
import { confirm } from "../utils";
import { listen } from "@tauri-apps/api/event";
const tvbox = reactive({
  // uri: "https://jihulab.com/z-blog/xh2/-/raw/main/t.json",
  uri: "https://jihulab.com/z-blog/vip/-/raw/main/dd/t.json",
  show: false,
  mergining: false,
  tips: "",
  percent: 0,
});
listen("check_connections://progress", async (e) => {
  const { payload } = e;
  const { progress, total } = payload as { progress: number; total: number };
  tvbox.tips = `${progress} / ${total}`;
  tvbox.percent = parseFloat((progress / total).toFixed(4)) || 0.0;
  if (progress == total) {
    // 需要更好的处理方法
    setTimeout(() => {
      tvbox.tips = "";
      tvbox.percent = 0;
    }, 1000);
  }
});

const store = useTvBoxStore();
async function load() {
  await store.load(tvbox.uri);
  Message.success("加载完成!");
}
const previewHandle = async () => {
  await store.cache();
  tvbox.show = true;
};
const merginHandle = () => {
  tvbox.mergining = true;
};
const merginConfirm = async () => {
  const code = await confirm("确定要合并这些tbvox源吗").catch((_) => 503);
  if (code == 503) {
    tvbox.mergining = false;
    Message.success("合并已取消");
    return;
  }
  await store.mergin();
  tvbox.mergining = false;
  Message.success("合并完成");
};
onMounted(() => {
  store.init();
});
</script>

<template>
  <a-spin
    :size="48"
    :loading="store.loading"
    :tip="tvbox.tips"
    class="p-2 w-full h-full max-w-full">
    <template #icon>
      <div class="p-4 rounded bg-white shadow">
        <a-progress
          type="circle"
          track-color="#06f3"
          :percent="tvbox.percent"
          v-if="tvbox.tips" />
        <icon-loading v-else />
      </div>
    </template>
    <template #tip>
      <small class="rounded bg-blue-100 px-2 inline-block" v-if="tvbox.tips">
        当前进度: {{ tvbox.tips }}
      </small>
    </template>

    <div class="tvbox w-full h-full flex flex-col">
      <div class="head-search flex flex-row w-full">
        <a-input
          size="mini"
          class="flex-1"
          v-model="tvbox.uri"
          :allow-clear="true"
          placeholder="点播源URL地址" />
        <a-button size="mini" class="ml-2" type="outline" @click="load">
          加载
        </a-button>
      </div>
      <div class="flex-y-auto mt-2 flex flex-col w-full max-w-full relative">
        <a-tabs default-active-key="1" size="mini" class="flex h-full w-full">
          <template #extra>
            <a-button
              class="mr-2"
              type="primary"
              size="mini"
              @click="previewHandle()">
              <small>预览</small>
            </a-button>
            <a-button type="outline" size="mini" @click="merginHandle()">
              <small>合并</small>
            </a-button>
          </template>
          <a-tab-pane key="1" title="点播源">
            <Vod></Vod>
          </a-tab-pane>
          <a-tab-pane key="2" title="直播源">
            <Live :data="store.source?.lives"></Live>
          </a-tab-pane>
          <a-tab-pane key="3" title="解析器">
            <Parses :data="store.source?.parses"></Parses>
          </a-tab-pane>
          <a-tab-pane key="4" title="VIP标识">
            <WidthAuto class="h-full">
              <Flags :data="store.source?.flags"></Flags>
            </WidthAuto>
          </a-tab-pane>
          <a-tab-pane key="5" title="广告">
            <Ads :data="store.source?.ads"></Ads>
          </a-tab-pane>
          <a-tab-pane key="6" title="IJK参数">
            <WidthAuto class="h-full">
              <Ijk :data="store.source?.ijk"></Ijk>
            </WidthAuto>
          </a-tab-pane>
          <a-tab-pane key="7" title="规则">
            <WidthAuto class="h-full">
              <Rules :data="store.source?.rules"></Rules>
            </WidthAuto>
          </a-tab-pane>
          <a-tab-pane key="9" title="爬虫引擎">
            <WidthAuto>
              <Spider :url="store.source?.spider"></Spider>
            </WidthAuto>
          </a-tab-pane>
          <a-tab-pane key="8" title="壁纸">
            <WidthAuto>
              <Wallpaper :url="store.source?.wallpaper"></Wallpaper>
            </WidthAuto>
          </a-tab-pane>
          <a-tab-pane key="0" title="提示">
            <WidthAuto>
              <Warning :text="store.source?.warningText"></Warning>
            </WidthAuto>
          </a-tab-pane>
        </a-tabs>
      </div>
      <a-modal
        v-model:visible="tvbox.show"
        title="预览"
        @cancel="tvbox.show = false"
        @ok="tvbox.show = false">
        <Preview v-if="tvbox.show"></Preview>
      </a-modal>
      <a-modal
        v-model:visible="tvbox.mergining"
        title="合并"
        width="72vw"
        @cancel="tvbox.mergining = false"
        @ok="merginConfirm()">
        <Mergin></Mergin>
      </a-modal>
    </div>
  </a-spin>
</template>
<style lang="scss" scoped>
.tvbox {
  ::v-deep() {
    .arco-tabs-nav-type-line .arco-tabs-tab {
      margin: 0 8px;
    }
  }
}
</style>
