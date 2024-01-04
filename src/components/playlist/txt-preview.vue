<script setup lang="ts">
// import { Message } from "@arco-design/web-vue";
import { useTxtPlaylistStore } from "../../store";
import { open } from "@tauri-apps/api/shell";
import { save } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { ref, computed, onMounted } from "vue";
import QRCodeVue3 from "qrcode-vue3";
import { Message } from "@arco-design/web-vue";
import Copy from "../ui/copy-text.vue";
const store = useTxtPlaylistStore();
const ip = ref("127.0.0.1");
const url = computed(() => {
  return `http://${ip.value}:8090/playlist.txt`;
});
const openWith = () => {
  open(url.value);
};
const saveWith = async () => {
  const filePath = await save({
    filters: [
      {
        name: "playlist",
        extensions: ["txt"],
      },
    ],
  });
  const content = store.content_text;
  const ok = await invoke("save", { path: filePath, content });
  if (ok) {
    Message.success("保存成功!");
  } else {
    Message.error("保存失败!");
  }
};
const load = async () => {
  const ips = await invoke<undefined | string[]>("lan_ip");
  if (ips) {
    ip.value = ips[0];
  }
};
onMounted(() => {
  load();
});
</script>
<template>
  <div class="spider">
    <div class="flex flex-col justify-center items-center">
      <div class="qrcode">
        <QRCodeVue3
          :width="200"
          :height="200"
          :value="url"
          :key="url"
          :dotsOptions="{
            type: 'square',
            color: '#222',
          }"
          cornersSquareOptions="{
        type:'square'
      }"></QRCodeVue3>
      </div>
      <Copy class="mt-2" v-text="url"></Copy>
      <div class="buttons">
        <a-button size="mini" class="mr-2" type="outline" @click="saveWith">
          下载
        </a-button>
        <a-button size="mini" type="outline" @click="openWith">打开</a-button>
      </div>
    </div>
  </div>
</template>
