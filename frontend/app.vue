<template>
  <div class="min-h-screen">
    <NuxtPage />
  </div>
</template>

<script setup lang="ts">
// 初始化游客ID
const visitorStore = useVisitorStore()
visitorStore.initVisitorId()
</script>
