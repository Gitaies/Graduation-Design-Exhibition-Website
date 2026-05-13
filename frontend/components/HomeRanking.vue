<template>
  <section id="ranking" class="ranking-section">
    <div class="container mx-auto px-4 md:px-8 lg:px-12">
      <!-- 标题区 -->
      <div class="ranking-header">
        <canvas ref="waveCanvas" class="ranking-wave-canvas"></canvas>
        <span class="ranking-eyebrow">HOT RANKING</span>
        <h2 class="ranking-title">
          <span class="title-dianzan">点赞</span>
          <span class="title-dot">·</span>
          <span class="title-rebang">热榜</span>
        </h2>
        <div class="ranking-title-decor">
          <svg class="title-ornament" viewBox="0 0 600 24" fill="none">
            <circle cx="5" cy="12" r="2.5" fill="currentColor" opacity="0.65"/>
            <circle cx="40" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="75" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="110" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="145" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="180" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="215" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="250" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="285" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="320" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="355" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="390" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="425" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="460" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="495" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="530" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="565" cy="12" r="6" fill="currentColor" opacity="0.8"/>
          </svg>
          <svg class="title-trophy" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 318.47 190.21">
            <defs>
              <clipPath id="trophy-clip">
                <path d="M0 0 L318.48 0 L318.48 190.21 L0 190.21 Z" fill="none"/>
              </clipPath>
            </defs>
            <g clip-path="url(#trophy-clip)">
              <path d="M254.45,93.8c1.45-1.7,2.9-3.54,4.25-5.35H59.78c1.34,1.81,2.79,3.65,4.28,5.35Z" fill="none" stroke="currentColor" stroke-width="3" stroke-miterlimit="10"/>
              <path d="M154.62,9.4H156c-1.25,2.09-2.79,9.71.12,16.11h0a.63.63,0,0,0-.63.63.64.64,0,0,0,.63.64h6.15a.64.64,0,0,0,.64-.64.64.64,0,0,0-.64-.63h0c2.91-6.4,1.36-14,.11-16.11h1.42a.82.82,0,1,0,0-1.63h-.09V7.59a2.54,2.54,0,0,0-1.93-2.46,1.79,1.79,0,0,0,.57-1.31V3.24a.64.64,0,0,0,.54-.63.64.64,0,0,0-.64-.63h-.63V1.87A1.38,1.38,0,0,0,160.4.5h-2.32a1.38,1.38,0,0,0-1.29,1.37V2h-.63a.63.63,0,0,0-.63.63.64.64,0,0,0,.53.63v.58a1.76,1.76,0,0,0,.58,1.31,2.54,2.54,0,0,0-1.94,2.46v.18h-.09a.82.82,0,0,0,0,1.63Z" fill="none" stroke="currentColor" stroke-width="3" stroke-miterlimit="10"/>
              <path d="M95.17,115.83H89a37.84,37.84,0,0,0,5.42-11.65s-13.89,8.37-24.54,1.65S47.66,78.51,47.66,78.51s-11.6,20.6-22.25,27.32S.87,104.18.87,104.18s5.36,17,14.17,17H303.43c8.81,0,14.17-17,14.17-17s-13.9,8.36-24.54,1.65-22.24-27.32-22.24-27.32-11.6,20.6-22.25,27.32S224,104.18,224,104.18a41.11,41.11,0,0,0,5.64,11.65h-6.18" fill="none" stroke="currentColor" stroke-width="3" stroke-miterlimit="10"/>
              <path d="M95.88,76.39l18.69-9.08h89.34l18.68,9.08c14.35-2.25,19.52-18.76,19.52-18.76-9.91,10.43-32.11,7-28.36-3.37,3.27-8.48,1.67-14,1.67-14a20,20,0,0,1-16.76,10.54,17.23,17.23,0,0,1-10.15-3.34V37.6l2.57-1.82V26.69h-8V31H164.16V28.84h-10V31H135.27V26.69H127.4v9.09L130,37.6v9.84a17.24,17.24,0,0,1-10.16,3.34,20,20,0,0,1-16.75-10.54s-1.6,5.54,1.66,14c3.75,10.35-18.45,13.8-28.35,3.37C76.37,57.63,81.53,74.14,95.88,76.39Z" fill="none" stroke="currentColor" stroke-width="2.5" stroke-miterlimit="10"/>
              <!-- WHCM 文字 — 左右柱中间空缺处 -->
              <text x="159" y="170" text-anchor="middle" font-family="'Inter','Geist',sans-serif" font-size="48" font-weight="600" fill="currentColor" letter-spacing="3" opacity="0.75">WHCM</text>
              <path d="M80.18,126.88L15.32,126.88L15.32,189.71L80.18,189.71" fill="none" stroke="currentColor" stroke-width="3" stroke-miterlimit="10"/>
              <path d="M238.3,189.71L303.15,189.71L303.15,126.88L238.3,126.88" fill="none" stroke="currentColor" stroke-width="3" stroke-miterlimit="10"/>
            </g>
          </svg>
          <svg class="title-ornament" viewBox="0 0 600 24" fill="none">
            <circle cx="595" cy="12" r="2.5" fill="currentColor" opacity="0.65"/>
            <circle cx="560" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="525" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="490" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="455" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="420" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="385" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="350" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="315" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="280" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="245" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="210" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="175" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="140" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="105" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="70" cy="12" r="4" fill="currentColor" opacity="0.65"/>
            <circle cx="35" cy="12" r="6" fill="currentColor" opacity="0.8"/>
          </svg>
        </div>
      </div>

      <!-- 排行列表 -->
      <div v-if="rankings.length > 0" class="ranking-content">
        <!-- 标签栏：右对齐，对齐卡片顶部 -->
        <div class="ranking-tabs-row">
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

        <!-- 前三名：错落颁奖台布局 -->
        <div ref="podiumRef" class="podium">
          <!-- 第2名 - 左侧（蓝色，中等高度） -->
          <NuxtLink
            v-if="topThree[1]"
            :to="`/works/${topThree[1].work_id}`"
            class="podium-card podium-card--second"
          >
            <div class="podium-poster">
              <video
                v-if="getWork(topThree[1].work_id)?.posterUrl"
                :src="getWork(topThree[1].work_id)?.posterUrl"
                preload="metadata"
                muted playsinline disablepictureinpicture
                class="podium-video"
                @loadeddata="(e) => { const v = e.target as HTMLVideoElement; v.currentTime = 5; v.pause(); }"
              />
              <div class="podium-rank-strip">
                <span class="podium-rank-num">02</span>
              </div>
              <div class="podium-play">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
              </div>
            </div>
            <div class="podium-info">
              <span class="podium-major" :data-major="getWork(topThree[1].work_id)?.majorCode">
                {{ getWork(topThree[1].work_id)?.majorName }}
              </span>
              <h3 class="podium-title">{{ getWork(topThree[1].work_id)?.title }}</h3>
              <div class="podium-likes">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
                <span>{{ topThree[1].like_count }}</span>
              </div>
            </div>
          </NuxtLink>

          <!-- 第1名 - 中央最突出（红色，最高） -->
          <NuxtLink
            v-if="topThree[0]"
            :to="`/works/${topThree[0].work_id}`"
            class="podium-card podium-card--first"
          >
            <div class="podium-poster">
              <video
                v-if="getWork(topThree[0].work_id)?.posterUrl"
                :src="getWork(topThree[0].work_id)?.posterUrl"
                preload="metadata"
                muted playsinline disablepictureinpicture
                class="podium-video"
                @loadeddata="(e) => { const v = e.target as HTMLVideoElement; v.currentTime = 5; v.pause(); }"
              />
              <div class="podium-rank-strip">
                <span class="podium-rank-num">01</span>
              </div>
              <div class="podium-play">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
              </div>
            </div>
            <div class="podium-info">
              <span class="podium-major" :data-major="getWork(topThree[0].work_id)?.majorCode">
                {{ getWork(topThree[0].work_id)?.majorName }}
              </span>
              <h3 class="podium-title">{{ getWork(topThree[0].work_id)?.title }}</h3>
              <div class="podium-likes">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
                <span>{{ topThree[0].like_count }}</span>
              </div>
            </div>
          </NuxtLink>

          <!-- 第3名 - 右侧（金色，最低） -->
          <NuxtLink
            v-if="topThree[2]"
            :to="`/works/${topThree[2].work_id}`"
            class="podium-card podium-card--third"
          >
            <div class="podium-poster">
              <video
                v-if="getWork(topThree[2].work_id)?.posterUrl"
                :src="getWork(topThree[2].work_id)?.posterUrl"
                preload="metadata"
                muted playsinline disablepictureinpicture
                class="podium-video"
                @loadeddata="(e) => { const v = e.target as HTMLVideoElement; v.currentTime = 5; v.pause(); }"
              />
              <div class="podium-rank-strip">
                <span class="podium-rank-num">03</span>
              </div>
              <div class="podium-play">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
              </div>
            </div>
            <div class="podium-info">
              <span class="podium-major" :data-major="getWork(topThree[2].work_id)?.majorCode">
                {{ getWork(topThree[2].work_id)?.majorName }}
              </span>
              <h3 class="podium-title">{{ getWork(topThree[2].work_id)?.title }}</h3>
              <div class="podium-likes">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
                <span>{{ topThree[2].like_count }}</span>
              </div>
            </div>
          </NuxtLink>
        </div>

        <!-- 分隔线 -->
        <div class="ranking-divider">
          <span class="ranking-divider-label">更多优秀作品</span>
        </div>

        <!-- 第4名及以后：简洁列表 -->
        <div ref="restListRef" class="rest-list">
          <NuxtLink
            v-for="(item, index) in restRankings"
            :key="item.work_id"
            :to="`/works/${item.work_id}`"
            class="rest-card"
          >
            <span class="rest-rank-num">{{ index + 4 }}</span>
            <div class="rest-poster">
              <video
                v-if="getWork(item.work_id)?.posterUrl"
                :src="getWork(item.work_id)?.posterUrl"
                preload="metadata"
                muted playsinline disablepictureinpicture
                class="rest-poster-video"
                @loadeddata="(e) => { const v = e.target as HTMLVideoElement; v.currentTime = 5; v.pause(); }"
              />
              <div class="rest-poster-play">
                <svg viewBox="0 0 24 24" fill="currentColor"><path d="M8 5v14l11-7z"/></svg>
              </div>
            </div>
            <div class="rest-info">
              <h3 class="rest-title">{{ getWork(item.work_id)?.title }}</h3>
              <div class="rest-meta">
                <span class="rest-major" :data-major="getWork(item.work_id)?.majorCode">
                  {{ getWork(item.work_id)?.majorName }}
                </span>
                <span class="rest-likes">
                  <svg viewBox="0 0 24 24" fill="currentColor"><path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/></svg>
                  {{ item.like_count }}
                </span>
              </div>
            </div>
            <div class="rest-arrow">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M5 12h14M12 5l7 7-7 7"/>
              </svg>
            </div>
          </NuxtLink>
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
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'
import { gsap } from 'gsap'
import { useWaveBackground } from '~/composables/useWaveBackground'

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

