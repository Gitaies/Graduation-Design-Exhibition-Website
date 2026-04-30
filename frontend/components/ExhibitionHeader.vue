<template>
  <header class="fixed top-0 left-0 right-0 z-50 glass-card">
    <div class="container mx-auto px-4">
      <div class="flex items-center justify-between h-16">
        <!-- Logo -->
        <NuxtLink to="/" class="flex items-center space-x-3">
          <div class="w-10 h-10 bg-primary-blue rounded-lg flex items-center justify-center">
            <span class="text-white font-bold text-xl">传</span>
          </div>
          <span class="text-lg font-semibold hidden md:block">传媒技术学院毕业设计展</span>
        </NuxtLink>

        <!-- 导航菜单 -->
        <nav class="hidden md:flex items-center space-x-8">
          <NuxtLink to="/" class="text-text-secondary hover:text-primary-blue transition-colors">首页</NuxtLink>
          <NuxtLink to="/majors" class="text-text-secondary hover:text-primary-blue transition-colors">专业展区</NuxtLink>
          <NuxtLink to="/ranking" class="text-text-secondary hover:text-primary-blue transition-colors">点赞热榜</NuxtLink>
          <NuxtLink to="/about" class="text-text-secondary hover:text-primary-blue transition-colors">关于毕展</NuxtLink>
        </nav>

        <!-- 移动端菜单按钮 -->
        <button @click="toggleMenu" class="md:hidden p-2">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
          </svg>
        </button>
      </div>
    </div>

    <!-- 移动端菜单 -->
    <div v-if="isMenuOpen" class="md:hidden border-t border-border-light">
      <nav class="container mx-auto px-4 py-4 space-y-3">
        <NuxtLink to="/" class="block py-2 text-text-secondary hover:text-primary-blue transition-colors" @click="closeMenu">首页</NuxtLink>
        <NuxtLink to="/majors" class="block py-2 text-text-secondary hover:text-primary-blue transition-colors" @click="closeMenu">专业展区</NuxtLink>
        <NuxtLink to="/ranking" class="block py-2 text-text-secondary hover:text-primary-blue transition-colors" @click="closeMenu">点赞热榜</NuxtLink>
        <NuxtLink to="/about" class="block py-2 text-text-secondary hover:text-primary-blue transition-colors" @click="closeMenu">关于毕展</NuxtLink>
      </nav>
    </div>
  </header>
</template>

<script setup lang="ts">
const isMenuOpen = ref(false)

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value
}

const closeMenu = () => {
  isMenuOpen.value = false
}
</script>
