<template>
  <div class="comment-panel bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-sm">
    <h2 class="text-xl font-bold text-text-main mb-6 flex items-center gap-2">
      <svg class="w-6 h-6 text-primary-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
      </svg>
      评论 ({{ totalCommentCount }})
    </h2>

    <!-- 评论输入框 -->
    <div class="mb-8">
      <textarea
        v-model="commentContent"
        placeholder="说点什么吧..."
        rows="3"
        :maxlength="200"
        class="w-full px-4 py-3 border-2 border-gray-200 rounded-xl focus:border-primary-blue focus:outline-none resize-none transition-colors"
      ></textarea>
      <div class="flex items-center justify-between mt-3">
        <span class="text-sm text-text-light">
          已输入 {{ charCount }} / 200 字
        </span>
        <button
          @click="submitComment"
          :disabled="!commentContent.trim() || isSubmitting"
          :class="[
            'px-6 py-2 rounded-lg font-medium transition-all',
            commentContent.trim() && !isSubmitting
              ? 'bg-primary-blue text-white hover:bg-primary-blue-dark shadow-md hover:shadow-lg'
              : 'bg-gray-200 text-gray-400 cursor-not-allowed'
          ]"
        >
          {{ isSubmitting ? '发送中...' : '发送' }}
        </button>
      </div>
    </div>

    <!-- 评论列表 -->
    <div v-if="comments.length > 0" class="space-y-4">
      <div
        v-for="comment in displayedComments"
        :key="comment.id"
        class="comment-item p-4 bg-gray-50 rounded-xl hover:bg-gray-100 transition-colors"
      >
        <div class="flex items-start gap-3">
          <!-- 头像 -->
          <div class="w-10 h-10 rounded-full bg-gradient-to-br from-primary-blue to-primary-cyan flex items-center justify-center text-white font-medium flex-shrink-0">
            {{ comment.visitor_name.slice(-2) }}
          </div>

          <div class="flex-1 min-w-0">
            <!-- 用户名和时间 -->
            <div class="flex items-center gap-2 mb-2">
              <span class="font-medium text-text-main">{{ comment.visitor_name }}</span>
              <span class="text-sm text-text-light">{{ formatTime(comment.created_at) }}</span>
            </div>

            <!-- 评论内容 -->
            <p class="text-text-secondary leading-relaxed break-words">
              {{ comment.content }}
            </p>
          </div>
        </div>
      </div>

      <!-- 加载更多按钮 -->
      <button
        v-if="hasMore"
        @click="loadMore"
        :disabled="isLoadingMore"
        class="w-full py-3 text-primary-blue hover:bg-primary-blue/5 rounded-xl font-medium transition-colors"
      >
        {{ isLoadingMore ? '加载中...' : '查看更多评论' }}
      </button>
    </div>

    <!-- 空状态 -->
    <div v-else class="text-center py-12">
      <svg class="w-16 h-16 mx-auto mb-4 text-gray-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
      </svg>
      <p class="text-text-secondary">暂无评论，快来抢沙发吧~</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

interface Comment {
  id: string
  work_id: string
  visitor_name: string
  content: string
  created_at: string
}

const props = defineProps<{
  workId: string
}>()

const config = useRuntimeConfig()
const visitorStore = useVisitorStore()
const interactionsStore = useInteractionsStore()

const comments = ref<Comment[]>([])
const commentContent = ref('')
const isSubmitting = ref(false)
const isLoadingMore = ref(false)
const nextCursor = ref<string | null>(null)
const hasMore = ref(true)

const displayedComments = computed(() => comments.value)

const charCount = computed(() => commentContent.value.length)

const totalCommentCount = computed(() => {
  return interactionsStore.getInteraction(props.workId)?.commentCount ?? comments.value.length
})

// 格式化时间
const formatTime = (timestamp: string) => {
  const now = new Date()
  const time = new Date(timestamp)
  const diff = Math.floor((now.getTime() - time.getTime()) / 1000)

  if (diff < 60) return '刚刚'
  if (diff < 3600) return `${Math.floor(diff / 60)} 分钟前`
  if (diff < 86400) return `${Math.floor(diff / 3600)} 小时前`
  if (diff < 2592000) return `${Math.floor(diff / 86400)} 天前`

  return time.toLocaleDateString('zh-CN')
}

