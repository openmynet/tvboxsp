<template>
  <div class="flex flex-col">
    <slot name="up"></slot>
    <div class="flex-y-auto"><slot></slot></div>
    <slot name="down"></slot>
  </div>
</template>
