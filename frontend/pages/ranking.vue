<template>
  <div class="ranking-page min-h-screen bg-bg-page">
    <ExhibitionHeader />

    <!-- 页面标题 -->
    <section class="relative overflow-hidden py-16 md:py-20">
      <!-- 背景装饰 -->
      <div class="absolute inset-0 bg-gradient-to-br from-primary-blue/5 via-primary-cyan/5 to-transparent"></div>
      <div class="absolute inset-0 opacity-20">
        <div class="absolute top-1/4 left-1/3 w-64 h-64 bg-primary-cyan/30 rounded-full blur-3xl animate-pulse"></div>
        <div class="absolute bottom-1/4 right-1/3 w-64 h-64 bg-primary-blue/30 rounded-full blur-3xl animate-pulse" style="animation-delay: 1s;"></div>
      </div>

      <div class="container mx-auto px-4 relative z-10 text-center">
        <div class="inline-block px-4 py-1.5 bg-primary-blue/10 text-primary-blue rounded-full text-sm font-medium mb-4">
          HOT RANKING
        </div>
        <h1 class="text-4xl md:text-6xl font-bold mb-4 bg-gradient-to-r from-primary-blue to-primary-cyan bg-clip-text text-transparent">
          点赞热榜
        </h1>
        <p class="text-lg text-text-secondary max-w-2xl mx-auto">
          最受欢迎的毕业设计作品排行榜
        </p>
      </div>
    </section>

    <!-- 榜单内容 -->
    <section class="py-12">
      <div class="container mx-auto px-4">
        <!-- 榜单切换 -->
        <div class="flex justify-center gap-4 mb-12">
          <button
            v-for="tab in tabs"
            :key="tab.value"
            @click="selectedRange = tab.value"
            :class="[
              'px-6 py-3 rounded-xl font-medium transition-all duration-200',
              selectedRange === tab.value
                ? 'bg-primary-blue text-white shadow-lg shadow-primary-blue/30'
                : 'bg-white/80 backdrop-blur-sm text-text-secondary hover:bg-white hover:text-primary-blue hover:shadow-md'
            ]"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- 前三名大卡片 -->
        <div v-if="topThree.length > 0" class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
          <div
            v-for="(item, index) in topThree"
            :key="item.work_id"
            :class="[
              'relative overflow-hidden rounded-2xl p-6 transition-all duration-300 hover:-translate-y-2',
              index === 0 ? 'md:col-span-3 bg-gradient-to-br from-yellow-400 to-orange-500 text-white shadow-2xl shadow-yellow-500/30' :
              index === 1 ? 'bg-gradient-to-br from-gray-300 to-gray-400 text-white shadow-xl' :
              'bg-gradient-to-br from-orange-300 to-orange-400 text-white shadow-xl'
            ]"
          >
            <!-- 排名徽章 -->
            <div class="absolute top-4 right-4">
              <div :class="[
                'w-16 h-16 rounded-full flex items-center justify-center text-2xl font-bold',
                index === 0 ? 'bg-white/20 backdrop-blur-sm' : 'bg-white/30 backdrop-blur-sm'
              ]">
                {{ index + 1 }}
              </div>
            </div>

            <!-- 作品信息 -->
            <div class="relative z-10">
              <div class="text-sm opacity-90 mb-2">{{ getWork(item.work_id)?.majorName }}</div>
              <h3 class="text-2xl font-bold mb-3 pr-20">
                {{ getWork(item.work_id)?.title }}
              </h3>
              <p class="text-sm opacity-90 mb-4 line-clamp-2">
                {{ getWork(item.work_id)?.introduction }}
              </p>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                  </svg>
                  <span class="text-2xl font-bold">{{ item.like_count }}</span>
                </div>
                <NuxtLink
                  :to="`/works/${item.work_id}`"
                  class="px-4 py-2 bg-white/20 backdrop-blur-sm hover:bg-white/30 rounded-lg font-medium transition-colors"
                >
                  查看详情
                </NuxtLink>
              </div>
            </div>
          </div>
        </div>

        <!-- 第4名及以后列表 -->
        <div v-if="restRankings.length > 0" class="space-y-4">
          <NuxtLink
            v-for="(item, index) in restRankings"
            :key="item.work_id"
            :to="`/works/${item.work_id}`"
            class="block bg-white/80 backdrop-blur-sm rounded-xl p-5 hover:shadow-xl hover:-translate-y-1 transition-all duration-300 group"
          >
            <div class="flex items-center gap-4">
              <!-- 排名 -->
              <div class="w-12 h-12 flex-shrink-0 bg-gradient-to-br from-gray-100 to-gray-200 rounded-xl flex items-center justify-center text-xl font-bold text-text-main">
                {{ index + 4 }}
              </div>

              <!-- 作品信息 -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                  <span class="px-2 py-0.5 bg-primary-blue/10 text-primary-blue text-xs rounded">
                    {{ getWork(item.work_id)?.majorName }}
                  </span>
                </div>
                <h3 class="text-lg font-bold text-text-main mb-1 group-hover:text-primary-blue transition-colors truncate">
                  {{ getWork(item.work_id)?.title }}
                </h3>
                <p class="text-sm text-text-secondary line-clamp-1">
                  {{ getWork(item.work_id)?.introduction }}
                </p>
              </div>

              <!-- 点赞数 -->
              <div class="flex items-center gap-2 text-primary-blue">
                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                </svg>
                <span class="text-xl font-bold">{{ item.like_count }}</span>
              </div>

              <!-- 箭头 -->
              <svg class="w-6 h-6 text-text-light group-hover:text-primary-blue group-hover:translate-x-1 transition-all" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </div>
          </NuxtLink>
        </div>

        <!-- 空状态 -->
        <div v-if="rankings.length === 0" class="text-center py-20">
          <div class="text-6xl mb-4">📊</div>
          <p class="text-xl text-text-secondary mb-2">暂无排行数据</p>
          <p class="text-text-light">快去给喜欢的作品点赞吧</p>
        </div>
      </div>
    </section>

    <ExhibitionFooter />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface RankingItem {
  rank: number
  work_id: string
  like_count: number
}