const waveCanvas = ref<HTMLCanvasElement | null>(null)
let waveCleanup: (() => void) | null = null

const selectedRange = ref('all')
const rankings = ref<RankingItem[]>([])
const restListRef = ref<HTMLElement | null>(null)
const podiumRef = ref<HTMLElement | null>(null)

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

function animateEntrance() {
  nextTick(() => {
    // Podium cards stagger
    if (podiumRef.value) {
      const cards = podiumRef.value.querySelectorAll('.podium-card')
      gsap.fromTo(cards,
        { opacity: 0, y: 40 },
        {
          opacity: 1, y: 0,
          duration: 0.7,
          stagger: 0.12,
          ease: 'power3.out',
          clearProps: 'transform'
        }
      )
    }
    // Rest cards stagger
    if (restListRef.value) {
      const cards = restListRef.value.querySelectorAll('.rest-card')
      gsap.fromTo(cards,
        { opacity: 0, x: -20 },
        {
          opacity: 1, x: 0,
          duration: 0.5,
          stagger: 0.08,
          ease: 'power3.out',
          delay: 0.3,
          clearProps: 'transform'
        }
      )
    }
  })
}

watch(rankings, () => {
  if (rankings.value.length > 0) {
    animateEntrance()
  }
})

onMounted(() => {
  loadRankings()

  // 波浪动画背景
  if (waveCanvas.value) {
    const { destroy } = useWaveBackground({
      el: waveCanvas.value,
      speed: 0.5,
      height: 280,
      waves: [
        { lineWidth: 2, amplitude: 120, wavelength: 130 },
        { lineWidth: 1.5, amplitude: 100, wavelength: 135 },
        { lineWidth: 1, amplitude: 80, wavelength: 140 },
        { lineWidth: 1, amplitude: 60, wavelength: 145 },
        { lineWidth: 1, amplitude: 45, wavelength: 150 },
        { lineWidth: 0.8, amplitude: 30, wavelength: 155 },
        { lineWidth: 0.8, amplitude: 20, wavelength: 160 },
        { lineWidth: 0.6, amplitude: 12, wavelength: 165 },
      ]
    })
    waveCleanup = destroy
  }
})

