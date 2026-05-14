<template>
  <div class="work-detail-page min-h-screen bg-bg-page">
    <ExhibitionHeader />

    <div v-if="work" class="container mx-auto px-4 pt-24 pb-8">
      <!-- 面包屑 -->
      <nav class="mb-6 text-sm">
        <ol class="flex items-center gap-2 text-text-secondary">
          <li><NuxtLink to="/" class="hover:text-primary-blue transition-colors">首页</NuxtLink></li>
          <li>/</li>
          <li><NuxtLink to="/#majors" class="hover:text-primary-blue transition-colors">专业展区</NuxtLink></li>
          <li>/</li>
          <li><NuxtLink :to="`/major/${work.major_code}`" class="hover:text-primary-blue transition-colors">{{ work.major_name }}</NuxtLink></li>
          <li>/</li>
          <li class="text-text-main font-medium">作品详情</li>
        </ol>
      </nav>

      <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <!-- 左侧：视频和作品信息 -->
        <div class="lg:col-span-2 space-y-6">
          <!-- 视频播放器 -->
          <div class="relative aspect-video rounded-2xl overflow-hidden bg-black shadow-2xl">
            <video
              v-if="work.video_url"
              ref="videoRef"
              :src="work.video_url"
              :poster="work.poster_url"
              controls
              preload="metadata"
              playsinline
              class="w-full h-full"
              @loadeddata="onVideoLoaded"
              @play="onVideoPlay"
            >
              您的浏览器不支持视频播放
            </video>
            <div v-else class="w-full h-full flex items-center justify-center text-white">
              <div class="text-center">
                <svg class="w-20 h-20 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
                </svg>
                <p class="text-gray-400">视频暂未上传</p>
              </div>
            </div>
          </div>

          <!-- 作品标题和信息 -->
          <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-sm">
            <h1 class="text-3xl md:text-4xl font-bold text-text-main mb-4">
              {{ work.title }}
            </h1>

            <!-- 标签 -->
            <div class="flex flex-wrap gap-2 mb-6">
              <span class="px-3 py-1.5 bg-primary-blue/10 text-primary-blue text-sm font-medium rounded-lg">
                {{ work.major_name }}
              </span>
              <span class="px-3 py-1.5 bg-primary-cyan/10 text-primary-cyan text-sm font-medium rounded-lg">
                {{ work.theme }}
              </span>
              <span
                v-for="tag in work.tags"
                :key="tag"
                class="px-3 py-1.5 bg-gray-100 text-text-secondary text-sm rounded-lg"
              >
                {{ tag }}
              </span>
            </div>

            <!-- 作品介绍 -->
            <div class="mb-6">
              <h2 class="text-lg font-bold text-text-main mb-3">作品介绍</h2>
              <p class="text-text-secondary leading-relaxed">
                {{ work.introduction }}
              </p>
            </div>

            <!-- 作者信息 -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-6 border-t border-gray-100">
              <div>
                <div class="text-sm text-text-light mb-1">作者</div>
                <div class="text-text-main font-medium">
                  {{ work.authors.join('、') }}
                </div>
              </div>
              <div>
                <div class="text-sm text-text-light mb-1">指导老师</div>
                <div class="text-text-main font-medium">
                  {{ work.teacher }}
                </div>
              </div>
            </div>
          </div>

          <!-- 评论区 -->
          <CommentPanel :workId="work.id" />
        </div>

        <!-- 右侧：交互和分享 -->
        <div class="space-y-6">
          <!-- 交互卡片 -->
          <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-sm sticky top-24">
            <!-- 点赞按钮 -->
            <LikeButton :workId="work.id" class="mb-4" />

            <!-- 统计信息 -->
            <div class="grid grid-cols-2 gap-4 mb-6 pt-6 border-t border-gray-100">
              <div class="text-center">
                <div class="text-2xl font-bold text-primary-blue mb-1">
                  {{ interaction?.likeCount || 0 }}
                </div>
                <div class="text-sm text-text-secondary">点赞</div>
              </div>
              <div class="text-center">
                <div class="text-2xl font-bold text-primary-cyan mb-1">
                  {{ interaction?.commentCount || 0 }}
                </div>
                <div class="text-sm text-text-secondary">评论</div>
              </div>
            </div>

            <!-- 分享按钮 -->
            <button
              @click="sharePanelRef?.open()"
              class="w-full px-4 py-3 bg-gray-100 hover:bg-gray-200 text-text-main rounded-xl font-medium transition-colors flex items-center justify-center gap-2"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
              </svg>
              分享作品
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 加载状态 -->
    <div v-else class="container mx-auto px-4 py-20 text-center">
      <div class="inline-block w-12 h-12 border-4 border-primary-blue border-t-transparent rounded-full animate-spin mb-4"></div>
      <p class="text-text-secondary">加载中...</p>
    </div>

    <ExhibitionFooter />

    <!-- 分享面板 -->
    <SharePanel
      ref="sharePanelRef"
      :work-title="work?.title || ''"
      :work-description="work?.introduction || ''"
      :work-poster="work?.poster_url || ''"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRoute } from 'vue-router'
import SharePanel from '~/components/SharePanel.vue'

const route = useRoute()
const workId = computed(() => route.params.workId as string)
const videoRef = ref<HTMLVideoElement | null>(null)
const posterSet = ref(false)

