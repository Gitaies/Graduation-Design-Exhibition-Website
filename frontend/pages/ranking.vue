<template>
  <div class="ranking-page min-h-screen bg-gradient-to-br from-gray-50 via-blue-50 to-gray-50">
    <ExhibitionHeader />

    <div class="container mx-auto px-4 py-8 pt-24">
      <!-- 页面标题 -->
      <div class="mb-8">
        <h1 class="text-4xl md:text-5xl font-bold text-gray-800 mb-2">
          点赞热榜
          <span class="text-lg md:text-xl text-gray-500 font-normal ml-3">/ HOT RANKING</span>
        </h1>
      </div>

      <!-- 顶部导航标签 -->
      <div class="flex items-center gap-8 mb-8 border-b border-gray-200">
        <NuxtLink to="/" class="tab-link">首页</NuxtLink>
        <NuxtLink to="/#majors" class="tab-link">专业展区</NuxtLink>
        <NuxtLink to="/ranking" class="tab-link tab-link-active">点赞热榜</NuxtLink>
        <NuxtLink to="/#about" class="tab-link">关于毕展</NuxtLink>
      </div>

      <!-- 时间范围切换 -->
      <div class="flex gap-4 mb-8">
        <button
          v-for="tab in tabs"
          :key="tab.value"
          @click="selectedRange = tab.value"
          :class="[
            'time-tab',
            selectedRange === tab.value ? 'time-tab-active' : ''
          ]"
        >
          {{ tab.label }}
        </button>
      </div>

      <!-- 加载状态 -->
      <div v-if="loading" class="text-center py-20">
        <div class="inline-block w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin"></div>
        <p class="mt-4 text-gray-500">加载中...</p>
      </div>

      <!-- 排行榜内容 -->
      <div v-else-if="rankings.length > 0" class="space-y-4">
        <!-- 前三名大卡片 -->
        <div
          v-for="(item, index) in topThree"
          :key="item.work_id"
          :class="[
            'ranking-card ranking-card-top',
            index === 0 ? 'ranking-card-1' : index === 1 ? 'ranking-card-2' : 'ranking-card-3'
          ]"
        >
          <div class="ranking-number">
            <span :class="[
              'ranking-badge',
              index === 0 ? 'text-orange-500' : index === 1 ? 'text-gray-600' : 'text-orange-400'
            ]">
              NO.{{ index + 1 }}
            </span>
          </div>

          <div class="ranking-content">
            <div class="ranking-poster">
              <img
                v-if="getWork(item.work_id)?.posterUrl"
                :src="getWork(item.work_id)?.posterUrl || ''"
                :alt="getWork(item.work_id)?.title"
                loading="lazy"
                class="w-full h-full object-cover"
              />
              <div class="play-button">
                <svg class="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M8 5v14l11-7z" />
                </svg>
              </div>
            </div>

            <div class="ranking-info">
              <div class="flex items-center gap-2 mb-2">
                <h3 class="text-xl md:text-2xl font-bold text-gray-800">
                  {{ getWork(item.work_id)?.title }}
                </h3>
              </div>
              <div class="flex items-center gap-3 mb-3">
                <span class="major-tag">{{ getMajorName(getWork(item.work_id)?.majorCode) }}</span>
              </div>
              <p class="text-sm text-gray-600 line-clamp-2">
                {{ getWork(item.work_id)?.introduction }}
              </p>
            </div>
          </div>

          <div class="ranking-actions">
            <div class="like-count">
              <svg class="w-6 h-6 text-blue-500" fill="currentColor" viewBox="0 0 24 24">
                <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
              </svg>
              <span class="text-2xl font-bold text-gray-800">{{ item.like_count }}</span>
            </div>
            <NuxtLink
              :to="`/works/${item.work_id}`"
              class="view-button"
            >
              查看详情
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </NuxtLink>
          </div>
        </div>

        <!-- 第4名及以后列表 -->
        <div
          v-for="(item, index) in restRankings"
          :key="item.work_id"
          class="ranking-card ranking-card-small"
        >
          <div class="ranking-number-small">
            {{ index + 4 }}
          </div>

          <div class="ranking-poster-small">
            <img
              v-if="getWork(item.work_id)?.posterUrl"
              :src="getWork(item.work_id)?.posterUrl || ''"
              :alt="getWork(item.work_id)?.title"
              loading="lazy"
              class="w-full h-full object-cover"
            />
          </div>

          <div class="ranking-info-small">
            <h3 class="text-lg font-bold text-gray-800 mb-1">
              {{ getWork(item.work_id)?.title }}
            </h3>
            <span class="major-tag-small">{{ getMajorName(getWork(item.work_id)?.majorCode) }}</span>
          </div>

          <div class="like-count-small">
            <svg class="w-5 h-5 text-blue-500" fill="currentColor" viewBox="0 0 24 24">
              <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
            </svg>
            <span class="text-lg font-bold text-gray-800">{{ item.like_count }}</span>
          </div>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-else class="text-center py-20">
        <p class="text-xl text-gray-600 mb-2">暂无排行数据</p>
        <p class="text-gray-400">快去给喜欢的作品点赞吧</p>
      </div>
    </div>

    <ExhibitionFooter />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'

