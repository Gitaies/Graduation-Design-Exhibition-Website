<template>
  <button
    @click="toggleLike"
    :disabled="isLoading"
    :class="[
      'like-button w-full px-6 py-4 rounded-xl font-medium transition-all duration-300 flex items-center justify-center gap-3',
      isLiked
        ? 'bg-gradient-to-r from-pink-500 to-red-500 text-white shadow-lg shadow-pink-500/30 hover:shadow-xl hover:shadow-pink-500/40'
        : 'bg-white border-2 border-gray-200 text-text-main hover:border-primary-blue hover:text-primary-blue hover:shadow-md',
      isLoading && 'opacity-50 cursor-not-allowed'
    ]"
  >
    <!-- 爱心图标 -->
    <svg
      class="w-6 h-6 transition-transform duration-300"
      :class="{ 'scale-110': isLiked, 'animate-bounce': justLiked }"
      :fill="isLiked ? 'currentColor' : 'none'"
      stroke="currentColor"
      viewBox="0 0 24 24"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"
      />
    </svg>

    <span class="text-lg">
      {{ isLiked ? '已点赞' : '点赞' }}
    </span>

    <!-- 加载动画 -->
    <div v-if="isLoading" class="w-5 h-5 border-2 border-current border-t-transparent rounded-full animate-spin"></div>
  </button>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

const props = defineProps<{
  workId: string
}>()

const interactionsStore = useInteractionsStore()
const isLoading = ref(false)
const justLiked = ref(false)

const isLiked = computed(() => {
  return interactionsStore.getInteraction(props.workId)?.likedByMe || false
})

// 从 interactions store 获取点赞状态
onMounted(async () => {
  await interactionsStore.fetchInteraction(props.workId)
})

const toggleLike = async () => {
  if (isLoading.value) return

  isLoading.value = true

  try {
    const response = await interactionsStore.toggleLike(props.workId)

    // 检查业务错误码
    if (response && response.code !== 0) {
      alert(response.message || '操作失败')
      return
    }

    // 点赞动画
    if (response.code === 0 && response.data.liked) {
      justLiked.value = true
      setTimeout(() => {
        justLiked.value = false
      }, 600)
    }
  } catch (error: any) {
    console.error('点赞失败:', error)
    if (error.data?.message) {
      alert(error.data.message)
    } else {
      alert('操作失败，请稍后再试')
    }
  } finally {
    isLoading.value = false
  }
}
</script>

<style scoped>
@keyframes bounce {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.2);
  }
}

.animate-bounce {
  animation: bounce 0.6s ease-in-out;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}
</style>
