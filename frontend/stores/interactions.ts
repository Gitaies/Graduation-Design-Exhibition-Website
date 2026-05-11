import { defineStore } from 'pinia'
import { ref } from 'vue'

interface Interaction {
  workId: string
  likeCount: number
  commentCount: number
  likedByMe: boolean
}

interface ApiResponse<T = any> {
  code: number
  message: string
  data: T
}

export const useInteractionsStore = defineStore('interactions', () => {
  // 在 store 首次实例化（位于组件 setup 上下文）时一次性捕获 Nuxt 运行时配置，
  // 避免在 action 内 await 之后再调用 useRuntimeConfig 导致脱离 Nuxt 实例上下文报错。
  const config = useRuntimeConfig()
  const visitorStore = useVisitorStore()
  const apiBase = config.public.apiBase as string

  const interactions = ref<Record<string, Interaction>>({})

  const getInteraction = (workId: string): Interaction | null => {
    return interactions.value[workId] || null
  }

  async function fetchSummary(workIds: string[]) {
    if (!workIds || workIds.length === 0) return
    try {
      const response = await $fetch<ApiResponse<any[]>>(`${apiBase}/interactions/summary`, {
        params: { work_ids: workIds.join(',') },
        headers: { 'X-Visitor-Id': visitorStore.getVisitorIdForRequest() }
      })
      if (response.code === 0 && response.data) {
        response.data.forEach((item: any) => {
          interactions.value[item.work_id] = {
            workId: item.work_id,
            likeCount: item.like_count,
            commentCount: item.comment_count,
            likedByMe: item.liked_by_me ?? false
          }
        })
      }
    } catch (error) {
      console.error('获取交互统计失败:', error)
    }
  }

  async function fetchInteraction(workId: string) {
    try {
      const response = await $fetch<ApiResponse<any>>(`${apiBase}/works/${workId}/interaction`, {
        headers: { 'X-Visitor-Id': visitorStore.getVisitorIdForRequest() }
      })
      if (response.code === 0 && response.data) {
        interactions.value[workId] = {
          workId: response.data.work_id,
          likeCount: response.data.like_count,
          commentCount: response.data.comment_count,
          likedByMe: response.data.liked_by_me
        }
      }
    } catch (error) {
      console.error('获取作品交互状态失败:', error)
    }
  }

  async function toggleLike(workId: string) {
    try {
      const response = await $fetch<ApiResponse<any>>(`${apiBase}/works/${workId}/like/toggle`, {
        method: 'POST',
        headers: { 'X-Visitor-Id': visitorStore.getVisitorIdForRequest() },
        body: { client_ts: Date.now() }
      })

      if (response.code === 0 && response.data) {
        if (interactions.value[workId]) {
          interactions.value[workId].likedByMe = response.data.liked
          interactions.value[workId].likeCount = response.data.like_count
        } else {
          interactions.value[workId] = {
            workId: response.data.work_id,
            likeCount: response.data.like_count,
            commentCount: 0,
            likedByMe: response.data.liked
          }
        }
      }

      return response
    } catch (error) {
      console.error('切换点赞状态失败:', error)
      throw error
    }
  }

  function updateCommentCount(workId: string, delta: number) {
    if (interactions.value[workId]) {
      interactions.value[workId].commentCount += delta
    }
  }

  return {
    interactions,
    getInteraction,
    fetchSummary,
    fetchInteraction,
    toggleLike,
    updateCommentCount
  }
})
