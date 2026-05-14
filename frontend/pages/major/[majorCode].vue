<template>
  <div class="major-works-page min-h-screen bg-bg-page">
    <ExhibitionHeader />

    <!-- 专业主题横幅 -->
    <section :class="['major-hero', `major-hero-${majorCode}`]">
      <div class="container mx-auto px-4">
        <!-- 面包屑 -->
        <nav class="hero-breadcrumb breadcrumb-anim">
          <NuxtLink to="/" class="hero-breadcrumb-link">首页</NuxtLink>
          <span class="hero-breadcrumb-sep">/</span>
          <NuxtLink to="/#majors" class="hero-breadcrumb-link">专业展区</NuxtLink>
          <span class="hero-breadcrumb-sep">/</span>
          <span class="hero-breadcrumb-current">{{ majorInfo?.name }}</span>
        </nav>

        <!-- 主标题区 -->
        <div class="hero-body">
          <!-- 左侧文字区 -->
          <div class="hero-left">
            <!-- 巨型编号 -->
            <div class="hero-index" data-anim="index" aria-hidden="true">
              <span class="hero-index-num">{{ majorNumber }}</span>
              <span class="hero-index-line"></span>
            </div>

            <!-- 核心文字 -->
            <div class="hero-text-block">
              <div class="hero-label-row" data-anim="label">
                <span class="hero-label">{{ majorInfo?.code.toUpperCase() }}</span>
                <span class="hero-label-dash"></span>
              </div>

              <h1 class="hero-name" data-anim="name">
                <span class="hero-name-main">{{ majorInfo?.name }}</span>
              </h1>

              <div class="hero-theme-row" data-anim="theme">
                <span class="hero-theme-prefix">—</span>
                <p class="hero-theme">{{ majorInfo?.theme }}</p>
              </div>

              <p class="hero-subtitle" data-anim="sub">{{ majorInfo?.subtitle }}</p>
            </div>

            <!-- 底部描述 + 装饰线 -->
            <div class="hero-footer" data-anim="footer">
              <div class="hero-divider"></div>
              <p class="hero-desc">{{ majorInfo?.description }}</p>
            </div>
          </div>

          <!-- 右侧几何装饰 -->
          <div class="hero-right" data-anim="decor" aria-hidden="true">
            <!-- 主几何环 -->
            <div class="geo-ring"></div>
            <div class="geo-ring geo-ring-inner"></div>

            <!-- 中心区域 -->
            <div class="geo-core">
              <div class="geo-core-dot"></div>
              <div class="geo-core-pulse"></div>
            </div>

            <!-- 轨道小点 -->
            <div class="geo-orbit geo-orbit-1"></div>
            <div class="geo-orbit geo-orbit-2"></div>
            <div class="geo-orbit geo-orbit-3"></div>

            <!-- 散落圆点 -->
            <div class="geo-dot geo-dot-1"></div>
            <div class="geo-dot geo-dot-2"></div>
            <div class="geo-dot geo-dot-3"></div>

            <!-- 十字标记 -->
            <div class="geo-cross geo-cross-1">
              <span></span><span></span>
            </div>
            <div class="geo-cross geo-cross-2">
              <span></span><span></span>
            </div>

            <!-- 竖排英文 -->
            <div class="geo-vert-text">MEDIA · TECHNOLOGY · {{ majorInfo?.code.toUpperCase() }}</div>
          </div>
        </div>
      </div>

      <!-- 背景网格纹理 -->
      <div class="hero-grid-texture" aria-hidden="true"></div>
    </section>

    <!-- 筛选和作品列表 -->
    <section class="py-12">
      <div class="container mx-auto px-4">
        <!-- 筛选标签 -->
        <div class="mb-8 flex flex-wrap gap-3">
          <button
            v-for="tag in filterTags"
            :key="tag"
            @click="selectedTag = tag"
            :class="[
              'px-6 py-2.5 rounded-full text-sm font-medium transition-all duration-200',
              selectedTag === tag
                ? 'bg-primary-blue text-white shadow-lg shadow-primary-blue/30 scale-105'
                : 'bg-white/80 backdrop-blur-sm text-text-secondary hover:bg-white hover:text-primary-blue hover:shadow-md'
            ]"
          >
            {{ tag }}
          </button>
        </div>

        <!-- 作品统计 -->
        <div class="mb-6 text-text-secondary">
          共 <span class="text-primary-blue font-semibold">{{ filteredWorks.length }}</span> 个作品
        </div>

        <!-- 作品网格 -->
        <div v-if="filteredWorks.length > 0" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 md:gap-8">
          <WorkCard
            v-for="work in paginatedWorks"
            :key="work.id"
            :work="work"
          />
        </div>

        <!-- 空状态 -->
        <div v-else class="text-center py-20">
          <div class="text-6xl mb-4">🔍</div>
          <p class="text-xl text-text-secondary mb-2">暂无相关作品</p>
          <p class="text-text-light">试试其他分类标签</p>
        </div>

        <!-- 分页器 -->
        <div v-if="totalPages > 1" class="mt-12 flex justify-center items-center gap-2">
          <button
            @click="currentPage--"
            :disabled="currentPage === 1"
            :class="[
              'px-4 py-2 rounded-lg font-medium transition-all',
              currentPage === 1
                ? 'bg-gray-100 text-gray-400 cursor-not-allowed'
                : 'bg-white text-text-main hover:bg-primary-blue hover:text-white shadow-sm'
            ]"
          >
            上一页
          </button>

          <div class="flex gap-2">
            <button
              v-for="page in displayPages"
              :key="page"
              @click="currentPage = page"
              :class="[
                'w-10 h-10 rounded-lg font-medium transition-all',
                currentPage === page
                  ? 'bg-primary-blue text-white shadow-lg shadow-primary-blue/30'
                  : 'bg-white text-text-main hover:bg-primary-blue/10'
              ]"
            >
              {{ page }}
            </button>
          </div>

          <button
            @click="currentPage++"
            :disabled="currentPage === totalPages"
            :class="[
              'px-4 py-2 rounded-lg font-medium transition-all',
              currentPage === totalPages
                ? 'bg-gray-100 text-gray-400 cursor-not-allowed'
                : 'bg-white text-text-main hover:bg-primary-blue hover:text-white shadow-sm'
            ]"
          >
            下一页
          </button>
        </div>
      </div>
    </section>

    <ExhibitionFooter />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { gsap } from 'gsap'

