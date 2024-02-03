<script setup lang="ts">
import { reactive, computed, h } from "vue";
import { open } from "@tauri-apps/api/dialog";
import CopyText from "../ui/copy-text.vue";
import { Message } from "@arco-design/web-vue";
import { useTxtPlaylistStore } from "../../store";
import { confirm } from "../../utils";
import { IconSearch } from "@arco-design/web-vue/es/icon";
import TxtPreview from "./txt-preview.vue";
import TxtGroup from "./txt-group.vue";
import { listen } from "@tauri-apps/api/event";
listen("urls_accessibility://progress", async (e) => {
  const { payload } = e;
  const { progress, total } = payload as { progress: number; total: number };
  playlist.tips = `${progress} / ${total}`;
  playlist.percent = parseFloat((progress / total).toFixed(4)) || 0.0;
  if (progress == total) {
    // 需要更好的处理方法
    setTimeout(() => {
      playlist.tips = "0/0";
      playlist.percent = 0;
    }, 1000);
  }
});
// TODO playlist.m3u8 playlist.txt 两种格式
const playlist = reactive({
  file: "https://agit.ai/Yoursmile7/TVBox/raw/branch/master/live.txt",
  skip_ipv6: true,
  loading: false,
  tips: "",
  percent: 0,
  preAdd: false,
  addSource: "",
  selectedKeys: [] as string[],
  preview: false,
});
const store = useTxtPlaylistStore();
const items = computed(() => {
  return store.content;
});
const stats = computed(() => {
  const online = store.content.filter((item) => item.online == 1).length;
  const count = store.content.filter((item) => item.http).length;
  return {
    online,
    count,
  };
});
async function check() {
  if (!playlist.file.trim()) {
    return;
  }
  playlist.loading = true;
  await store.check();
  playlist.loading = false;
}
async function recheck() {
  const items = store.content.filter((item) => item.online == -1);
  if (!items.length) {
    return;
  }
  playlist.loading = true;
  await store.check_by(items);
  playlist.loading = false;
}
async function load() {
  if (!playlist.file) {
    return;
  }
  playlist.loading = true;
  await store.load(playlist.file);
  playlist.loading = false;
}
async function onChange(push?: boolean) {
  // Open a selection dialog for image files
  const selected = await open({
    multiple: true,
    filters: [
      {
        name: "playlist.txt",
        extensions: ["txt"],
      },
    ],
  });
  let file;
  if (Array.isArray(selected)) {
    file = selected[0];
  } else if (typeof selected == "string") {
    file = selected;
  }
  if (!file) {
    return;
  }
  if (push) {
    playlist.addSource = file;
  } else {
    playlist.file = file;
  }
}
const text = computed({
  get: () => {
    return store.content_text;
  },
  set: (text: string) => {
    store.update(text);
  },
});
const add = async () => {
  const source = playlist.addSource.trim();
  if (!source) {
    return;
  }
  playlist.loading = true;
  await store.push(source);
  playlist.loading = false;
};
const removeInvalid = async () => {
  await confirm("确定要删除这些直播源吗");
  const items = store.content.filter((item) => item.online == -1);
  if (!items.length) {
    return;
  }
  store.remove_by(items);
  Message.success("删除完成");
};
const removeSelected = async () => {
  await confirm("确定要删除选中项吗");
  if (!playlist.selectedKeys.length) {
    return;
  }
  const items = store.content.filter((item) =>
    playlist.selectedKeys.includes(item.url)
  );
  if (!items.length) {
    return;
  }
  store.remove_by(items);
  Message.success("删除完成");
};
const removeIpv6 = async () => {
  await confirm("确定要删除所有IPv6直播源吗");
  const items = store.content.filter((item) =>
    /^https?:\/\/\[/i.test(item.url)
  );
  if (!items.length) {
    return;
  }
  store.remove_by(items);
  Message.success("删除完成");
};

const filterable = {
  filters: [
    {
      text: "无效",
      value: "-1",
    },
    {
      text: "有效",
      value: "1",
    },
    {
      text: "全部",
      value: "0",
    },
  ],
  defaultFilteredValue: ["0"],
  filter: (value: string[], row: any) => {
    if (!value || !row) {
      return true;
    }
    let i = parseInt(value[0]);
    if (!i) {
      return true;
    }
    return row.online == i;
  },
};
const preview = async () => {
  await store.cache();
  playlist.preview = true;
};
</script>

<template>
  <a-spin
    :size="48"
    :loading="playlist.loading"
    :tip="playlist.tips"
    class="h-full w-full">
    <template #icon>
      <div class="p-4 rounded bg-white shadow">
        <a-progress
          type="circle"
          track-color="#06f3"
          :percent="playlist.percent"
          v-if="playlist.tips" />
        <icon-loading v-else />
      </div>
    </template>
    <template #tip>
      <small class="rounded bg-blue-100 px-2 inline-block" v-if="playlist.tips">
        当前进度: {{ playlist.tips }}
      </small>
    </template>

    <div class="playlist text-sm w-full h-full flex flex-col">
      <form class="group peer w-full">
        <div class="item flex flex-row w-full">
          <a-input-search
            v-model="playlist.file"
            size="mini"
            class="flex-1"
            placeholder="输入直播源地址或选择文件"
            search-button
            :allow-clear="true"
            @search="onChange()">
            <template #button-icon>
              <icon-file />
            </template>
            <template #button-default>文件</template>
          </a-input-search>
          <a-button size="mini" type="outline" class="ml-2" @click="load">
            加载
          </a-button>
          <a-tooltip
            content="仅测试直播源是否可以打开"
            position="left"
            mini
            :content-style="{ fontSize: '12px' }">
            <a-button size="mini" type="outline" class="ml-2" @click="check">
              测试
            </a-button>
          </a-tooltip>
        </div>
      </form>
      <hr class="mt-6" />
      <div class="content flex-1">
        <a-tabs default-active-key="3" size="mini" class="flex">
          <template #extra>
            <a-button
              type="primary"
              size="mini"
              class="mr-2"
              @click="preview()">
              <small>预览</small>
            </a-button>
            <a-button
              type="outline"
              size="mini"
              class="mr-2"
              @click="recheck()">
              <small>重测无效源</small>
            </a-button>
            <a-button
              type="outline"
              size="mini"
              class="mr-2"
              @click="removeInvalid()">
              <small>删除无效源</small>
            </a-button>
            <a-button
              type="outline"
              size="mini"
              @click="playlist.preAdd = true">
              <small>合并直播源</small>
            </a-button>
          </template>
          <a-tab-pane key="1" :title="`列表 ${stats.online} / ${stats.count}`">
            <div class="flex-y-auto">
              <a-table
                class="h-full"
                row-key="url"
                size="small"
                table-layout-fixed
                sticky-header
                :data="items"
                v-model:selected-keys="playlist.selectedKeys"
                :row-selection="{
                  type: 'checkbox',
                  showCheckedAll: true,
                  onlyCurrent: false,
                }"
                :pagination="{
                  size: 'mini',
                  showTotal: true,
                  showPageSize: true,
                  pageSize: 50,
                }">
                <template #columns>
                  <a-table-column
                    title="频道名称"
                    data-index="name"
                    header-cell-class="text-sm"
                    :width="128"
                    :filterable="{
                      filter: (value, record) => record.name.includes(value),
                      slotName: 'name-filter',
                      icon: () => h(IconSearch),
                    }">
                    <template
                      #filter-content="{
                        filterValue,
                        setFilterValue,
                        handleFilterConfirm,
                        handleFilterReset,
                      }">
                      <div
                        class="custom-filter bg-white p-4 border shadow rounded">
                        <a-space direction="vertical">
                          <a-input
                            size="mini"
                            placeholder="搜索频道名称"
                            :model-value="filterValue[0]"
                            allow-clear
                            @input="(value) => setFilterValue([value])" />
                          <div class="flex flex-row justify-between mt-2">
                            <a-button
                              size="mini"
                              class="mr-4"
                              type="outline"
                              @click="handleFilterReset">
                              重置
                            </a-button>
                            <a-button
                              size="mini"
                              type="primary"
                              @click="handleFilterConfirm">
                              确定
                            </a-button>
                          </div>
                        </a-space>
                      </div>
                    </template>
                    <template #cell="{ record }">
                      <span
                        v-text="record.name"
                        :title="record.name"
                        class="inline-block text-sm truncate w-full"></span>
                    </template>
                  </a-table-column>
                  <a-table-column
                    title="直播源"
                    data-index="url"
                    header-cell-class="text-sm"
                    ellipsis
                    :filterable="{
                      filter: (value, record) => record.url.includes(value),
                      slotName: 'name-filter',
                      icon: () => h(IconSearch),
                    }">
                    <template
                      #filter-content="{
                        filterValue,
                        setFilterValue,
                        handleFilterConfirm,
                        handleFilterReset,
                      }">
                      <div
                        class="custom-filter bg-white p-4 border shadow rounded">
                        <a-space direction="vertical">
                          <a-input
                            size="mini"
                            placeholder="搜索频道地址"
                            :model-value="filterValue[0]"
                            allow-clear
                            @input="(value) => setFilterValue([value])" />
                          <div class="flex flex-row justify-between mt-2">
                            <a-button
                              size="mini"
                              class="mr-4"
                              type="outline"
                              @click="handleFilterReset">
                              重置
                            </a-button>
                            <a-button
                              size="mini"
                              type="primary"
                              @click="handleFilterConfirm">
                              确定
                            </a-button>
                          </div>
                        </a-space>
                      </div>
                    </template>
                    <template #cell="{ record }">
                      <CopyText :text="record.url" class="text-sm"></CopyText>
                    </template>
                  </a-table-column>
                  <a-table-column
                    title="状态"
                    :width="64"
                    header-cell-class="text-sm"
                    :filterable="filterable">
                    <template #cell="{ record }">
                      <a-tooltip
                        :content="
                          record.online == 1
                            ? '当前连接可以打开，无法确定内容是否可以正常观看'
                            : '未测试或无法打开'
                        "
                        position="top"
                        mini
                        :content-style="{ fontSize: '12px' }">
                        <span
                          class="text-sm"
                          v-text="
                            record.online == 1
                              ? 'ok'
                              : record.online == -1
                              ? 'error'
                              : '-'
                          "></span>
                      </a-tooltip>
                    </template>
                  </a-table-column>
                  <a-table-column
                    title="操作"
                    header-cell-class="text-sm"
                    :width="128">
                    <template #cell="{ record }">
                      <span
                        class="px-2 aria-disabled:opacity-40 aria-disabled:pointer-events-none aria-readonly:text-green-600 aria-checked:text-red-500"
                        :aria-disabled="!record.http"
                        :aria-readonly="record.online == 1"
                        :aria-checked="record.online == -1"
                        title="检测"
                        @click="store.check_by([record])">
                        <icon-bug class="" />
                      </span>
                      <span
                        class="px-2 aria-disabled:opacity-40 aria-disabled:pointer-events-none"
                        :aria-disabled="!record.http"
                        title="播放"
                        @click="store.play(record)">
                        <icon-play-circle-fill />
                      </span>
                      <span
                        class="px-2"
                        title="删除"
                        @click="store.remove_by([record])">
                        <icon-close-circle-fill class="text-red-500" />
                      </span>
                    </template>
                  </a-table-column>
                </template>
                <template #pagination-left>
                  <div class="w-full">
                    <a-button
                      type="outline"
                      size="mini"
                      class="mr-2"
                      @click="removeSelected()">
                      <small>删除选中</small>
                    </a-button>
                    <a-button type="outline" size="mini" @click="removeIpv6()">
                      <small>删除IPv6源</small>
                    </a-button>
                  </div>
                </template>
              </a-table>
            </div>
          </a-tab-pane>
          <a-tab-pane key="2" title="原格式">
            <a-textarea
              v-model="text"
              placeholder=""
              :auto-size="false"
              class="text flex-y-auto flex" />
          </a-tab-pane>
          <a-tab-pane key="3" title="分组管理">
            <div class="flex-y-auto">
              <TxtGroup></TxtGroup>
            </div>            
          </a-tab-pane>
        </a-tabs>
      </div>
      <a-modal
        v-model:visible="playlist.preAdd"
        title="合并新的直播源"
        @cancel="playlist.preAdd = false"
        @ok="add">
        <a-form :model="playlist" size="small" auto-label-width>
          <a-form-item field="addSource" label="直播源">
            <a-input-search
              v-model="playlist.addSource"
              placeholder="输入直播源地址或选择文件"
              search-button
              @search="onChange(true)">
              <template #button-icon>
                <icon-file />
              </template>
              <template #button-default>文件</template>
            </a-input-search>
          </a-form-item>
          <p class="text-gray-400 text-sm text-center -mt-2">
            新的直播源将被合并到当前打开的直播源当中
          </p>
        </a-form>
      </a-modal>
      <a-modal
        v-model:visible="playlist.preview"
        title="预览"
        @cancel="playlist.preview = false"
        @ok="playlist.preview = false">
        <TxtPreview v-if="playlist.preview"></TxtPreview>
      </a-modal>
    </div>
  </a-spin>
</template>
<style lang="scss" scoped>
.text {
  ::v-deep() {
    textarea {
      flex: 1;
      resize: none;
      font-size: 12px;
    }
  }
}
.content {
  ::v-deep() {
    span.arco-table-th-title {
      font-size: 12px;
    }
  }
}
</style>
<style lang="scss">
.arco-trigger-popup {
  .arco-table-filters-list {
    .arco-radio-label {
      font-size: 12px;
    }
  }
}
</style>
