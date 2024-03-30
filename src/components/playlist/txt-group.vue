<script setup lang="ts">
// import { Message } from "@arco-design/web-vue";
import { useTxtPlaylistStore } from "../../store";
import { reactive } from "vue";
import { confirm } from "../../utils";
import { Message } from "@arco-design/web-vue";
import CopyText from "../ui/copy-text.vue";
import List from "../ui/list.vue";
import ListItem from "../ui/list-item.vue";

const store = useTxtPlaylistStore();

const state = reactive({
  group_hash: "",
  items: [] as TxtPlaylist[],
  selectedKeys: [] as string[],
  move: false,
});
const onSelect = (item: ITxtPlaylistGroup) => {
  state.group_hash = item.raw.hash || "";
  state.items = item.items;
};
const onMove = () => {
  state.move = true;
};
const onSort = () => {
  state.items.sort((a, b) => {
    const chna = parseInt(a.name.replace(/[^\d]+/g, ""));
    const chnb = parseInt(b.name.replace(/[^\d]+/g, ""));
    const s = chna - chnb;
    if (isNaN(s) || s == 0) {
      return a.name.localeCompare(b.name);
    } else {
      return s;
    }
  });
  const group = store.group;
  group.forEach((g) => {
    if (g.raw.hash == state.group_hash) {
      g.items.sort((a, b) => {
        const chna = parseInt(a.name.replace(/[^\d]+/g, ""));
        const chnb = parseInt(b.name.replace(/[^\d]+/g, ""));
        const s = chna - chnb;
        if (isNaN(s) || s == 0) {
          return a.name.localeCompare(b.name);
        } else {
          return s;
        }
      });
    }
  });
  store.group = group;
};
const onCallMove = (item: ITxtPlaylistGroup) => {
  store.group_move(state.selectedKeys, state.group_hash, item.raw.hash);
  state.move = false;
  state.items = state.items.filter(
    (i: TxtPlaylist) => !state.selectedKeys.includes(i.hash)
  );
  state.selectedKeys = [];
  state.group_hash = item.raw.hash;
};
const removeSelected = async () => {
  await confirm("确定要删除选中项吗");
  if (!state.selectedKeys.length) {
    return;
  }
  const items = store.content.filter((item: TxtPlaylist) =>
    state.selectedKeys.includes(item.hash)
  );
  if (!items.length) {
    return;
  }
  store.remove_by(items);
  Message.success("删除完成");
};
const removeGroup = async (i: number, item: ITxtPlaylistGroup) => {
  if (item.items.length) {
    return;
  }
  store.group.splice(i, 1);
};
</script>
<template>
  <div class="spider w-full h-full flex flex-row">
    <div class="txt-group-left w-72 min-w-72 h-full">
      <list class="h-full text-sm">
        <ListItem
          v-for="(item, key) in store.group"
          :key="key"
          @click="onSelect(item)"
          :aria-checked="item.raw.hash == state.group_hash"
        >
          <div class="flex flex-row w-full">
            <span class="flex-1"
              >{{ item.group }}({{ item.items.length }})</span
            >
            <span
              class="px-2"
              title="删除"
              @click="removeGroup(key, item)"
              v-show="!item.items.length"
            >
              <icon-close-circle-fill class="text-red-500" />
            </span>
          </div>
        </ListItem>
      </list>
    </div>
    <div class="txt-group-right flex-auto">
      <a-table
        class="h-full"
        row-key="hash"
        size="small"
        table-layout-fixed
        sticky-header
        :data="state.items"
        v-model:selected-keys="state.selectedKeys"
        :row-selection="{
          type: 'checkbox',
          showCheckedAll: true,
          onlyCurrent: false,
        }"
        :pagination="{
          size: 'mini',
          showTotal: true,
          showPageSize: true,
          pageSize: 200,
        }"
      >
        <template #columns>
          <a-table-column
            title="频道名称"
            data-index="name"
            header-cell-class="text-sm"
            :width="128"
          >
            <template #cell="{ record }">
              <span
                v-text="record.name"
                :title="record.name"
                class="inline-block text-sm truncate w-full"
              ></span>
            </template>
          </a-table-column>
          <a-table-column
            title="直播源"
            data-index="url"
            header-cell-class="text-sm"
            ellipsis
          >
            <template #cell="{ record }">
              <CopyText :text="record.url" class="text-sm"></CopyText>
            </template>
          </a-table-column>

          <a-table-column title="操作" header-cell-class="text-sm" :width="128">
            <template #cell="{ record }">
              <span
                class="px-2 aria-disabled:opacity-40 aria-disabled:pointer-events-none aria-readonly:text-green-600 aria-checked:text-red-500"
                :aria-disabled="!record.http"
                :aria-readonly="record.online == 1"
                :aria-checked="record.online == -1"
                title="检测"
                @click="store.check_by([record])"
              >
                <icon-bug class="" />
              </span>
              <span
                class="px-2 aria-disabled:opacity-40 aria-disabled:pointer-events-none"
                :aria-disabled="!record.http"
                title="播放"
                @click="store.play(record)"
              >
                <icon-play-circle-fill />
              </span>
              <span
                class="px-2"
                title="删除"
                @click="store.remove_by([record])"
              >
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
              @click="removeSelected()"
            >
              <small>删除选中</small>
            </a-button>
            <a-button type="outline" size="mini" class="mr-2" @click="onMove">
              <small>转移选中</small>
            </a-button>
            <a-button type="outline" size="mini" class="mr-2" @click="onSort">
              <small>频道排序</small>
            </a-button>
          </div>
        </template>
      </a-table>
    </div>
    <a-modal
      v-model:visible="state.move"
      title="将选择的频道转移到以下分组"
      @cancel="state.move = false"
      @ok="state.move = false"
    >
      <list class="h-full text-sm">
        <ListItem
          v-for="(item, key) in store.group"
          :key="key"
          @click="onCallMove(item)"
          :aria-checked="item.raw.hash == state.group_hash"
        >
          <span v-text="item.group"></span>
          <span>({{ item.items.length }})</span>
        </ListItem>
      </list>
    </a-modal>
  </div>
</template>