const route = useRoute()
const majorCode = computed(() => route.params.majorCode as string)

const majorNumber = computed(() => {
  const map: Record<string, string> = { software: '01', electronic: '02', broadcast: '03' }
  return map[majorCode.value] || '01'
})

// 专业信息配置
const majorsConfig = {
  software: {
    code: 'software',
    name: '软件工程',
    theme: '数智焕新',
    subtitle: '数智驱动 · 焕新应用',
    description: '以软件创新为引擎，融合智能技术与数据能力，打造更高效、更智能、更美好的应用体验。',
    tags: ['全部作品', '人工智能', '大数据', '软件开发', 'Web应用', '其他']
  },
  electronic: {
    code: 'electronic',
    name: '电子信息工程',
    theme: '芯火智造',
    subtitle: '芯火相传 · 智造终端',
    description: '聚焦嵌入式、物联网、通信与硬件系统的创新应用。',
    tags: ['全部作品', '嵌入式', '物联网', '智能硬件', '通信系统', '其他']
  },
  broadcast: {
    code: 'broadcast',
    name: '广播电视工程',
    theme: '虚实共生',
    subtitle: '虚实共生 · 视界新生',
    description: '聚焦视音频制作、虚拟现实与数字媒体的融合创新。',
    tags: ['全部作品', '虚拟制作', '影像交互', '数字媒体', '智能媒资', '其他']
  }
}

const majorIconIndex = computed(() => {
  const map: Record<string, number> = { software: 1, electronic: 2, broadcast: 3 }
  return map[majorCode.value] || 1
})

