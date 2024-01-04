<template>
  <div class="flex flex-row">
    <slot name="head"></slot>
    <div class="flex-x-auto"><slot></slot></div>
    <slot name="tail"></slot>
  </div>
</template>
