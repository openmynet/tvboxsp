<script setup lang="ts">
import { PropType } from "vue";
import List from "../ui/list.vue";
import ListItem from "../ui/list-item.vue";
defineProps({
  data: Array as PropType<TvBoxIjk[]>,
});
</script>
<template>
  <div class="ijk h-full w-full">
    <list class="h-full w-full flex flex-col">
      <ListItem class="text-sm w-full" v-for="item in data">
        <div class="group">
          <span class="font-bold">{{ item.group }}</span>
          配置参数：
        </div>
        <div class="flex-auto">
          <div v-for="opt in item.options">
            <div class="truncate">
              category:{{ opt.category }},&nbsp; {{ opt.name }} :
              {{ opt.value }}
            </div>
          </div>
        </div>
      </ListItem>
    </list>
  </div>
</template>