const majorInfo = computed(() => majorsConfig[majorCode.value as keyof typeof majorsConfig])
const filterTags = computed(() => majorInfo.value?.tags || [])

// 使用 API 加载作品数据
const { fetchWorks } = useWorks()
const interactionsStore = useInteractionsStore()
const selectedTag = ref('全部作品')
const currentPage = ref(1)
const pageSize = 9

const allWorks = ref<any[]>([])
const loading = ref(true)

// 加载作品列表
const loadWorks = async () => {
  try {
    loading.value = true
    const response = await fetchWorks({
      major: majorCode.value,
      page: 1,
      page_size: 100 // 获取所有作品用于前端筛选
    })
    allWorks.value = response.items

    // 批量获取交互统计
    const workIds = response.items.map((work: any) => work.id)
    if (workIds.length > 0) {
      await interactionsStore.fetchSummary(workIds)
    }
  } catch (error) {
    console.error('Failed to load works:', error)
  } finally {
    loading.value = false
  }
}

// 初始加载
await loadWorks()

// 筛选逻辑
const filteredWorks = computed(() => {
  let works = allWorks.value

  if (selectedTag.value !== '全部作品') {
    works = works.filter((work: any) => work.tags.includes(selectedTag.value))
  }

  return works
})

const totalPages = computed(() => Math.ceil(filteredWorks.value.length / pageSize))

const paginatedWorks = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  const end = start + pageSize
  return filteredWorks.value.slice(start, end)
})

const displayPages = computed(() => {
  const pages = []
  const total = totalPages.value
  const current = currentPage.value

  if (total <= 7) {
    for (let i = 1; i <= total; i++) {
      pages.push(i)
    }
  } else {
    if (current <= 4) {
      for (let i = 1; i <= 5; i++) pages.push(i)
      pages.push('...')
      pages.push(total)
    } else if (current >= total - 3) {
      pages.push(1)
      pages.push('...')
      for (let i = total - 4; i <= total; i++) pages.push(i)
    } else {
      pages.push(1)
      pages.push('...')
      for (let i = current - 1; i <= current + 1; i++) pages.push(i)
      pages.push('...')
      pages.push(total)
    }
  }

  return pages.filter(p => p !== '...')
})

// 重置分页
watch(selectedTag, () => {
  currentPage.value = 1
})

// 监听专业代码变化，重新加载数据
watch(majorCode, () => {
  loadWorks()
  selectedTag.value = '全部作品'
  currentPage.value = 1
})

// GSAP 入场动画
onMounted(() => {
  const tl = gsap.timeline({ defaults: { ease: 'power3.out' } })

  tl.from('[data-anim="index"]', {
    opacity: 0,
    x: -40,
    duration: 0.8,
  }, '-=0.3')
  tl.from('[data-anim="label"]', {
    opacity: 0,
    y: 12,
    duration: 0.5,
  }, '-=0.4')
  tl.from('[data-anim="name"]', {
    opacity: 0,
    y: 28,
    duration: 0.7,
  }, '-=0.2')
  tl.from('[data-anim="theme"]', {
    opacity: 0,
    x: -16,
    duration: 0.55,
  }, '-=0.3')
  tl.from('[data-anim="sub"]', {
    opacity: 0,
    y: 12,
    duration: 0.5,
  }, '-=0.2')
  tl.from('[data-anim="footer"]', {
    opacity: 0,
    y: 20,
    duration: 0.65,
  }, '-=0.2')
  tl.from('[data-anim="decor"]', {
    opacity: 0,
    scale: 0.88,
    duration: 0.9,
    ease: 'power2.out',
  }, '-=0.5')
})

useHead({
  title: `${majorInfo.value?.name} - 传媒技术学院2026届毕业设计展`
})
</script>

<style scoped>
/* ============================================
   专业主题横幅 — Typographic Editorial
   ============================================ */
.major-hero {
  position: relative;
  padding: 7rem 0 5rem;
  overflow: hidden;
  isolation: isolate;
  background: oklch(0.985 0.002 250);
}

