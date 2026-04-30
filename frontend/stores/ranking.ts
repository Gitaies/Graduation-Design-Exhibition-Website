import { defineStore } from 'pinia'

interface RankingItem {
  rank: number
  work_id: string
  like_count: number
}

export const useRankingStore = defineStore('ranking', {
  state: () => ({
    rankings: {
      all: [] as RankingItem[],
      week: [] as RankingItem[],
      today: [] as RankingItem[]
    },
    loading: false
  }),

  getters: {
    getRankings: (state) => (range: 'all' | 'week' | 'today') => {
      return state.rankings[range]
    }
  },

  actions: {
    async fetchRankings(range: 'all' | 'week' | 'today' = 'all', page: number = 1, pageSize: number = 10) {
      this.loading = true

      try {
        const config = useRuntimeConfig()

        const response = await $fetch(`${config.public.apiBase}/rankings/likes`, {
          params: {
            range,
            page,
            page_size: pageSize
          }
        })

        if (response.code === 0 && response.data) {
          this.rankings[range] = response.data.items
        }

        return response
      } catch (error) {
        console.error('获取排行榜失败:', error)
        throw error
      } finally {
        this.loading = false
      }
    }
  }
})
