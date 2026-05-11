<template>
  <section id="ranking" class="ranking-section">
    <div class="container mx-auto px-4">
      <!-- 区域标题行：左侧标题，右侧时间切换 -->
      <div class="ranking-topbar">
        <div class="ranking-title-group">
          <span class="ranking-eyebrow">HOT RANKING</span>
          <h2 class="ranking-title">点赞热榜</h2>
        </div>
        <div class="ranking-tabs">
          <button
            v-for="tab in tabs"
            :key="tab.value"
            @click="selectedRange = tab.value"
            :class="['ranking-tab', { active: selectedRange === tab.value }]"
          >
            {{ tab.label }}
          </button>
        </div>
      </div>

      <!-- 排行列表 -->
      <div v-if="rankings.length > 0" class="ranking-items">
        <!-- 前三名：NO.X 渐变数字 + 专属卡片 -->
        <div
          v-for="(item, index) in topThree"
          :key="item.work_id"
          :class="['rank-card-hero', `rank-hero-${index + 1}`]"
        >
          <div class="rank-hero-num">
            <span :class="['rank-hero-digit', `rank-gradient-${index + 1}`]">NO.{{ index + 1 }}</span>
          </div>
          <div class="rank-hero-body">
            <NuxtLink :to="`/works/${item.work_id}`" class="rank-hero-poster">
              <video
                v-if="getWork(item.work_id)?.posterUrl"
                :src="getWork(item.work_id)?.posterUrl"
                preload="metadata"
                muted playsinline disablepictureinpicture
                class="rank-hero-video"
                @loadeddata="(e) => { const v = e.target as HTMLVideoElement; v.currentTime = 3; v.pause(); }"
              />
              <div class="rank-hero-play">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
              </div>
            </NuxtLink>
            <div class="rank-hero-info">
              <span class="rank-hero-major" :data-major="getWork(item.work_id)?.majorCode">
                {{ getWork(item.work_id)?.majorName }}
              </span>
              <h3 class="rank-hero-title">{{ getWork(item.work_id)?.title }}</h3>
            </div>
            <div class="rank-hero-meta">
              <div class="rank-hero-likes">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
                <span>{{ item.like_count }}</span>
              </div>
              <NuxtLink :to="`/works/${item.work_id}`" class="rank-hero-btn">
                查看详情
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/></svg>
              </NuxtLink>
            </div>
          </div>
        </div>

        <!-- 第4名及以后：标准行 -->
        <div
          v-for="(item, index) in restRankings"
          :key="item.work_id"
          class="rank-card"
        >
          <div class="rank-badge">
            {{ index + 4 }}
          </div>
          <NuxtLink :to="`/works/${item.work_id}`" class="rank-poster">
            <video
              v-if="getWork(item.work_id)?.posterUrl"
              :src="getWork(item.work_id)?.posterUrl"
              preload="metadata"
              muted playsinline disablepictureinpicture
              class="rank-poster-video"
              @loadeddata="(e) => { const v = e.target as HTMLVideoElement; v.currentTime = 3; v.pause(); }"
            />
            <div class="rank-poster-play">
              <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
            </div>
          </NuxtLink>
          <div class="rank-info">
            <h3 class="rank-title">{{ getWork(item.work_id)?.title }}</h3>
            <span class="rank-major-tag" :data-major="getWork(item.work_id)?.majorCode">
              {{ getWork(item.work_id)?.majorName }}
            </span>
          </div>
          <div class="rank-likes">
            <svg viewBox="0 0 24 24" fill="currentColor"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
            <span>{{ item.like_count }}</span>
          </div>
        </div>
      </div>

      <!-- 空状态 -->
      <div v-else class="ranking-empty">
        <p class="ranking-empty-text">暂无排行数据，快去给喜欢的作品点赞吧</p>
      </div>
    </div>
  </section>
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

const tabs = [
  { label: '总榜', value: 'all' },
  { label: '本周热榜', value: 'week' },
  { label: '今日热榜', value: 'today' }
]

const selectedRange = ref('all')
const rankings = ref<RankingItem[]>([])

const getWork = (workId: string) => {
  const ranking = rankings.value.find(r => r.work_id === workId)
  return ranking ? {
    id: ranking.work_id,
    title: ranking.title,
    majorCode: ranking.major_code,
    majorName: ranking.major_name,
    posterUrl: ranking.poster_url,
    introduction: ranking.introduction || '暂无简介'
  } : null
}