onUnmounted(() => {
  if (waveCleanup) waveCleanup()
})
</script>

<style scoped>
/* ============================================
   区域容器
   ============================================ */
.ranking-section {
  padding: 5rem 0;
  background: linear-gradient(to bottom, #F3FBFB 0%, #F6F7F8 100%);
}

/* ============================================
   标题区：居中独立一行 + 波浪动画背景
   ============================================ */
.ranking-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-bottom: 1rem;
  position: relative;
  padding: 2.5rem 0 3rem;
  overflow: hidden;
}

.ranking-wave-canvas {
  position: absolute;
  top: 0;
  left: 50%;
  transform: translateX(-50%);
  width: 150vw;
  height: 100%;
  pointer-events: none;
  z-index: 0;
}

.ranking-header > *:not(.ranking-wave-canvas) {
  position: relative;
  z-index: 1;
}

.ranking-eyebrow {
  font-size: 0.72rem;
  font-weight: 700;
  letter-spacing: 0.2em;
  color: #94a3b8;
  text-transform: uppercase;
  margin-bottom: 0.25rem;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
}

.ranking-title {
  font-size: clamp(2rem, 4.5vw, 3rem);
  color: #0f172a;
  letter-spacing: 0.02em;
  line-height: 1.1;
  white-space: nowrap;
  margin-bottom: 0.35rem;
}