const onVideoLoaded = () => {
  if (videoRef.value) {
    videoRef.value.currentTime = 5
    posterSet.value = true
  }
}

const onVideoPlay = () => {
  if (posterSet.value && videoRef.value) {
    videoRef.value.currentTime = 0
    posterSet.value = false
  }
}

// 使用 API 加载作品数据
const { fetchWorkDetail } = useWorks()
const work = ref<any>(null)
const loading = ref(true)

try {
  work.value = await fetchWorkDetail(workId.value)
} catch (error) {
  console.error('Failed to load work:', error)
} finally {
  loading.value = false
}

// 从 interactions store 获取交互数据
const interactionsStore = useInteractionsStore()
const interaction = computed(() => interactionsStore.getInteraction(workId.value))

// 分享面板
const sharePanelRef = ref<InstanceType<typeof SharePanel> | null>(null)

// 设置页面标题
useHead({
  title: computed(() => work.value ? `${work.value.title} - 传媒技术学院2026届毕业设计展` : '作品详情')
})

// 组件挂载时获取交互数据
onMounted(async () => {
  await interactionsStore.fetchInteraction(workId.value)
})
</script>

<style scoped>
@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}

/* ============================================
   响应式
   ============================================ */

/* 平板横屏 —— 保持双栏但缩小间距 */
@media (max-width: 1024px) {
  .work-detail-page .grid {
    gap: 1.5rem;
  }

  .work-detail-page .sticky {
    position: static;
  }
}

/* 平板竖屏 —— 切换为单栏 */
@media (max-width: 768px) {
  .work-detail-page .container {
    padding-top: 5rem;
  }

  .work-detail-page .grid.grid-cols-1.lg\\:grid-cols-3 {
    grid-template-columns: 1fr;
    gap: 1.5rem;
  }

  .work-detail-page .lg\\:col-span-2 {
    grid-column: span 1;
  }

  .work-detail-page h1 {
    font-size: 1.5rem;
  }

  .work-detail-page .text-3xl {
    font-size: 1.35rem;
  }

  .work-detail-page .md\\:text-4xl {
    font-size: 1.5rem;
  }

  /* 面包屑 */
  .work-detail-page nav.mb-6 {
    font-size: 0.78rem;
    margin-bottom: 1rem;
  }

  /* 视频 */
  .work-detail-page .aspect-video {
    border-radius: 1rem;
  }

  /* 信息卡片 */
  .work-detail-page .rounded-2xl {
    border-radius: 1rem;
    padding: 1.25rem;
  }

  /* 标签 */
  .work-detail-page .flex.flex-wrap.gap-2 {
    gap: 0.4rem;
  }

  .work-detail-page .px-3.py-1\\.5 {
    padding: 0.35rem 0.75rem;
    font-size: 0.78rem;
  }

  /* 作者信息 */
  .work-detail-page .grid.grid-cols-1.md\\:grid-cols-2 {
    gap: 0.75rem;
  }

  /* 右侧边栏 —— 静态定位 */
  .work-detail-page .sticky {
    position: static;
  }
}

/* 大屏手机 */
@media (max-width: 640px) {
  .work-detail-page .container {
    padding-top: 4rem;
    padding-left: 0.75rem;
    padding-right: 0.75rem;
  }

  .work-detail-page nav.mb-6 ol {
    flex-wrap: wrap;
    gap: 0.25rem;
    font-size: 0.72rem;
  }

  .work-detail-page h1 {
    font-size: 1.25rem;
    margin-bottom: 0.75rem;
  }

  .work-detail-page .rounded-2xl {
    padding: 1rem;
  }

  .work-detail-page .px-3.py-1\\.5 {
    padding: 0.25rem 0.6rem;
    font-size: 0.72rem;
  }

  .work-detail-page .text-lg.font-bold {
    font-size: 1rem;
  }

  .work-detail-page .text-text-secondary.leading-relaxed {
    font-size: 0.85rem;
    line-height: 1.7;
  }

  .work-detail-page .text-sm.text-text-light {
    font-size: 0.75rem;
  }

  .work-detail-page .grid.grid-cols-1.md\\:grid-cols-2 {
    grid-template-columns: 1fr;
    gap: 0.5rem;
  }

  /* 统计数字 */
  .work-detail-page .text-2xl.font-bold {
    font-size: 1.5rem;
  }

  /* 交互卡片 */
  .work-detail-page .space-y-6 > .bg-white\/80 {
    padding: 1rem;
  }
}

/* 小屏手机 */
@media (max-width: 480px) {
  .work-detail-page .container {
    padding-top: 3.5rem;
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  .work-detail-page h1 {
    font-size: 1.15rem;
  }

  .work-detail-page .rounded-2xl {
    padding: 0.875rem;
    border-radius: 0.75rem;
  }

  .work-detail-page .aspect-video {
    border-radius: 0.75rem;
  }

  .work-detail-page .px-3.py-1\\.5 {
    padding: 0.2rem 0.5rem;
    font-size: 0.68rem;
    border-radius: 0.4rem;
  }

  .work-detail-page .text-text-secondary.leading-relaxed {
    font-size: 0.8rem;
    line-height: 1.65;
  }
}
</style>