// 加载评论列表
const loadComments = async (cursor?: string) => {
  try {
    const params: any = { limit: 20 }
    if (cursor) {
      params.cursor = cursor
    }

    const response = await $fetch(`${config.public.apiBase}/works/${props.workId}/comments`, {
      params,
      headers: {
        'X-Visitor-Id': visitorStore.getVisitorIdForRequest()
      }
    })

    if (response.code === 0 && response.data) {
      if (cursor) {
        comments.value.push(...response.data.items)
      } else {
        comments.value = response.data.items
      }
      nextCursor.value = response.data.next_cursor
      hasMore.value = response.data.has_more
    }
  } catch (error) {
    console.error('加载评论失败:', error)
  }
}

// 提交评论
const submitComment = async () => {
  if (!commentContent.value.trim() || isSubmitting.value) return

  isSubmitting.value = true

  try {
    const response = await $fetch(`${config.public.apiBase}/works/${props.workId}/comments`, {
      method: 'POST',
      headers: {
        'X-Visitor-Id': visitorStore.getVisitorIdForRequest()
      },
      body: {
        content: commentContent.value.trim(),
        client_ts: Date.now(),
        website: '' // 蜜罐字段
      }
    })

    if (response.code === 0 && response.data) {
      // 添加新评论到列表顶部
      comments.value.unshift(response.data)
      commentContent.value = ''

      // 更新评论数
      interactionsStore.updateCommentCount(props.workId, 1)

      alert('评论已发布')
    } else {
      alert(response.message || '评论失败')
    }
  } catch (error: any) {
    console.error('评论失败:', error)
    if (error.data?.message) {
      alert(error.data.message)
    } else {
      alert('评论失败，请稍后再试')
    }
  } finally {
    isSubmitting.value = false
  }
}

// 加载更多评论
const loadMore = async () => {
  if (isLoadingMore.value || !hasMore.value || !nextCursor.value) return

  isLoadingMore.value = true

  try {
    await loadComments(nextCursor.value)
  } finally {
    isLoadingMore.value = false
  }
}

// 组件挂载时加载评论
onMounted(async () => {
  await loadComments()
})
</script>

<style scoped>
.comment-item {
  animation: fadeIn 0.3s ease-in-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* ============================================
   响应式
   ============================================ */
@media (max-width: 768px) {
  .comment-panel {
    padding: 1.25rem;
    border-radius: 1rem;
  }

  .comment-panel h2 {
    font-size: 1.1rem;
    margin-bottom: 1rem;
  }

  .comment-panel textarea {
    font-size: 0.9rem;
    padding: 0.75rem 0.875rem;
  }
}

@media (max-width: 640px) {
  .comment-panel {
    padding: 1rem;
  }

  .comment-panel h2 {
    font-size: 1rem;
  }

  .comment-panel h2 svg {
    width: 1.25rem;
    height: 1.25rem;
  }

  .comment-panel textarea {
    font-size: 0.85rem;
    padding: 0.65rem 0.75rem;
    border-radius: 0.75rem;
  }

  .comment-panel .flex.items-center.justify-between {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .comment-panel button {
    width: 100%;
    padding: 0.6rem 1rem;
    font-size: 0.9rem;
  }

  .comment-item {
    padding: 0.75rem;
  }

  .comment-item .w-10.h-10 {
    width: 2.25rem;
    height: 2.25rem;
    font-size: 0.7rem;
  }

  .comment-item .font-medium {
    font-size: 0.85rem;
  }

  .comment-item .text-sm.text-text-light {
    font-size: 0.7rem;
  }

  .comment-item .text-text-secondary {
    font-size: 0.85rem;
  }
}

@media (max-width: 480px) {
  .comment-panel {
    padding: 0.75rem;
  }

  .comment-panel textarea {
    font-size: 0.8rem;
    rows: 2;
  }
}
</style>
