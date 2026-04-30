<template>
  <div class="major-works-page min-h-screen bg-bg-page">
    <ExhibitionHeader />

    <!-- 专业主题横幅 -->
    <section class="major-hero relative overflow-hidden py-16 md:py-24">
      <!-- 动态背景 -->
      <div class="absolute inset-0 bg-gradient-to-br from-primary-blue/5 via-primary-cyan/5 to-transparent"></div>
      <div class="absolute inset-0 opacity-30">
        <div class="absolute top-0 left-1/4 w-96 h-96 bg-primary-cyan/20 rounded-full blur-3xl animate-pulse"></div>
        <div class="absolute bottom-0 right-1/4 w-96 h-96 bg-primary-blue/20 rounded-full blur-3xl animate-pulse" style="animation-delay: 1s;"></div>
      </div>

      <div class="container mx-auto px-4 relative z-10">
        <!-- 面包屑 -->
        <nav class="mb-8 text-sm">
          <ol class="flex items-center gap-2 text-text-secondary">
            <li><NuxtLink to="/" class="hover:text-primary-blue transition-colors">首页</NuxtLink></li>
            <li>/</li>
            <li><NuxtLink to="/majors" class="hover:text-primary-blue transition-colors">专业展区</NuxtLink></li>
            <li>/</li>
            <li class="text-text-main font-medium">{{ majorInfo?.name }}</li>
          </ol>
        </nav>

        <!-- 专业信息 -->
        <div class="max-w-4xl">
          <div class="inline-block px-4 py-1 bg-primary-blue/10 text-primary-blue rounded-full text-sm font-medium mb-4">
            {{ majorInfo?.code.toUpperCase() }}
          </div>
          <h1 class="text-4xl md:text-6xl font-bold mb-4 bg-gradient-to-r from-primary-blue to-primary-cyan bg-clip-text text-transparent">
            {{ majorInfo?.name }}
          </h1>
          <p class="text-2xl md:text-3xl font-bold text-text-main mb-4">
            {{ majorInfo?.theme }}
          </p>
          <p class="text-lg md:text-xl text-text-secondary mb-2">
            {{ majorInfo?.subtitle }}
          </p>
          <p class="text-base md:text-lg text-text-secondary leading-relaxed">
            {{ majorInfo?.description }}
          </p>
        </div>
      </div>
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
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const majorCode = computed(() => route.params.majorCode as string)

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

const majorInfo = computed(() => majorsConfig[majorCode.value as keyof typeof majorsConfig])
const filterTags = computed(() => majorInfo.value?.tags || [])

// 使用 API 加载作品数据
const { fetchWorks } = useWorks()
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
    allWorks.value = response.items.map(item => item.work)
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

useHead({
  title: `${majorInfo.value?.name} - 传媒技术学院2026届毕业设计展`
})
</script>

<style scoped>
.major-hero {
  position: relative;
}

@keyframes pulse {
  0%, 100% {
    opacity: 0.3;
    transform: scale(1);
  }
  50% {
    opacity: 0.5;
    transform: scale(1.1);
  }
}
</style>