/* 点赞 — 轻字重 */
.title-dianzan {
  font-weight: 300;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
}

/* 间隔点 */
.title-dot {
  font-weight: 500;
  /* color: #1466ff; */
  margin: 0 0.15em;
}

/* 热榜 — 粗字重 */
.title-rebang {
  font-weight: 300;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
  letter-spacing: 0.04em;
}

/* 装饰行 — 奖杯+点点，标题下方独立一行 */
.ranking-title-decor {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
}

.title-ornament {
  width: clamp(6rem, 14vw, 10rem);
  height: 1.25rem;
  color: #64748b;
  flex-shrink: 0;
}

.title-trophy {
  width: 4.2rem;
  height: auto;
  color: #1466ff;
  flex-shrink: 0;
}

/* 标签栏 — 右对齐，与卡片顶部齐平 */
.ranking-tabs-row {
  display: flex;
  justify-content: flex-end;
  margin-bottom: -0.5rem;
  position: relative;
  z-index: 3;
}

/* 时间切换标签 */
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
}

/* ============================================
   前三名：错落颁奖台
   ============================================ */
.podium {
  display: flex;
  align-items: flex-end;
  justify-content: center;
  gap: 3.5rem;
  margin-bottom: 2.5rem;
}

/* 单张卡片 */
.podium-card {
  display: flex;
  flex-direction: column;
  background: white;
  border-radius: 0.875rem;
  overflow: hidden;
  text-decoration: none;
  transition: box-shadow 0.35s ease, transform 0.35s ease;
  flex: 1;
  max-width: 300px;
  border: 1px solid oklch(0.92 0.005 255);
}

.podium-card:hover {
  box-shadow: 0 12px 40px oklch(0.6 0.015 255 / 0.13);
  transform: translateY(-3px);
}

/* ---- 错落高度 ---- */
.podium-card--second {
  order: 1;
  margin-bottom: 2rem;
}

.podium-card--first {
  order: 2;
  max-width: 350px;
  margin-bottom: 4.5rem;
  box-shadow: 0 8px 40px rgba(0,0,0,0.08);
  border-color: #e2e8f0;
  z-index: 2;
}

.podium-card--first:hover {
  box-shadow: 0 20px 56px rgba(0,0,0,0.12);
}

.podium-card--third {
  order: 3;
  margin-bottom: 0rem;
}

/* ---- 海报 ---- */
.podium-poster {
  position: relative;
  width: 100%;
  aspect-ratio: 16 / 9;
  background: oklch(0.96 0.003 255);
  overflow: hidden;
}

.podium-card--first .podium-poster {
  aspect-ratio: 16 / 9;
}

