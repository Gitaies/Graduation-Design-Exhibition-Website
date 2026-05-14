<template>
  <header
    :class="['fixed top-0 left-0 right-0 z-50 nav-header', { 'scrolled': isScrolled, 'menu-open': isMenuOpen }]"
    @mouseenter="onHeaderHover"
    @mouseleave="onHeaderLeave"
  >
    <div class="nav-header-bg"></div>
    <div class="container mx-auto px-4 relative z-10">
      <div class="flex items-center justify-between h-20">
        <!-- Logo -->
        <NuxtLink to="/" class="flex items-center space-x-3 group">
          <img
            src="/images/logo.webp"
            alt="传媒技术学院"
            class="h-12 w-auto transition-transform duration-300 group-hover:scale-105"
          >
          <div class="flex flex-col">
            <span class="text-sm md:text-lg font-semibold text-gray-800 leading-tight">传媒技术学院</span>
            <span class="text-[0.6rem] md:text-xs text-gray-500 tracking-wide">School of Media Technology</span>
          </div>
        </NuxtLink>

        <!-- 桌面端导航菜单 -->
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
        <button
          @click="toggleMenu"
          class="md:hidden relative w-11 h-11 flex items-center justify-center text-gray-700 hover:text-primary-blue transition-colors"
          :aria-label="isMenuOpen ? '关闭菜单' : '打开菜单'"
        >
          <span class="hamburger-box relative w-5 h-4">
            <span :class="['hamburger-line', { 'open': isMenuOpen }]"></span>
            <span :class="['hamburger-line', { 'open': isMenuOpen }]"></span>
            <span :class="['hamburger-line', { 'open': isMenuOpen }]"></span>
          </span>
        </button>
      </div>
    </div>

    <!-- 移动端菜单 -->
    <Transition name="mobile-menu">
      <div v-if="isMenuOpen" class="md:hidden mobile-menu-panel">
        <nav class="container mx-auto px-4 py-4 space-y-1">
          <a
            v-for="item in navItems"
            :key="item.path"
            :href="item.path"
            class="mobile-nav-link"
            @click.prevent="scrollToSection(item.path)"
          >
            {{ item.label }}
          </a>
        </nav>
      </div>
    </Transition>
  </header>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { gsap } from 'gsap'

const route = useRoute()
const router = useRouter()

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
  closeMenu()

  // 非首页 → 先跳转到首页，再滚动到对应区块
  if (route.path !== '/') {
    router.push({ path: '/', hash: path })
    return
  }

  // 已在首页 → 直接滚动
  const sectionId = path.replace('#', '')
  const element = document.getElementById(sectionId)

  if (element) {
    const headerHeight = 80
    const elementPosition = element.getBoundingClientRect().top
    const offsetPosition = elementPosition + window.pageYOffset - headerHeight

    window.scrollTo({
      top: offsetPosition,
      behavior: 'smooth'
    })
  }
}

// 移动端菜单打开时锁定 body 滚动
watch(isMenuOpen, (open) => {
  if (open) {
    document.body.style.overflow = 'hidden'
  } else {
    document.body.style.overflow = ''
  }
})

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

  // 移动端：滚动时关闭菜单
  if (isMenuOpen.value && scrolled) {
    closeMenu()
  }
}

// 监听窗口大小变化，关闭移动端菜单
const handleResize = () => {
  if (window.innerWidth >= 768 && isMenuOpen.value) {
    closeMenu()
  }
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll)
  window.addEventListener('resize', handleResize)
  // 初始检查滚动状态
  handleScroll()
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
  window.removeEventListener('resize', handleResize)
  document.body.style.overflow = ''
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
  if (!isScrolled.value && !isMenuOpen.value) {
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

/* 移动端始终显示背景（无 hover 能力） */
@media (max-width: 767px) {
  .nav-header-bg {
    transform: scaleY(0.92);
    background: rgba(255, 255, 255, 0.88);
  }

  .nav-header.scrolled .nav-header-bg {
    transform: scaleY(1);
    background: rgba(255, 255, 255, 0.95);
  }

  .nav-header.menu-open .nav-header-bg {
    transform: scaleY(1);
    background: rgba(255, 255, 255, 0.98);
  }
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

/* ============================================
   汉堡图标
   ============================================ */
.hamburger-box {
  display: block;
}

.hamburger-line {
  display: block;
  position: absolute;
  left: 0;
  width: 100%;
  height: 2px;
  background-color: currentColor;
  border-radius: 2px;
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.hamburger-line:nth-child(1) {
  top: 0;
}

.hamburger-line:nth-child(2) {
  top: 50%;
  transform: translateY(-50%);
}

.hamburger-line:nth-child(3) {
  bottom: 0;
}

/* 汉堡变 X */
.hamburger-line.open:nth-child(1) {
  top: 50%;
  transform: translateY(-50%) rotate(45deg);
}

.hamburger-line.open:nth-child(2) {
  opacity: 0;
  transform: translateY(-50%) scaleX(0);
}

.hamburger-line.open:nth-child(3) {
  bottom: 50%;
  transform: translateY(50%) rotate(-45deg);
}

/* ============================================
   移动端菜单面板
   ============================================ */
.mobile-menu-panel {
  position: relative;
  background: rgba(255, 255, 255, 0.98);
  backdrop-filter: blur(16px);
  border-top: 1px solid rgba(0, 0, 0, 0.06);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
}

.mobile-nav-link {
  display: flex;
  align-items: center;
  min-height: 3rem;
  padding: 0.5rem 0.75rem;
  font-size: 1.05rem;
  font-weight: 600;
  color: #374151;
  text-decoration: none;
  border-radius: 0.5rem;
  transition: all 0.2s ease;
}

.mobile-nav-link:hover,
.mobile-nav-link:active {
  color: #1466ff;
  background: rgba(20, 102, 255, 0.06);
  padding-left: 1rem;
}

/* ============================================
   移动端菜单过渡动画
   ============================================ */
.mobile-menu-enter-active {
  transition: all 0.3s cubic-bezier(0.22, 1, 0.36, 1);
}

.mobile-menu-leave-active {
  transition: all 0.2s ease-in;
}

.mobile-menu-enter-from {
  opacity: 0;
  transform: translateY(-8px);
}

.mobile-menu-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* ============================================
   响应式
   ============================================ */
@media (max-width: 1024px) {
  .nav-link {
    padding: 0.6rem 0.9rem;
    font-size: 0.875rem;
  }
}

@media (max-width: 768px) {
  .nav-header .container {
    padding-left: 0.75rem;
    padding-right: 0.75rem;
  }

  .nav-header .flex.items-center {
    height: 3.5rem;
  }

  .nav-header img {
    height: 2.5rem;
  }
}

@media (max-width: 480px) {
  .nav-header .flex.items-center {
    height: 3.25rem;
  }

  .nav-header img {
    height: 2rem;
  }

  .nav-header .flex-col span:first-child {
    font-size: 0.8rem;
  }

  .nav-header .flex-col span:last-child {
    font-size: 0.5rem;
  }
}
</style>