/* 背景网格纹理 */
.hero-grid-texture {
  position: absolute;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  opacity: 0.25;
  background-image:
    linear-gradient(oklch(0.75 0.02 255) 1px, transparent 1px),
    linear-gradient(90deg, oklch(0.75 0.02 255) 1px, transparent 1px);
  background-size: 64px 64px;
  background-position: -1px -1px;
  mask-image: radial-gradient(ellipse 60% 60% at 50% 40%, black 30%, transparent 70%);
}

/* 面包屑 */
.hero-breadcrumb {
  display: flex;
  align-items: center;
  flex-wrap: nowrap;
  gap: 0.5rem;
  margin-bottom: 3rem;
  font-size: 0.8rem;
  line-height: 1.4;
  position: relative;
  z-index: 1;
}

.hero-breadcrumb-link {
  color: oklch(0.48 0.04 250);
  transition: color 0.2s ease;
  text-decoration: none;
  white-space: nowrap;
  flex-shrink: 0;
  display: flex;
  align-items: center;
}
.hero-breadcrumb-link:hover {
  color: #1466ff;
}

.hero-breadcrumb-sep {
  color: oklch(0.7 0.02 250);
  flex-shrink: 0;
  user-select: none;
  display: flex;
  align-items: center;
}

.hero-breadcrumb-current {
  color: oklch(0.25 0.01 250);
  font-weight: 600;
  white-space: nowrap;
  display: flex;
  align-items: center;
}

/* 修复移动端触摸设备上 <a> 标签被全局 min-height/min-width 撑大导致面包屑对不齐 */
@media (hover: none) and (pointer: coarse) {
  .hero-breadcrumb-link {
    min-height: 0;
    min-width: 0;
  }
}

/* 面包屑 CSS 入场动画（避免 GSAP transform 破坏 flex 布局） */
.breadcrumb-anim {
  animation: breadcrumbFadeIn 0.6s ease-out both;
}
@keyframes breadcrumbFadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

/* 主体区域 — 左右两栏 */
.hero-body {
  position: relative;
  z-index: 1;
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  align-items: start;
}