.podium-video {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

/* 排名数字条 — 统一暗色渐变背景 */
.podium-rank-strip {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  padding: 1.25rem 1rem 0.75rem;
  background: linear-gradient(to top, rgba(15, 23, 42, 0.65) 0%, rgba(15, 23, 42, 0) 100%);
  pointer-events: none;
}

/* 数字 — 纯色着色 */
.podium-rank-num {
  font-size: 2rem;
  font-weight: 900;
  line-height: 1;
  letter-spacing: -0.03em;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
}

/* 第1名 — 红色 */
.podium-card--first .podium-rank-num {
  font-size: 2.75rem;
  color: #dc2626;
  filter: drop-shadow(0 2px 6px rgba(220, 38, 38, 0.25));
}

/* 第2名 — 蓝色 */
.podium-card--second .podium-rank-num {
  color: #2563eb;
  filter: drop-shadow(0 2px 6px rgba(37, 99, 235, 0.25));
}

/* 第3名 — 琥珀色 */
.podium-card--third .podium-rank-num {
  color: #d97706;
  filter: drop-shadow(0 2px 6px rgba(217, 119, 6, 0.25));
}

/* 播放图标 */
.podium-play {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.12);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.podium-play svg {
  width: 2.5rem;
  height: 2.5rem;
  color: white;
  filter: drop-shadow(0 2px 6px rgba(0, 0, 0, 0.3));
}

.podium-card--first .podium-play svg {
  width: 3rem;
  height: 3rem;
}

.podium-poster:hover .podium-play {
  opacity: 1;
}

/* ---- 信息区 ---- */
.podium-info {
  padding: 0.9rem 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
  flex: 1;
}

.podium-card--first .podium-info {
  padding: 1.1rem 1.2rem;
  gap: 0.5rem;
}

.podium-major {
  display: inline-block;
  align-self: flex-start;
  padding: 0.12rem 0.55rem;
  font-size: 0.65rem;
  font-weight: 600;
  border-radius: 9999px;
  letter-spacing: 0.03em;
}

.podium-card--first .podium-major {
  font-size: 0.7rem;
}

.podium-major[data-major="software"]   { color: #1466ff; background: #eef4ff; }
.podium-major[data-major="electronic"] { color: #0891b2; background: #ecfeff; }
.podium-major[data-major="broadcast"]  { color: #7c3aed; background: #f5f3ff; }

.podium-title {
  font-size: 0.95rem;
  font-weight: 700;
  color: #0f172a;
  line-height: 1.3;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.podium-card--first .podium-title {
  font-size: 1.1rem;
}

.podium-likes {
  display: inline-flex;
  align-items: center;
  gap: 0.35rem;
  margin-top: auto;
  padding: 0.3rem 0.75rem 0.3rem 0.6rem;
  background: oklch(0.97 0.008 15);
  border-radius: 9999px;
  align-self: flex-start;
}

.podium-likes svg {
  width: 0.95rem;
  height: 0.95rem;
  color: #ef4444;
  filter: drop-shadow(0 1px 2px rgba(239, 68, 68, 0.3));
  flex-shrink: 0;
}

.podium-card--first .podium-likes svg {
  width: 1.1rem;
  height: 1.1rem;
}

.podium-likes span {
  font-size: 0.9rem;
  font-weight: 700;
  color: oklch(0.25 0.01 15);
  line-height: 1;
}

.podium-card--first .podium-likes span {
  font-size: 1.05rem;
}

/* ============================================
   分隔线
   ============================================ */
.ranking-divider {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 1rem;
}

.ranking-divider::before,
.ranking-divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: linear-gradient(to right, transparent, #e2e8f0, transparent);
}

.ranking-divider::before {
  background: linear-gradient(to right, transparent, #e2e8f0);
}

.ranking-divider::after {
  background: linear-gradient(to left, transparent, #e2e8f0);
}

.ranking-divider-label {
  font-size: 0.75rem;
  font-weight: 500;
  color: #94a3b8;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  white-space: nowrap;
}

/* ============================================
   第4名+：简洁列表行
   ============================================ */
.rest-list {
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
}

.rest-card {
  display: grid;
  grid-template-columns: 2.5rem 72px 1fr auto;
  gap: 1rem;
  align-items: center;
  padding: 0.75rem 1.25rem;
  background: white;
  border-radius: 0.875rem;
  border: 1px solid #f1f5f9;
  text-decoration: none;
  transition: box-shadow 0.3s cubic-bezier(0.22, 1, 0.36, 1), transform 0.3s cubic-bezier(0.22, 1, 0.36, 1), border-color 0.3s ease;
}

.rest-card:hover {
  box-shadow: 0 8px 30px rgba(20, 102, 255, 0.08);
  transform: translateY(-2px);
  border-color: #dbeafe;
}

.rest-card:hover .rest-arrow {
  opacity: 1;
  transform: translateX(0);
}

.rest-card:hover .rest-rank-num {
  background: #eff6ff;
  color: #1466ff;
}

.rest-card:hover .rest-poster::after {
  opacity: 1;
}

/* 排名数字 */
.rest-rank-num {
  width: 2.25rem;
  height: 2.25rem;
  border-radius: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.9rem;
  font-weight: 700;
  background: #f8fafc;
  color: #94a3b8;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
  flex-shrink: 0;
  transition: background 0.3s ease, color 0.3s ease, box-shadow 0.3s ease;
}

/* 第4名 — 微妙铜色调 */
.rest-card:nth-child(1) .rest-rank-num {
  background: oklch(0.97 0.012 75);
  color: oklch(0.55 0.08 70);
}

/* 第5名 — 微妙钢蓝 */
.rest-card:nth-child(2) .rest-rank-num {
  background: oklch(0.97 0.01 240);
  color: oklch(0.5 0.06 250);
}

/* 第6名 — 微妙紫调 */
.rest-card:nth-child(3) .rest-rank-num {
  background: oklch(0.97 0.012 290);
  color: oklch(0.5 0.07 285);
}

/* 第7名及以后 — 默认灰调 */
.rest-card:nth-child(n+4) .rest-rank-num {
  background: #f8fafc;
  color: #94a3b8;
}

.rest-card:hover .rest-rank-num {
  background: #eff6ff;
  color: #1466ff;
  box-shadow: 0 0 0 3px oklch(0.88 0.06 255 / 0.4);
}

/* 缩略海报 */
.rest-poster {
  position: relative;
  width: 72px;
  aspect-ratio: 16 / 9;
  border-radius: 0.5rem;
  overflow: hidden;
  background: #f1f5f9;
  flex-shrink: 0;
}

.rest-poster::after {
  content: '';
  position: absolute;
  inset: 0;
  border-radius: 0.5rem;
  border: 2px solid #1466ff;
  opacity: 0;
  transition: opacity 0.3s ease;
  pointer-events: none;
  z-index: 2;
}

.rest-poster-video {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.rest-poster-play {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.2);
  opacity: 0;
  transition: opacity 0.25s ease;
}

.rest-poster-play svg {
  width: 1.25rem;
  height: 1.25rem;
  color: white;
}

.rest-poster:hover .rest-poster-play {
  opacity: 1;
}

/* 右侧箭头 */
.rest-arrow {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 2.25rem;
  height: 2.25rem;
  border-radius: 9999px;
  background: oklch(0.96 0.01 255);
  color: #94a3b8;
  opacity: 0;
  transform: translateX(-6px);
  transition: opacity 0.3s cubic-bezier(0.22, 1, 0.36, 1), transform 0.3s cubic-bezier(0.22, 1, 0.36, 1), background 0.25s ease, color 0.25s ease;
}

.rest-arrow svg {
  width: 1rem;
  height: 1rem;
}

.rest-card:hover .rest-arrow {
  background: #1466ff;
  color: white;
}

/* 信息区 */
.rest-info {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
}

.rest-title {
  font-size: 0.9rem;
  font-weight: 600;
  color: #0f172a;
  line-height: 1.3;
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.rest-meta {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.rest-major {
  display: inline-block;
  padding: 0.1rem 0.5rem;
  font-size: 0.65rem;
  font-weight: 600;
  border-radius: 9999px;
  letter-spacing: 0.02em;
}

.rest-major[data-major="software"]   { color: #1466ff; background: #eef4ff; }
.rest-major[data-major="electronic"] { color: #0891b2; background: #ecfeff; }
.rest-major[data-major="broadcast"]  { color: #7c3aed; background: #f5f3ff; }

.rest-likes {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  font-size: 0.8rem;
  font-weight: 700;
  color: oklch(0.4 0.01 15);
  padding: 0.2rem 0.55rem;
  background: oklch(0.97 0.006 15);
  border-radius: 9999px;
}

.rest-likes svg {
  width: 0.8rem;
  height: 0.8rem;
  color: #ef4444;
  filter: drop-shadow(0 1px 2px rgba(239, 68, 68, 0.25));
}

/* ============================================
   空状态
   ============================================ */
.ranking-empty {
  text-align: center;
  padding: 4rem 0;
}

.ranking-empty-text {
  color: #94a3b8;
  font-size: 0.95rem;
}

/* ============================================
   响应式
   ============================================ */
@media (max-width: 1024px) {
  .podium {
    gap: 2rem;
  }

  .podium-card--first {
    max-width: 320px;
  }

  .ranking-title-decor .title-ornament {
    width: clamp(3rem, 8vw, 6rem);
  }
}

@media (max-width: 900px) {
  .ranking-section {
    padding: 3.5rem 0;
  }

  .ranking-header {
    padding: 1.5rem 0 2rem;
  }

  .podium {
    gap: 1.25rem;
  }

  .podium-card--first {
    max-width: none;
  }

  .podium-card--second {
    margin-bottom: 1.25rem;
  }
  .podium-card--first {
    margin-bottom: 3rem;
  }
  .podium-card--third {
    margin-bottom: 0;
  }

  .podium-rank-num {
    font-size: 1.5rem;
  }
  .podium-card--first .podium-rank-num {
    font-size: 2rem;
  }

  .podium-title {
    font-size: 0.85rem;
  }
  .podium-card--first .podium-title {
    font-size: 0.95rem;
  }

  .ranking-title-decor .title-ornament {
    display: none;
  }

  .ranking-tabs-row {
    justify-content: center;
    margin-bottom: 0.75rem;
  }
}

/* 平板竖屏 / 大手机 — 紧凑三列颁奖台 */
@media (max-width: 768px) {
  .ranking-section {
    padding: 3rem 0;
  }

  .ranking-header {
    padding: 1rem 0 1.5rem;
    margin-bottom: 0.5rem;
  }

  .ranking-title {
    font-size: clamp(1.5rem, 5vw, 2.2rem);
  }

  .title-trophy {
    width: 3rem;
  }

  .ranking-tabs-row {
    justify-content: center;
    margin-bottom: 1rem;
  }

  .ranking-tab {
    padding: 0.35rem 1rem;
    font-size: 0.78rem;
  }

  /* 紧凑三列：横排，极简信息 */
  .podium {
    flex-direction: row;
    align-items: stretch;
    gap: 0.5rem;
    margin-bottom: 1.5rem;
  }

  .podium-card {
    flex: 1;
    max-width: none;
    border-radius: 0.65rem;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    text-align: center;
    padding: 0.65rem 0.4rem;
    min-height: 0;
  }

  .podium-card--second {
    order: 1;
    margin-bottom: 0;
  }
  .podium-card--first {
    order: 2;
    margin-bottom: 0;
    max-width: none;
    box-shadow: 0 4px 20px rgba(0,0,0,0.1);
    border: 2px solid oklch(0.55 0.20 265 / 0.35);
    border-radius: 0.75rem;
    padding: 0.75rem 0.4rem;
  }
  .podium-card--third {
    order: 3;
    margin-bottom: 0;
  }

  /* 隐藏海报视频，只保留排名数字 */
  .podium-poster {
    display: none;
  }

  /* 排名数字 — 纯排版展示 */
  .podium-rank-num {
    font-size: 1.5rem;
    font-weight: 900;
    display: block;
    margin-bottom: 0.2rem;
    line-height: 1;
  }
  .podium-card--first .podium-rank-num {
    font-size: 2rem;
    color: #dc2626;
  }
  .podium-card--second .podium-rank-num {
    color: #2563eb;
  }
  .podium-card--third .podium-rank-num {
    color: #d97706;
  }

  /* 信息区 — 紧凑居中 */
  .podium-info {
    padding: 0;
    gap: 0.15rem;
    align-items: center;
  }
  .podium-card--first .podium-info {
    padding: 0;
    gap: 0.2rem;
  }

  .podium-major {
    font-size: 0.58rem;
    padding: 0.08rem 0.4rem;
  }
  .podium-card--first .podium-major {
    font-size: 0.62rem;
  }

  .podium-title {
    font-size: 0.72rem;
    -webkit-line-clamp: 2;
    text-align: center;
    line-height: 1.3;
  }
  .podium-card--first .podium-title {
    font-size: 0.78rem;
  }

  .podium-likes {
    gap: 0.2rem;
    padding: 0.2rem 0.45rem;
    margin-top: 0.25rem;
  }
  .podium-likes svg {
    width: 0.7rem;
    height: 0.7rem;
  }
  .podium-likes span {
    font-size: 0.75rem;
  }
  .podium-card--first .podium-likes span {
    font-size: 0.8rem;
  }
}

@media (max-width: 640px) {
  .ranking-section {
    padding: 2.5rem 0;
  }

  .ranking-eyebrow {
    font-size: 0.62rem;
    letter-spacing: 0.15em;
  }

  .ranking-title {
    font-size: clamp(1.35rem, 6vw, 1.8rem);
  }

  .ranking-wave-canvas {
    width: 200vw;
  }

  .ranking-tabs {
    gap: 0;
  }

  .ranking-tab {
    padding: 0.3rem 0.8rem;
    font-size: 0.72rem;
  }

  .podium {
    gap: 0.35rem;
    margin-bottom: 1.25rem;
  }

  .podium-card {
    padding: 0.5rem 0.25rem;
    border-radius: 0.5rem;
  }

  .podium-card--first {
    padding: 0.6rem 0.3rem;
    border-radius: 0.6rem;
  }

  .podium-rank-num {
    font-size: 1.15rem;
  }
  .podium-card--first .podium-rank-num {
    font-size: 1.4rem;
  }

  .podium-title {
    font-size: 0.62rem;
  }
  .podium-card--first .podium-title {
    font-size: 0.68rem;
  }

  .podium-major {
    font-size: 0.5rem;
    padding: 0.05rem 0.3rem;
  }

  .podium-likes {
    padding: 0.15rem 0.35rem;
    gap: 0.15rem;
  }
  .podium-likes svg {
    width: 0.6rem;
    height: 0.6rem;
  }
  .podium-likes span {
    font-size: 0.65rem;
  }
  .podium-card--first .podium-likes span {
    font-size: 0.7rem;
  }

  .rest-card {
    grid-template-columns: 1.75rem 56px 1fr auto;
    gap: 0.5rem;
    padding: 0.625rem 0.75rem;
  }

  .rest-poster {
    width: 56px;
  }

  .rest-rank-num {
    width: 1.75rem;
    height: 1.75rem;
    font-size: 0.8rem;
    border-radius: 0.375rem;
  }

  .rest-title {
    font-size: 0.8rem;
  }

  .rest-meta {
    gap: 0.5rem;
  }

  .rest-arrow {
    width: 1.75rem;
    height: 1.75rem;
    opacity: 0.6;
    transform: translateX(0);
  }

  .rest-arrow svg {
    width: 0.85rem;
    height: 0.85rem;
  }

  .ranking-divider {
    margin-bottom: 0.75rem;
  }
}

@media (max-width: 480px) {
  .ranking-title {
    font-size: 1.2rem;
  }

  .title-trophy {
    width: 2.5rem;
  }

  .ranking-tab {
    padding: 0.25rem 0.65rem;
    font-size: 0.68rem;
  }

  .podium-rank-num {
    font-size: 1.15rem;
  }
  .podium-card--first .podium-rank-num {
    font-size: 1.5rem;
  }

  .podium-likes span {
    font-size: 0.8rem;
  }
  .podium-card--first .podium-likes span {
    font-size: 0.9rem;
  }

  .rest-card {
    grid-template-columns: 1.5rem 48px 1fr auto;
    gap: 0.4rem;
    padding: 0.5rem 0.6rem;
    border-radius: 0.65rem;
  }

  .rest-poster {
    width: 48px;
  }

  .rest-rank-num {
    width: 1.5rem;
    height: 1.5rem;
    font-size: 0.72rem;
  }

  .rest-title {
    font-size: 0.75rem;
  }

  .rest-major {
    font-size: 0.58rem;
    padding: 0.06rem 0.4rem;
  }

  .rest-likes {
    font-size: 0.7rem;
    padding: 0.15rem 0.4rem;
  }

  .rest-likes svg {
    width: 0.65rem;
    height: 0.65rem;
  }

  .rest-arrow {
    width: 1.5rem;
    height: 1.5rem;
  }
}
</style>
