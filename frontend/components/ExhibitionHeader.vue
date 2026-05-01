<template>
  <header
    :class="['fixed top-0 left-0 right-0 z-50 nav-header', { 'scrolled': isScrolled }]"
    @mouseenter="onHeaderHover"
    @mouseleave="onHeaderLeave"
  >
    <div class="nav-header-bg"></div>
    <div class="container mx-auto px-4 relative z-10">
      <div class="flex items-center justify-between h-20">
        <!-- Logo -->
        <NuxtLink to="/" class="flex items-center space-x-3 group">
          <img
            src="/images/logo.png"
            alt="传媒技术学院"
            class="h-12 w-auto transition-transform duration-300 group-hover:scale-105"
          >
          <div class="hidden md:flex flex-col">
            <span class="text-lg font-semibold text-gray-800 leading-tight">传媒技术学院</span>
            <span class="text-xs text-gray-500 tracking-wide">School of Media Technology</span>
          </div>
        </NuxtLink>

        <!-- 导航菜单 -->
        <nav class="hidden md:flex items-center space-x-1">
          <a
            v-for="item in navItems"
            :key="item.path"
            :href="item.path"
            class="nav-link"
            @click.prevent="scrollToSection(item.path)"
            @mouseenter="onNavHover"
            @mouseleave="onNavLeave"
          >
            <span class="nav-link-text">{{ item.label }}</span>
            <span class="nav-link-line"></span>
          </a>
        </nav>

        <!-- 移动端菜单按钮 -->
        <button @click="toggleMenu" class="md:hidden p-2 text-gray-700 hover:text-primary-blue transition-colors">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
          </svg>
        </button>
      </div>
    </div>

    <!-- 移动端菜单 -->
    <div v-if="isMenuOpen" class="md:hidden bg-white/95 backdrop-blur-md border-t border-gray-200">
      <nav class="container mx-auto px-4 py-4 space-y-3">
        <a
          v-for="item in navItems"
          :key="item.path"
          :href="item.path"
          class="block py-2 text-gray-600 hover:text-primary-blue transition-colors"
          @click.prevent="scrollToSection(item.path)"
        >
          {{ item.label }}
        </a>
      </nav>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { gsap } from 'gsap'

const isMenuOpen = ref(false)
const isScrolled = ref(false)

const navItems = [
  { path: '#hero', label: '首页' },
  { path: '#majors', label: '专业展区' },
  { path: '#ranking', label: '点赞热榜' },
  { path: '#about', label: '关于毕展' }
]

const toggleMenu = () => {
  isMenuOpen.value = !isMenuOpen.value
}

const closeMenu = () => {
  isMenuOpen.value = false
}

const scrollToSection = (path: string) => {
  const sectionId = path.replace('#', '')
  const element = document.getElementById(sectionId)

  if (element) {
    const headerHeight = 80 // 导航栏高度
    const elementPosition = element.getBoundingClientRect().top
    const offsetPosition = elementPosition + window.pageYOffset - headerHeight

    window.scrollTo({
      top: offsetPosition,
      behavior: 'smooth'
    })
  }

  closeMenu()
}

// 监听滚动事件
const handleScroll = () => {
  const scrolled = window.scrollY > 50
  isScrolled.value = scrolled

  // 滚动时控制背景动画
  const bg = document.querySelector('.nav-header-bg') as HTMLElement
  if (bg) {
    gsap.to(bg, {
      scaleY: scrolled ? 1 : 0,
      duration: 1,
      ease: 'power2.out'
    })
  }
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll)
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})

const onHeaderHover = () => {
  if (!isScrolled.value) {
    const bg = document.querySelector('.nav-header-bg') as HTMLElement
    if (bg) {
      gsap.to(bg, {
        scaleY: 1,
        duration: 0.5,
        ease: 'power2.out'
      })
    }
  }
}

const onHeaderLeave = () => {
  if (!isScrolled.value) {
    const bg = document.querySelector('.nav-header-bg') as HTMLElement
    if (bg) {
      gsap.to(bg, {
        scaleY: 0,
        duration: 0.5,
        ease: 'power2.out'
      })
    }
  }
}

const onNavHover = (event: MouseEvent) => {
  const target = event.currentTarget as HTMLElement
  const line = target.querySelector('.nav-link-line') as HTMLElement

  if (line) {
    gsap.to(line, {
      scaleX: 1,
      duration: 0.4,
      ease: 'power2.out'
    })
  }
}

const onNavLeave = (event: MouseEvent) => {
  const target = event.currentTarget as HTMLElement
  const line = target.querySelector('.nav-link-line') as HTMLElement

  if (line) {
    gsap.to(line, {
      scaleX: 0,
      duration: 0.4,
      ease: 'power2.out'
    })
  }
}
</script>

<style scoped>
.nav-header {
  overflow: hidden;
  background: transparent;
}

.nav-header-bg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(12px);
  transform: scaleY(0);
  transform-origin: top;
  z-index: 0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.nav-link {
  position: relative;
  display: inline-flex;
  flex-direction: column;
  align-items: center;
  padding: 0.75rem 1.25rem;
  color: #374151;
  font-weight: 600;
  transition: color 0.3s ease;
}

.nav-link:hover {
  color: #1466ff;
}

.nav-link-text {
  position: relative;
  z-index: 1;
  text-shadow: 0 1px 2px rgba(255, 255, 255, 0.8);
}

.nav-link-line {
  position: absolute;
  bottom: 0.5rem;
  left: 50%;
  width: 100%;
  height: 2px;
  background: linear-gradient(90deg, transparent, #1466ff 50%, transparent);
  transform: translateX(-50%) scaleX(0);
  transform-origin: center;
  z-index: 1;
}
</style>
