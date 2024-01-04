<script setup lang="ts">
import { PropType } from "vue";
defineProps({
  data: Array as PropType<string[]>,
});
</script>
<template>
  <div class="flags text-sm rounded border p-4 shadow w-full">
    <a-space wrap>
      <a-tag bordered color="purple" v-for="item in data">
        {{ item }}
      </a-tag>
    </a-space>
  </div>
</template>
