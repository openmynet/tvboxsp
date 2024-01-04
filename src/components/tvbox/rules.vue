<script setup lang="ts">
import { PropType } from "vue";
import List from "../ui/list.vue";
defineProps({
  data: Array as PropType<TvBoxRule[]>,
});
</script>
<template>
  <div class="rules h-full">
    <List class="h-full">
      <div
        class="flex-1 p-2 transition-colors duration-200 hover:bg-slate-200 text-sm border-b"
        v-for="item in data">
        <div class="name" v-if="item.name">规则名称：{{ item.name }}</div>
        <div class="hosts" v-if="item.hosts">
          生效站点：{{ item.hosts?.join(",") }}
        </div>

        <div class="regex flex flex-row" v-if="item.regex">
          <span>正则匹配：</span>
          <pre
            class="whitespace-pre-wrap break-all"
            v-text="item.regex?.join('\n')"></pre>
        </div>
        <div class="hosts" v-if="item.host">生效站点：{{ item.host }}</div>
        <div class="regex flex flex-row" v-if="item.rule">
          <span>匹配规则：</span>
          <pre
            class="whitespace-pre-wrap break-all"
            v-text="item.rule?.join('\n')"></pre>
        </div>
      </div>
    </List>
  </div>
</template>
