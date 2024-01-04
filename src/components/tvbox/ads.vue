<script setup lang="ts">
import { PropType } from "vue";
import List from "../ui/list.vue";
import ListItem from "../ui/list-item.vue";
import HeightAuto from "../ui/height-auto.vue";
import { useTvBoxStore } from "../../store";
const store = useTvBoxStore();
defineProps({
  data: Array as PropType<string[]>,
});
const remove = (i: number) => {
  store.remove_ads(i);
};
</script>
<template>
  <div class="ads p-2 h-full">
    <HeightAuto class="h-full">
      <template #up>
        <div class="text-sm pb-2" v-show="data && data.length">
          屏蔽一下站点广告:
        </div>
      </template>
      <List>
        <ListItem v-for="(item, key) in data" class="first:border-t" :key="key">
          <div class="flex flex-row w-full text-sm">
            <div class="flex-x-auto truncate" v-text="item"></div>
            <span class="px-2" title="删除" @click="remove(key)">
              <a-button
                type="outline"
                status="danger"
                size="mini"
                @click="remove(key)">
                删除
              </a-button>
            </span>
          </div>
        </ListItem>
      </List>
    </HeightAuto>
  </div>
</template>
