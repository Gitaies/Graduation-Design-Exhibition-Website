import { defineStore } from 'pinia'

// crypto.randomUUID() 仅在 secure context（HTTPS 或 localhost）下可用
// 局域网 HTTP 访问需要 fallback
function generateUUID(): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID()
  }
  return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, (c) => {
    const r = (Math.random() * 16) | 0
    return (c === 'x' ? r : (r & 0x3) | 0x8).toString(16)
  })
}

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
          id = generateUUID()
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