interface RankingItem {
  rank: number
  work_id: string
  title: string
  major_code: string
  major_name: string
  poster_url: string
  introduction?: string | null
  like_count: number
}

interface RankingResponse {
  code: number
  message: string
  data: {
    items: RankingItem[]
    range: string
    limit: number
  }
}

const tabs = [
  { label: '今日热榜', value: 'today' },
  { label: '本周热榜', value: 'week' },
  { label: '总榜', value: 'all' }
]

const selectedRange = ref('today')
const rankings = ref<RankingItem[]>([])
const loading = ref(true)

const getWork = (workId: string) => {
  const ranking = rankings.value.find(r => r.work_id === workId)
  return ranking ? {
    id: ranking.work_id,
    title: ranking.title,
    majorCode: ranking.major_code,
    posterUrl: ranking.poster_url,
    introduction: ranking.introduction
  } : null
}

const getMajorName = (majorCode: string | undefined) => {
  const majorMap: Record<string, string> = {
    'software': '软件工程',
    'electronic': '电子信息工程',
    'broadcast': '广播电视工程'
  }
  return majorCode ? majorMap[majorCode] || majorCode : ''
}

const topThree = computed(() => rankings.value.slice(0, 3))
const restRankings = computed(() => rankings.value.slice(3, 6))

