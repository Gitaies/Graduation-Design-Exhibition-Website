import { defineStore } from 'pinia'

interface Interaction {
  workId: string
  likeCount: number
  commentCount: number
  likedByMe: boolean
}

export const useInteractionsStore = defineStore('interactions', {
  state: () => ({
    interactions: {} as Record<string, Interaction>
  }),

  getters: {
    getInteraction: (state) => (workId: string) => {
      return state.interactions[workId] || null
    }
  },

  actions: {
    async fetchSummary(workIds: string[]) {
      try {
        const config = useRuntimeConfig()
        const visitorStore = useVisitorStore()

        const response = await $fetch(`${config.public.apiBase}/interactions/summary`, {
          params: {
            work_ids: workIds.join(',')
          },
          headers: {
            'X-Visitor-Id': visitorStore.getVisitorIdForRequest()
          }
        })

        if (response.code === 0 && response.data) {
          response.data.forEach((item: any) => {
            this.interactions[item.work_id] = {
              workId: item.work_id,
              likeCount: item.like_count,
              commentCount: item.comment_count,
              likedByMe: item.liked_by_me
            }
          })
        }
      } catch (error) {
        console.error('获取交互统计失败:', error)
      }
    },

    async fetchInteraction(workId: string) {
      try {
        const config = useRuntimeConfig()
        const visitorStore = useVisitorStore()

        const response = await $fetch(`${config.public.apiBase}/works/${workId}/interaction`, {
          headers: {
            'X-Visitor-Id': visitorStore.getVisitorIdForRequest()
          }
        })

        if (response.code === 0 && response.data) {
          this.interactions[workId] = {
            workId: response.data.work_id,
            likeCount: response.data.like_count,
            commentCount: response.data.comment_count,
            likedByMe: response.data.liked_by_me
          }
        }
      } catch (error) {
        console.error('获取作品交互状态失败:', error)
      }
    },

    async toggleLike(workId: string) {
      try {
        const config = useRuntimeConfig()
        const visitorStore = useVisitorStore()

        const response = await $fetch(`${config.public.apiBase}/works/${workId}/like/toggle`, {
          method: 'POST',
          headers: {
            'X-Visitor-Id': visitorStore.getVisitorIdForRequest()
          },
          body: {
            client_ts: Date.now()
          }
        })

        if (response.code === 0 && response.data) {
          // 更新本地状态
          if (this.interactions[workId]) {
            this.interactions[workId].likedByMe = response.data.liked
            this.interactions[workId].likeCount = response.data.like_count
          } else {
            this.interactions[workId] = {
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
    },

    updateCommentCount(workId: string, delta: number) {
      if (this.interactions[workId]) {
        this.interactions[workId].commentCount += delta
      }
    }
  }
})
