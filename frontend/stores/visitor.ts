import { defineStore } from 'pinia'

export const useVisitorStore = defineStore('visitor', {
  state: () => ({
    visitorId: '' as string
  }),

  getters: {
    getVisitorId: (state) => state.visitorId
  },

  actions: {
    initVisitorId() {
      if (process.client) {
        let id = localStorage.getItem('visitor_id')
        if (!id) {
          id = crypto.randomUUID()
          localStorage.setItem('visitor_id', id)
        }
        this.visitorId = id
      }
    },

    getVisitorIdForRequest(): string {
      if (!this.visitorId) {
        this.initVisitorId()
      }
      return this.visitorId
    }
  }
})