const loadRankings = async () => {
  loading.value = true
  try {
    const config = useRuntimeConfig()
    const visitorStore = useVisitorStore()

    const response = await $fetch<RankingResponse>(`${config.public.apiBase}/rankings/likes`, {
      params: {
        range: selectedRange.value,
        limit: 10
      },
      headers: {
        'X-Visitor-Id': visitorStore.getVisitorIdForRequest()
      }
    })

    if (response.code === 0 && response.data) {
      rankings.value = response.data.items || []
    }
  } catch (error) {
    console.error('加载排行榜失败:', error)
  } finally {
    loading.value = false
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
.tab-link {
  padding: 1rem 0;
  font-size: 1.125rem;
  color: #6b7280;
  border-bottom: 3px solid transparent;
  transition: all 0.3s;
}

.tab-link:hover {
  color: #1f2937;
}

.tab-link-active {
  color: #1f2937;
  font-weight: 600;
  border-bottom-color: #3b82f6;
}

.time-tab {
  padding: 0.75rem 2rem;
  font-size: 1rem;
  font-weight: 500;
  color: #6b7280;
  background: white;
  border-radius: 0.5rem;
  transition: all 0.3s;
  border: 1px solid #e5e7eb;
}

.time-tab:hover {
  color: #1f2937;
  border-color: #3b82f6;
}

.time-tab-active {
  color: white;
  background: #3b82f6;
  border-color: #3b82f6;
}

.ranking-card {
  background: white;
  border-radius: 1rem;
  padding: 1.5rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  transition: all 0.3s;
}

.ranking-card:hover {
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  transform: translateY(-2px);
}

.ranking-card-top {
  display: grid;
  grid-template-columns: auto 1fr auto;
  gap: 1.5rem;
  align-items: center;
}

.ranking-number {
  display: flex;
  align-items: center;
  justify-content: center;
}

.ranking-badge {
  font-size: 2.5rem;
  font-weight: 900;
  font-family: 'Arial Black', sans-serif;
}

.ranking-content {
  display: flex;
  gap: 1.5rem;
  align-items: center;
  flex: 1;
  min-width: 0;
}

.ranking-poster {
  position: relative;
  width: 160px;
  height: 90px;
  border-radius: 0.75rem;
  overflow: hidden;
  flex-shrink: 0;
  background: #f3f4f6;
}

.play-button {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.3);
  opacity: 0;
  transition: opacity 0.3s;
}

.ranking-poster:hover .play-button {
  opacity: 1;
}

.ranking-info {
  flex: 1;
  min-width: 0;
}

.major-tag {
  display: inline-block;
  padding: 0.25rem 0.75rem;
  font-size: 0.875rem;
  color: #3b82f6;
  background: #eff6ff;
  border-radius: 0.375rem;
}

.ranking-actions {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 1rem;
}

.like-count {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.view-button {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.625rem 1.5rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: white;
  background: #3b82f6;
  border-radius: 0.5rem;
  transition: all 0.3s;
}

.view-button:hover {
  background: #2563eb;
  transform: translateX(2px);
}

.ranking-card-small {
  display: grid;
  grid-template-columns: auto auto 1fr auto;
  gap: 1rem;
  align-items: center;
  padding: 1rem 1.5rem;
}

.ranking-number-small {
  font-size: 1.5rem;
  font-weight: 700;
  color: #6b7280;
  width: 3rem;
  text-align: center;
}

.ranking-poster-small {
  width: 80px;
  height: 45px;
  border-radius: 0.5rem;
  overflow: hidden;
  background: #f3f4f6;
}

.ranking-info-small {
  flex: 1;
  min-width: 0;
}

.major-tag-small {
  display: inline-block;
  padding: 0.125rem 0.5rem;
  font-size: 0.75rem;
  color: #3b82f6;
  background: #eff6ff;
  border-radius: 0.25rem;
}

.like-count-small {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

@media (max-width: 1024px) {
  .ranking-card-top {
    gap: 1rem;
  }

  .ranking-badge {
    font-size: 2rem;
  }

  .ranking-content {
    gap: 1rem;
  }

  .ranking-poster {
    width: 140px;
    height: 80px;
  }
}

@media (max-width: 768px) {
  .ranking-page h1 {
    font-size: 2rem;
  }

  .ranking-page h1 span {
    font-size: 1rem;
  }

  .tab-link {
    font-size: 1rem;
    padding: 0.75rem 0;
  }

  .time-tab {
    padding: 0.6rem 1.25rem;
    font-size: 0.9rem;
  }

  .ranking-card-top {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .ranking-content {
    flex-direction: column;
    align-items: flex-start;
  }

  .ranking-poster {
    width: 100%;
    height: auto;
    aspect-ratio: 16/9;
    border-radius: 0.75rem;
  }

  .ranking-actions {
    width: 100%;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
  }

  .like-count {
    flex-direction: row;
  }

  .like-count span {
    font-size: 1.5rem;
  }

  .view-button {
    padding: 0.5rem 1.25rem;
    font-size: 0.85rem;
  }

  .ranking-card-small {
    grid-template-columns: auto 1fr auto;
    gap: 0.75rem;
  }

  .ranking-poster-small {
    display: none;
  }

  .ranking-number-small {
    width: 2rem;
    font-size: 1.25rem;
  }
}

@media (max-width: 640px) {
  .ranking-page .container {
    padding-top: 1.5rem;
  }

  .ranking-page h1 {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }

  .ranking-page h1 span {
    font-size: 0.85rem;
    margin-left: 0.5rem;
    display: inline;
  }

  .tab-link {
    font-size: 0.85rem;
    padding: 0.6rem 0;
  }

  .flex.items-center.gap-8.mb-8 {
    gap: 1rem;
    flex-wrap: wrap;
  }

  .flex.gap-4.mb-8 {
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .time-tab {
    padding: 0.5rem 1rem;
    font-size: 0.8rem;
    border-radius: 0.375rem;
  }

  .ranking-card {
    padding: 1rem;
    border-radius: 0.75rem;
  }

  .ranking-badge {
    font-size: 1.5rem;
  }

  .ranking-card-top h3 {
    font-size: 1.1rem;
  }

  .major-tag {
    font-size: 0.75rem;
    padding: 0.2rem 0.6rem;
  }

  .ranking-card-small {
    grid-template-columns: auto 1fr;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
  }

  .like-count-small {
    display: none;
  }

  .ranking-number-small {
    width: 1.75rem;
    font-size: 1.1rem;
  }

  .ranking-info-small h3 {
    font-size: 0.95rem;
  }

  .major-tag-small {
    font-size: 0.68rem;
  }
}

@media (max-width: 480px) {
  .ranking-page h1 {
    font-size: 1.25rem;
  }

  .time-tab {
    padding: 0.4rem 0.75rem;
    font-size: 0.72rem;
  }

  .ranking-card-top h3 {
    font-size: 1rem;
  }

  .ranking-info p {
    font-size: 0.78rem;
  }

  .ranking-info-small h3 {
    font-size: 0.85rem;
  }
}
</style>
