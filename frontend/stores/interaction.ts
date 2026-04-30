import { defineStore } from 'pinia'
import type { InteractionData } from '~/types'

export const useInteractionStore = defineStore('interaction', {
  state: () => ({
    interactionMap: {} as Record<string, InteractionData>
  }),

  actions: {
    async fetchSummary(workIds: string[]) {
      const config = useRuntimeConfig()
      const visitorStore = useVisitorStore()

      try {
        const response = await $fetch<{ code: number; data: InteractionData[] }>(
          `${config.public.apiBase}/interactions/summary?work_ids=${workIds.join(',')}`,
          {
            headers: {
              'X-Visitor-Id': visitorStore.getVisitorId()
            }
          }
        )

        if (response.code === 0 && response.data) {
          response.data.forEach(item => {
            this.interactionMap[item.work_id] = item
          })
        }
      } catch (error) {
        console.error('获取交互数据失败:', error)
      }
    },

    async toggleLike(workId: string) {
      const config = useRuntimeConfig()
      const visitorStore = useVisitorStore()

      try {
        const response = await $fetch<{
          code: number
          data: { work_id: string; liked: boolean; like_count: number }
        }>(
          `${config.public.apiBase}/works/${workId}/like/toggle`,
          {
            method: 'POST',
            headers: {
              'X-Visitor-Id': visitorStore.getVisitorId()
            },
            body: {
              client_ts: Date.now()
            }
          }
        )

        if (response.code === 0 && response.data) {
          this.interactionMap[workId] = {
            work_id: workId,
            like_count: response.data.like_count,
            comment_count: this.interactionMap[workId]?.comment_count || 0,
            liked_by_me: response.data.liked
          }
        }

        return response
      } catch (error) {
        console.error('点赞操作失败:', error)
        throw error
      }
    },

    getInteraction(workId: string): InteractionData | null {
      return this.interactionMap[workId] || null
    }
  }
})
