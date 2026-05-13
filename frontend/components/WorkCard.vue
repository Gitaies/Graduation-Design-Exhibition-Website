<template>
  <NuxtLink
    :to="`/works/${work.id}`"
    class="work-card group block"
  >
    <div class="relative overflow-hidden rounded-2xl bg-white/80 backdrop-blur-sm shadow-sm hover:shadow-2xl hover:shadow-primary-blue/20 transition-all duration-300 hover:-translate-y-2">
      <!-- 视频封面 -->
      <div class="relative aspect-video overflow-hidden bg-gradient-to-br from-gray-100 to-gray-200">
        <video
          v-if="work.poster_url"
          :src="work.poster_url"
          preload="metadata"
          muted
          playsinline
          disablepictureinpicture
          class="w-full h-full object-cover group-hover:scale-110 transition-transform duration-500"
          @loadeddata="(e) => { const v = e.target as HTMLVideoElement; v.currentTime = 5; v.pause(); }"
        />
        <div v-else class="w-full h-full flex items-center justify-center text-gray-400">
          <svg class="w-16 h-16" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
          </svg>
        </div>

        <!-- 时长标签 -->
        <div class="absolute bottom-3 right-3 px-2.5 py-1 bg-black/70 backdrop-blur-sm text-white text-xs font-medium rounded-lg">
          {{ work.duration }}
        </div>

        <!-- 播放按钮覆盖层 -->
        <div class="absolute inset-0 bg-black/0 group-hover:bg-black/30 transition-colors duration-300 flex items-center justify-center">
          <div class="w-16 h-16 rounded-full bg-white/90 backdrop-blur-sm flex items-center justify-center opacity-0 group-hover:opacity-100 transform scale-75 group-hover:scale-100 transition-all duration-300">
            <svg class="w-8 h-8 text-primary-blue ml-1" fill="currentColor" viewBox="0 0 24 24">
              <path d="M8 5v14l11-7z" />
            </svg>
          </div>
        </div>
      </div>

      <!-- 作品信息 -->
      <div class="p-5">
        <!-- 标题 -->
        <h3 class="text-lg font-bold text-text-main mb-2 line-clamp-2 group-hover:text-primary-blue transition-colors">
          {{ work.title }}
        </h3>

        <!-- 简介 -->
        <p class="text-sm text-text-secondary mb-4 line-clamp-2 leading-relaxed">
          {{ work.introduction }}
        </p>

        <!-- 标签 -->
        <div class="flex flex-wrap gap-2 mb-4">
          <span
            v-for="tag in work.tags.slice(0, 3)"
            :key="tag"
            class="px-2.5 py-1 bg-primary-blue/5 text-primary-blue text-xs rounded-lg"
          >
            {{ tag }}
          </span>
        </div>

        <!-- 底部信息 -->
        <div class="flex items-center justify-between pt-4 border-t border-gray-100">
          <!-- 作者信息 -->
          <div class="flex items-center gap-2 text-sm text-text-secondary">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
            <span class="truncate">
              {{ formatAuthors(work.authors) }}
            </span>
          </div>

          <!-- 交互统计 -->
          <div class="flex items-center gap-4 text-sm text-text-secondary">
            <div class="flex items-center gap-1">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
              </svg>
              <span>{{ interaction?.likeCount || 0 }}</span>
            </div>
            <div class="flex items-center gap-1">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
              </svg>
              <span>{{ interaction?.commentCount || 0 }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </NuxtLink>
</template>

<script setup lang="ts">
import type { Work } from '~/types/work'

interface Interaction {
  workId: string
  likeCount: number
  commentCount: number
  likedByMe: boolean
}

const props = defineProps<{
  work: Work
}>()

const interactionsStore = useInteractionsStore()

const interaction = computed(() => {
  return interactionsStore.getInteraction(props.work.id)
})

const formatAuthors = (authors: string[]) => {
  if (authors.length === 0) return '未知作者'
  if (authors.length === 1) return authors[0]
  if (authors.length === 2) return authors.join('、')
  return `${authors[0]} 等${authors.length}人`
}
</script>

<style scoped>
.work-card {
  display: block;
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* ============================================
   响应式
   ============================================ */
@media (max-width: 768px) {
  .work-card .p-5 {
    padding: 0.875rem;
  }

  .work-card h3 {
    font-size: 0.95rem;
  }

  .work-card .text-sm.text-text-secondary {
    font-size: 0.8rem;
  }

  .work-card .px-2\.5 {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
    font-size: 0.7rem;
  }
}

@media (max-width: 480px) {
  .work-card .rounded-2xl {
    border-radius: 1rem;
  }

  .work-card .p-5 {
    padding: 0.75rem;
  }

  .work-card h3 {
    font-size: 0.85rem;
    margin-bottom: 0.35rem;
  }

  .work-card .text-sm.text-text-secondary {
    font-size: 0.72rem;
    margin-bottom: 0.75rem;
  }

  .work-card .flex.flex-wrap.gap-2 {
    gap: 0.35rem;
    margin-bottom: 0.75rem;
  }

  .work-card .w-16.h-16 {
    width: 2.5rem;
    height: 2.5rem;
  }

  .work-card .w-8.h-8 {
    width: 1.5rem;
    height: 1.5rem;
  }

  .work-card .bottom-3 {
    bottom: 0.5rem;
    right: 0.5rem;
  }
}
</style>