.hero-left {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

/* 巨型编号 */
.hero-index {
  display: flex;
  align-items: flex-start;
  gap: 1.25rem;
}

.hero-index-num {
  font-size: clamp(5rem, 10vw, 8rem);
  font-weight: 900;
  line-height: 0.85;
  color: oklch(0.55 0.20 265);
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
  letter-spacing: -0.04em;
}

.hero-index-line {
  display: block;
  width: 2px;
  height: clamp(4rem, 8vw, 6.5rem);
  background: oklch(0.55 0.20 265 / 0.3);
  margin-top: 0.5rem;
  border-radius: 1px;
}

/* 文字块 */
.hero-text-block {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

/* 标签行 */
.hero-label-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.25rem;
}

.hero-label {
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.15em;
  color: oklch(0.50 0.12 250);
  text-transform: uppercase;
}

.hero-label-dash {
  width: 2.5rem;
  height: 1.5px;
  background: oklch(0.50 0.12 250 / 0.35);
  border-radius: 1px;
}

/* 专业名称 */
.hero-name-main {
  font-size: clamp(2.25rem, 5vw, 3.5rem);
  font-weight: 900;
  color: oklch(0.12 0.01 250);
  letter-spacing: -0.03em;
  line-height: 1.05;
}

/* 主题行 */
.hero-theme-row {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

.hero-theme-prefix {
  font-size: 1.5rem;
  font-weight: 300;
  color: oklch(0.55 0.15 260);
  line-height: 1;
}

.hero-theme {
  font-size: clamp(1.5rem, 3vw, 2.25rem);
  font-weight: 800;
  color: oklch(0.55 0.20 265);
  letter-spacing: -0.02em;
  line-height: 1.1;
  margin: 0;
}

/* 副标题 */
.hero-subtitle {
  font-size: clamp(0.95rem, 1.5vw, 1.1rem);
  font-weight: 500;
  color: oklch(0.40 0.03 250);
  letter-spacing: 0.03em;
  margin-top: 0.5rem;
}

/* 底部分隔线 + 描述 */
.hero-footer {
  position: relative;
  z-index: 1;
  max-width: 560px;
}

.hero-divider {
  width: 100%;
  height: 1px;
  background: oklch(0.75 0.02 255);
  margin-bottom: 1.25rem;
}

.hero-desc {
  font-size: 0.9rem;
  color: oklch(0.45 0.03 250);
  line-height: 1.75;
  max-width: 48ch;
}

/* ============================================
   右侧几何装饰
   ============================================ */
.hero-right {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 360px;
}

/* 主几何环 */
.geo-ring,
.geo-ring-inner {
  position: absolute;
  border-radius: 50%;
  pointer-events: none;
}

.geo-ring {
  width: 280px;
  height: 280px;
  border: 1.5px solid oklch(0.55 0.20 265 / 0.22);
  animation: geo-ring-rotate 28s linear infinite;
}

.geo-ring-inner {
  width: 180px;
  height: 180px;
  border: 1.5px dashed oklch(0.55 0.20 265 / 0.22);
  animation: geo-ring-rotate 20s linear infinite reverse;
}

@keyframes geo-ring-rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* 中心点 */
.geo-core {
  position: absolute;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  pointer-events: none;
  opacity: 0.3;
}

.geo-core-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: oklch(0.55 0.20 265 / 0.55);
  z-index: 2;
}

.geo-core-pulse {
  position: absolute;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: oklch(0.55 0.20 265 / 0.35);
  animation: core-pulse 2.5s ease-in-out infinite;
}

@keyframes core-pulse {
  0%, 100% { transform: scale(1); opacity: 0.5; }
  50% { transform: scale(20); opacity: 0; }
}

/* 轨道小点 */
.geo-orbit {
  position: absolute;
  border-radius: 50%;
  pointer-events: none;
}

.geo-orbit-1 {
  width: 5px;
  height: 5px;
  background: oklch(0.55 0.20 265 / 0.35);
  top: 42%;
  left: 30%;
  animation: orbit-float 4s ease-in-out infinite;
}

.geo-orbit-2 {
  width: 4px;
  height: 4px;
  background: oklch(0.50 0.15 260 / 0.3);
  top: 55%;
  left: 65%;
  animation: orbit-float 3.5s ease-in-out infinite 1s;
}

.geo-orbit-3 {
  width: 6px;
  height: 6px;
  background: oklch(0.55 0.18 260 / 0.32);
  top: 35%;
  left: 58%;
  animation: orbit-float 4.5s ease-in-out infinite 0.5s;
}

@keyframes orbit-float {
  0%, 100% { transform: translate(0, 0); opacity: 0.3; }
  50% { transform: translate(4px, -6px); opacity: 0.6; }
}

/* 散落圆点 */
.geo-dot {
  position: absolute;
  border-radius: 50%;
  pointer-events: none;
}

.geo-dot-1 {
  width: 10px;
  height: 10px;
  background: oklch(0.55 0.20 265 / 0.45);
  top: 15%;
  right: 25%;
  animation: geo-dot-float 5s ease-in-out infinite;
}

.geo-dot-2 {
  width: 6px;
  height: 6px;
  background: oklch(0.50 0.12 250 / 0.35);
  bottom: 28%;
  right: 15%;
  animation: geo-dot-float 4.5s ease-in-out infinite 0.8s;
}

.geo-dot-3 {
  width: 8px;
  height: 8px;
  background: oklch(0.55 0.18 260 / 0.4);
  top: 55%;
  right: 35%;
  animation: geo-dot-float 5.5s ease-in-out infinite 1.5s;
}

@keyframes geo-dot-float {
  0%, 100% { transform: translate(0, 0); opacity: 0.4; }
  50% { transform: translate(6px, -8px); opacity: 0.75; }
}

/* 十字标记 */
.geo-cross {
  position: absolute;
  pointer-events: none;
}

.geo-cross span {
  position: absolute;
  background: oklch(0.55 0.20 265 / 0.45);
}

.geo-cross span:first-child {
  width: 1.5px;
  height: 20px;
  left: 9px;
  top: 0;
}

.geo-cross span:last-child {
  width: 20px;
  height: 1.5px;
  left: 0;
  top: 9px;
}

.geo-cross-1 {
  top: 10%;
  right: 10%;
  animation: geo-cross-pulse 6s ease-in-out infinite;
}

.geo-cross-2 {
  bottom: 22%;
  right: 32%;
  animation: geo-cross-pulse 6s ease-in-out infinite 2s;
}

@keyframes geo-cross-pulse {
  0%, 100% { opacity: 0.35; transform: scale(1); }
  50% { opacity: 0.7; transform: scale(1.2); }
}

/* 竖排英文 */
.geo-vert-text {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  writing-mode: vertical-rl;
  text-orientation: mixed;
  font-size: 0.6rem;
  font-weight: 600;
  letter-spacing: 0.2em;
  color: oklch(0.55 0.15 260 / 0.25);
  white-space: nowrap;
  font-family: 'Inter', -apple-system, sans-serif;
  pointer-events: none;
}

/* 各专业颜色 */
.major-hero-software .hero-index-num { color: oklch(0.55 0.21 265); }
.major-hero-software .hero-index-line { background: oklch(0.55 0.21 265 / 0.3); }
.major-hero-software .hero-theme { color: oklch(0.55 0.21 265); }
.major-hero-software .hero-theme-prefix { color: oklch(0.55 0.18 260); }
.major-hero-software .hero-label { color: oklch(0.50 0.14 260); }
.major-hero-software .hero-label-dash { background: oklch(0.50 0.14 260 / 0.35); }
.major-hero-software .geo-ring { border-color: oklch(0.55 0.21 265 / 0.22); }
.major-hero-software .geo-ring-inner { border-color: oklch(0.55 0.21 265 / 0.22); }
.major-hero-software .geo-core-dot { background: oklch(0.55 0.21 265 / 0.55); }
.major-hero-software .geo-core-pulse { background: oklch(0.55 0.21 265 / 0.3); }
.major-hero-software .geo-orbit-1 { background: oklch(0.55 0.21 265 / 0.35); }
.major-hero-software .geo-dot-1 { background: oklch(0.55 0.21 265 / 0.5); }
.major-hero-software .geo-cross span { background: oklch(0.55 0.21 265 / 0.45); }
.major-hero-software .geo-vert-text { color: oklch(0.55 0.16 260 / 0.3); }

.major-hero-electronic .hero-index-num { color: oklch(0.50 0.18 295); }
.major-hero-electronic .hero-index-line { background: oklch(0.50 0.18 295 / 0.3); }
.major-hero-electronic .hero-theme { color: oklch(0.50 0.18 295); }
.major-hero-electronic .hero-theme-prefix { color: oklch(0.50 0.16 290); }
.major-hero-electronic .hero-label { color: oklch(0.48 0.15 290); }
.major-hero-electronic .hero-label-dash { background: oklch(0.48 0.15 290 / 0.35); }
.major-hero-electronic .geo-ring { border-color: oklch(0.50 0.18 295 / 0.22); }
.major-hero-electronic .geo-ring-inner { border-color: oklch(0.50 0.18 295 / 0.22); }
.major-hero-electronic .geo-core-dot { background: oklch(0.50 0.18 295 / 0.55); }
.major-hero-electronic .geo-core-pulse { background: oklch(0.50 0.18 295 / 0.3); }
.major-hero-electronic .geo-orbit-1 { background: oklch(0.50 0.18 295 / 0.35); }
.major-hero-electronic .geo-dot-1 { background: oklch(0.50 0.18 295 / 0.5); }
.major-hero-electronic .geo-cross span { background: oklch(0.50 0.18 295 / 0.45); }
.major-hero-electronic .geo-vert-text { color: oklch(0.50 0.14 290 / 0.3); }

.major-hero-broadcast .hero-index-num { color: oklch(0.52 0.15 225); }
.major-hero-broadcast .hero-index-line { background: oklch(0.52 0.15 225 / 0.3); }
.major-hero-broadcast .hero-theme { color: oklch(0.52 0.15 225); }
.major-hero-broadcast .hero-theme-prefix { color: oklch(0.52 0.13 220); }
.major-hero-broadcast .hero-label { color: oklch(0.48 0.12 220); }
.major-hero-broadcast .hero-label-dash { background: oklch(0.48 0.12 220 / 0.35); }
.major-hero-broadcast .geo-ring { border-color: oklch(0.52 0.15 225 / 0.22); }
.major-hero-broadcast .geo-ring-inner { border-color: oklch(0.52 0.15 225 / 0.22); }
.major-hero-broadcast .geo-core-dot { background: oklch(0.52 0.15 225 / 0.55); }
.major-hero-broadcast .geo-core-pulse { background: oklch(0.52 0.15 225 / 0.3); }
.major-hero-broadcast .geo-orbit-1 { background: oklch(0.52 0.15 225 / 0.35); }
.major-hero-broadcast .geo-dot-1 { background: oklch(0.52 0.15 225 / 0.5); }
.major-hero-broadcast .geo-cross span { background: oklch(0.52 0.15 225 / 0.45); }
.major-hero-broadcast .geo-vert-text { color: oklch(0.52 0.12 220 / 0.3); }

/* 响应式 */
@media (max-width: 1024px) {
  .hero-body {
    gap: 1.5rem;
  }

  .hero-index-num {
    font-size: clamp(4rem, 8vw, 7rem);
  }

  .hero-right {
    min-height: 280px;
  }

  .geo-ring {
    width: 220px;
    height: 220px;
  }

  .geo-ring-inner {
    width: 140px;
    height: 140px;
  }
}

@media (max-width: 768px) {
  .major-hero {
    padding: 5rem 0 3rem;
  }

  .hero-breadcrumb {
    margin-bottom: 2rem;
    font-size: 0.72rem;
    gap: 0.4rem;
  }

  .hero-body {
    grid-template-columns: 1fr auto;
    gap: 0.75rem;
  }

  .hero-left {
    gap: 0.75rem;
  }

  .hero-index {
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
  }

  .hero-index-num {
    font-size: 2.5rem;
  }

  .hero-index-line {
    width: 2px;
    height: 2.5rem;
    margin-top: 0;
  }

  .hero-name-main {
    font-size: 1.5rem;
  }

  .hero-theme {
    font-size: 1.15rem;
  }

  .hero-footer {
    max-width: none;
  }

  .hero-right {
    min-height: auto;
    align-self: center;
    position: absolute;
    top: 4rem;
    right: 11rem;
  }

  .geo-ring {
    width: 200px;
    height: 200px;
  }

  .geo-ring-inner {
    width: 130px;
    height: 130px;
  }

  .geo-core-dot,
  .geo-core-pulse {
    width: 7px;
    height: 7px;
  }

  .geo-orbit-1 { width: 11px; height: 11px; top: 67px; left: 44px; }
  .geo-orbit-2 { width: 9px; height: 9px; top: -44px; left: -67px; }
  .geo-orbit-3 { width: 7px; height: 7px; top: -111px; left: 56px; }

  .geo-dot-1 { width: 11px; height: 11px; top: 89px; left: -111px; }
  .geo-dot-2 { width: 9px; height: 9px; top: 0px; left: 67px; }
  .geo-dot-3 { width: 7px; height: 7px; top: -67px; left: -111px; }

  .geo-cross span:first-child { width: 1px; height: 27px; left: 14px; top: 0; }
  .geo-cross span:last-child  { width: 27px; height: 1px; left: 0; top: 14px; }

  .geo-vert-text {
    top: -1.5rem;
    left: 10.2rem;
  }

  .mb-8.flex.flex-wrap {
    gap: 0.5rem;
  }

  button.px-6.py-2\\.5 {
    padding: 0.5rem 1rem;
    font-size: 0.8rem;
  }
}

@media (max-width: 640px) {
  .major-hero {
    padding: 4rem 0 2rem;
  }

  .hero-breadcrumb {
    font-size: 0.68rem;
    gap: 0.3rem;
    margin-bottom: 1.5rem;
  }

  .hero-index-num {
    font-size: 2.75rem;
  }

  .hero-name-main {
    font-size: 1.6rem;
  }

  .hero-theme {
    font-size: 1.25rem;
  }

  .hero-subtitle {
    font-size: 0.9rem;
  }

  .hero-desc {
    font-size: 0.82rem;
  }

  .hero-right {
    min-height: auto;
    align-self: center;
    position: absolute;
    top: 4rem;
    right: 10rem;
  }

  .geo-ring {
    width: 190px;
    height: 190px;
  }

  .geo-ring-inner {
    width: 120px;
    height: 120px;
  }

  .geo-core-dot,
  .geo-core-pulse {
    width: 7px;
    height: 7px;
  }

  .geo-orbit-1 { width: 11px; height: 11px; top: 63px; left: 42px; }
  .geo-orbit-2 { width: 8px; height: 8px; top: -42px; left: -63px; }
  .geo-orbit-3 { width: 6px; height: 6px; top: -106px; left: 53px; }

  .geo-dot-1 { width: 11px; height: 11px; top: 85px; left: -106px; }
  .geo-dot-2 { width: 8px; height: 8px; top: 0px; left: 63px; }
  .geo-dot-3 { width: 6px; height: 6px; top: -63px; left: -106px; }

  .geo-cross span:first-child { width: 1px; height: 25px; left: 13px; top: 0; }
  .geo-cross span:last-child  { width: 25px; height: 1px; left: 0; top: 13px; }

  .geo-vert-text {
    top: -1.5rem;
    left: 9.5rem;
  }

  .hero-theme-prefix {
    font-size: 1.2rem;
  }

  .grid.grid-cols-1 {
    gap: 1rem;
  }

  .mt-12.flex.justify-center button {
    padding: 0.4rem 1rem;
    font-size: 0.8rem;
  }

  .w-10.h-10 {
    width: 2.25rem;
    height: 2.25rem;
    font-size: 0.8rem;
  }
}

@media (max-width: 480px) {
  .major-hero {
    padding: 3.5rem 0 1.5rem;
  }

  .hero-breadcrumb {
    font-size: 0.64rem;
    gap: 0.25rem;
    margin-bottom: 1.25rem;
  }

  .hero-index-num {
    font-size: 2.25rem;
  }

  .hero-right {
    min-height: auto;
    align-self: center;
    position: absolute;
    top: 4rem;
    right: 9rem;
  }

  .geo-ring {
    width: 180px;
    height: 180px;
  }

  .geo-ring-inner {
    width: 100px;
    height: 100px;
  }

  .geo-core-dot,
  .geo-core-pulse {
    width: 6px;
    height: 6px;
  }

  .geo-orbit-1 { width: 10px; height: 10px; top: 60px; left: 40px;}
  .geo-orbit-2 { width: 8px; height: 8px; top: -40px; left: -60px;}
  .geo-orbit-3 { width: 6px; height: 6px; top: -100px; left: 50px;}

  .geo-dot-1 { width: 10px; height: 10px; top: 80px; left: -100px;}
  .geo-dot-2 { width: 8px; height: 8px; top: 0px; left: 60px;}
  .geo-dot-3 { width: 6px; height: 6px; top: -60px; left: -100px;}

  .geo-cross span:first-child { width: 1px; height: 24px; left: 12px; top: 0; }
  .geo-cross span:last-child  { width: 24px; height: 1px; left: 0; top: 12px; }

  .geo-vert-text {
    top: -1.5rem;
    left: 8.3rem;
  }

  .hero-name-main {
    font-size: 1.35rem;
  }

  .hero-theme {
    font-size: 1.1rem;
  }

  .hero-label {
    font-size: 0.6rem;
  }

  .hero-label-dash {
    width: 1.5rem;
  }

  button.px-6.py-2\\.5 {
    padding: 0.4rem 0.75rem;
    font-size: 0.72rem;
    border-radius: 9999px;
  }
}
</style>