const tabs = [
  { label: '总榜', value: 'all' },
  { label: '本周热榜', value: 'week' },
  { label: '今日热榜', value: 'today' }
]

const selectedRange = ref('all')
const rankings = ref<RankingItem[]>([])

// 加载作品数据
const { data: worksData } = await useFetch('/data/works.json')
const allWorks = computed(() => worksData.value?.works || [])

const getWork = (workId: string) => {
  return allWorks.value.find((w: any) => w.id === workId)
}

const topThree = computed(() => rankings.value.slice(0, 3))
const restRankings = computed(() => rankings.value.slice(3))

// 加载排行榜数据
const loadRankings = async () => {
  try {
    // TODO: 调用 API 获取排行榜
    await new Promise(resolve => setTimeout(resolve, 500))

    // 生成模拟排行榜
    const mockRankings: RankingItem[] = allWorks.value
      .map((work: any, index: number) => ({
        rank: index + 1,
        work_id: work.id,
        like_count: Math.floor(Math.random() * 1000) + 100
      }))
      .sort((a, b) => b.like_count - a.like_count)
      .slice(0, 10)

    rankings.value = mockRankings
  } catch (error) {
    console.error('加载排行榜失败:', error)
  }
}

watch(selectedRange, () => {
  loadRankings()
})

onMounted(() => {
  loadRankings()
})

useHead({
  title: '点赞热榜 - 传媒技术学院2026届毕业设计展'
})
</script>

<style scoped>
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

@keyframes pulse {
  0%, 100% {
    opacity: 0.2;
    transform: scale(1);
  }
  50% {
    opacity: 0.4;
    transform: scale(1.1);
  }
}

.animate-pulse {
  animation: pulse 3s ease-in-out infinite;
}
</style>