const topThree = computed(() => rankings.value.slice(0, 3))
const restRankings = computed(() => rankings.value.slice(3))

const loadRankings = async () => {
  try {
    const config = useRuntimeConfig()
    const visitorStore = useVisitorStore()

    const response = await $fetch(`${config.public.apiBase}/rankings/likes`, {
      params: {
        range: selectedRange.value,
        limit: 10
      },
      headers: {
        'X-Visitor-Id': visitorStore.getVisitorIdForRequest()
      }
    })

    if ((response as any).code === 0 && (response as any).data) {
      rankings.value = (response as any).data.items || []
    }
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
</script>

<style scoped>
/* ============================================
   点赞热榜
   ============================================ */
.ranking-section {
  padding: 5rem 0;
  background: linear-gradient(to bottom, #F3FBFB 0%, #F6F7F8 100%);
}

/* --- 顶栏：标题 + 时间切换 --- */
.ranking-topbar {
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  margin-bottom: 2rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.ranking-title-group {
  display: flex;
  flex-direction: column;
}

.ranking-eyebrow {
  font-size: 0.72rem;
  font-weight: 700;
  letter-spacing: 0.2em;
  color: #94a3b8;
  text-transform: uppercase;
  margin-bottom: 0.25rem;
}

.ranking-title {
  font-size: clamp(1.75rem, 3.5vw, 2.25rem);
  font-weight: 800;
  color: #0f172a;
  letter-spacing: -0.02em;
  line-height: 1.1;
}

/* --- 时间切换标签 --- */
.ranking-tabs {
  display: inline-flex;
  gap: 0.25rem;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 9999px;
  padding: 0.2rem;
}

.ranking-tab {
  padding: 0.45rem 1.25rem;
  font-size: 0.825rem;
  font-weight: 500;
  color: #64748b;
  background: transparent;
  border: none;
  border-radius: 9999px;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.ranking-tab:hover {
  color: #0f172a;
}

.ranking-tab.active {
  color: white;
  background: #1466ff;
  box-shadow: 0 2px 8px rgba(20, 102, 255, 0.25);
}

/* ============================================
   前三名 Hero 卡片 — 超大数字 / 杂志风格
   ============================================ */
.rank-card-hero {
  display: flex;
  align-items: stretch;
  gap: 0;
  background: white;
  border-radius: 1.25rem;
  border: 1px solid #f1f5f9;
  overflow: hidden;
  transition: box-shadow 0.3s ease, transform 0.3s ease;
}

.rank-card-hero:hover {
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
  transform: translateY(-2px);
}

/* #1 最突出 */
.rank-hero-1 {
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.04);
}
.rank-hero-1:hover {
  box-shadow: 0 12px 40px rgba(20, 102, 255, 0.14), 0 1px 3px rgba(0, 0, 0, 0.04);
}

/* 超大数字区 — 无背景，纯渐变文字 */
.rank-hero-num {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 1.5rem 0 2rem;
  flex-shrink: 0;
}

.rank-hero-digit {
  font-size: 2.25rem;
  font-weight: 900;
  line-height: 1;
  letter-spacing: -0.02em;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

.rank-hero-1 .rank-hero-digit { font-size: 2.5rem; }

/* NO.1 — 红色渐变 */
.rank-gradient-1 {
  background-image: linear-gradient(180deg, #ef4444 0%, #dc2626 40%, #991b1b 100%);
}

/* NO.2 — 蓝色渐变 */
.rank-gradient-2 {
  background-image: linear-gradient(180deg, #3b82f6 0%, #2563eb 40%, #1e40af 100%);
}

/* NO.3 — 金色渐变 */
.rank-gradient-3 {
  background-image: linear-gradient(180deg, #fbbf24 0%, #f59e0b 40%, #d97706 100%);
}

/* 内容区 */
.rank-hero-body {
  display: flex;
  align-items: center;
  gap: 1.25rem;
  flex: 1;
  min-width: 0;
  padding: 1.125rem 1.5rem 1.125rem 0;
}

/* 海报 */
.rank-hero-poster {
  position: relative;
  width: 160px;
  aspect-ratio: 16 / 9;
  border-radius: 0.625rem;
  overflow: hidden;
  background: #f1f5f9;
  flex-shrink: 0;
  display: block;
}

.rank-hero-1 .rank-hero-poster {
  width: 180px;
}

.rank-hero-video {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.rank-hero-play {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.18);
  opacity: 0;
  transition: opacity 0.25s ease;
}

.rank-hero-play svg {
  width: 2rem;
  height: 2rem;
  color: white;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
}

.rank-hero-poster:hover .rank-hero-play {
  opacity: 1;
}

/* 文字信息 */
.rank-hero-info {
  flex: 1;
  min-width: 0;
}

.rank-hero-major {
  display: inline-block;
  padding: 0.15rem 0.6rem;
  font-size: 0.7rem;
  font-weight: 600;
  border-radius: 9999px;
  letter-spacing: 0.03em;
  margin-bottom: 0.375rem;
}

.rank-hero-major[data-major="software"]  { color: #1466ff; background: #eef4ff; }
.rank-hero-major[data-major="electronic"]{ color: #0891b2; background: #ecfeff; }
.rank-hero-major[data-major="broadcast"] { color: #7c3aed; background: #f5f3ff; }

.rank-hero-title {
  font-size: 1.125rem;
  font-weight: 700;
  color: #0f172a;
  line-height: 1.3;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.rank-hero-1 .rank-hero-title {
  font-size: 1.25rem;
}

/* 点赞 + 按钮 */
.rank-hero-meta {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.625rem;
  flex-shrink: 0;
}

.rank-hero-likes {
  display: flex;
  align-items: center;
  gap: 0.2rem;
}

.rank-hero-likes svg {
  width: 1.25rem;
  height: 1.25rem;
  color: #ef4444;
}

.rank-hero-1 .rank-hero-likes svg {
  width: 1.375rem;
  height: 1.375rem;
}

.rank-hero-likes span {
  font-size: 1.25rem;
  font-weight: 800;
  color: #0f172a;
}

.rank-hero-1 .rank-hero-likes span {
  font-size: 1.375rem;
}

.rank-hero-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.3rem;
  padding: 0.45rem 1.2rem;
  font-size: 0.8rem;
  font-weight: 600;
  color: #1466ff;
  background: white;
  border: 1.5px solid #e2e8f0;
  border-radius: 9999px;
  text-decoration: none;
  transition: all 0.22s ease;
  white-space: nowrap;
}

.rank-hero-btn svg {
  width: 0.85rem;
  height: 0.85rem;
  transition: transform 0.22s ease;
}

.rank-hero-btn:hover {
  color: white;
  background: #1466ff;
  border-color: #1466ff;
  box-shadow: 0 4px 14px rgba(20, 102, 255, 0.25);
}

.rank-hero-btn:hover svg {
  transform: translateX(2px);
}

/* --- 排行列表容器 --- */
.ranking-items {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

/* --- 排名数字 --- */
/* --- 4名+ 标准行 --- */
.rank-card {
  display: grid;
  grid-template-columns: 3rem 100px 1fr auto auto;
  gap: 1rem;
  align-items: center;
  padding: 0.75rem 1.25rem;
  background: white;
  border-radius: 0.875rem;
  border: 1px solid #f1f5f9;
  transition: box-shadow 0.25s ease, transform 0.25s ease;
}

.rank-card:hover {
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.06);
  transform: translateY(-1px);
  border-color: #e2e8f0;
}

.rank-badge {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: 0.625rem;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.15rem;
  font-weight: 800;
  background: #f8fafc;
  color: #94a3b8;
  flex-shrink: 0;
}

/* --- 海报 --- */
.rank-poster {
  position: relative;
  width: 100px;
  aspect-ratio: 16 / 9;
  border-radius: 0.5rem;
  overflow: hidden;
  background: #f1f5f9;
  flex-shrink: 0;
  display: block;
}

.rank-poster-video {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.rank-poster-play {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.2);
  opacity: 0;
  transition: opacity 0.25s ease;
}

.rank-poster-play svg {
  width: 1.75rem;
  height: 1.75rem;
  color: white;
}

.rank-poster:hover .rank-poster-play {
  opacity: 1;
}

/* --- 信息 --- */
.rank-info {
  min-width: 0;
}

.rank-title {
  font-size: 0.925rem;
  font-weight: 600;
  color: #0f172a;
  line-height: 1.3;
  margin-bottom: 0.25rem;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.rank-major-tag {
  display: inline-block;
  padding: 0.12rem 0.55rem;
  font-size: 0.675rem;
  font-weight: 600;
  border-radius: 9999px;
  letter-spacing: 0.02em;
}

.rank-major-tag[data-major="software"]  { color: #1466ff; background: #eef4ff; }
.rank-major-tag[data-major="electronic"]{ color: #0891b2; background: #ecfeff; }
.rank-major-tag[data-major="broadcast"] { color: #7c3aed; background: #f5f3ff; }

/* --- 点赞数 --- */
.rank-likes {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  flex-shrink: 0;
}

.rank-likes svg {
  width: 1.1rem;
  height: 1.1rem;
  color: #ef4444;
}

.rank-likes span {
  font-size: 1.1rem;
  font-weight: 700;
  color: #0f172a;
}

/* --- 空状态 --- */
.ranking-empty {
  text-align: center;
  padding: 4rem 0;
}

.ranking-empty-text {
  color: #94a3b8;
  font-size: 0.95rem;
}

/* --- 响应式 --- */
@media (max-width: 1024px) {
  .rank-hero-num {
    padding: 0 1rem 0 1.25rem;
  }

  .rank-hero-digit {
    font-size: 1.75rem;
  }

  .rank-hero-1 .rank-hero-digit {
    font-size: 2rem;
  }

  .rank-hero-body {
    padding: 1rem 1.25rem 1rem 0;
    gap: 1rem;
  }

  .rank-hero-poster {
    width: 130px;
  }

  .rank-hero-1 .rank-hero-poster {
    width: 140px;
  }

  .rank-hero-title {
    font-size: 1rem;
  }

  .rank-hero-1 .rank-hero-title {
    font-size: 1.1rem;
  }
}

@media (max-width: 900px) {
  .rank-card {
    grid-template-columns: 2.5rem 80px 1fr auto;
    gap: 0.75rem;
    padding: 0.7rem 1rem;
  }

  .rank-hero-num {
    padding: 0 0.75rem 0 1rem;
  }

  .rank-hero-digit {
    font-size: 1.5rem;
  }

  .rank-hero-1 .rank-hero-digit {
    font-size: 1.65rem;
  }

  .rank-hero-body {
    padding: 0.875rem 1rem 0.875rem 0;
    gap: 0.75rem;
  }

  .rank-hero-poster {
    width: 100px;
  }

  .rank-hero-1 .rank-hero-poster {
    width: 110px;
  }

  .rank-hero-title {
    font-size: 0.9rem;
  }

  .rank-hero-1 .rank-hero-title {
    font-size: 0.95rem;
  }

  .rank-hero-meta {
    gap: 0.5rem;
  }

  .rank-hero-likes svg {
    width: 1.05rem;
    height: 1.05rem;
  }

  .rank-hero-likes span {
    font-size: 1.05rem;
  }

  .rank-hero-btn {
    padding: 0.4rem 1rem;
    font-size: 0.725rem;
  }
}

@media (max-width: 640px) {
  .ranking-topbar {
    flex-direction: column;
    align-items: flex-start;
  }

  .rank-card-hero {
    flex-direction: column;
    border-radius: 1rem;
  }

  .rank-hero-num {
    flex-direction: row;
    padding: 0.75rem 1rem 0;
  }

  .rank-hero-digit {
    font-size: 1.35rem;
  }

  .rank-hero-1 .rank-hero-digit {
    font-size: 1.5rem;
  }

  .rank-hero-body {
    flex-wrap: wrap;
    padding: 0.75rem 1rem 1rem;
    gap: 0.625rem;
  }

  .rank-hero-poster,
  .rank-hero-1 .rank-hero-poster {
    width: 100%;
    aspect-ratio: 16 / 9;
  }

  .rank-hero-info {
    flex: 1 1 60%;
    min-width: 0;
  }

  .rank-hero-meta {
    flex-direction: row;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    justify-content: flex-end;
  }

  .rank-card {
    grid-template-columns: 2.25rem 64px 1fr auto;
    gap: 0.5rem;
    padding: 0.625rem 0.75rem;
  }

  .rank-poster {
    width: 64px;
  }

  .rank-badge {
    width: 2rem;
    height: 2rem;
    font-size: 1rem;
    border-radius: 0.5rem;
  }

  .rank-title {
    font-size: 0.8rem;
  }

  .rank-likes svg {
    width: 0.9rem;
    height: 0.9rem;
  }

  .rank-likes span {
    font-size: 0.95rem;
  }
}
</style>
