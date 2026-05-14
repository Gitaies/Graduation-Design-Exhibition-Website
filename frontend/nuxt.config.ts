export default defineNuxtConfig({
  devtools: { enabled: true },

  modules: [
    '@nuxtjs/tailwindcss',
    '@pinia/nuxt',
    '@vueuse/nuxt'
  ],

  css: ['~/assets/css/main.css'],

  app: {
    head: {
      title: '传媒技术学院2026届毕业设计展',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: '智媒融合 · 创启未来 - 传媒技术学院2026届毕业设计展' }
      ],
      link: [
        { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
      ]
    }
  },

  runtimeConfig: {
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE || '/api'
    }
  },

  vite: {
    server: {
      allowedHosts: ['localhost', '192.168.0.16', 'frontend', '.local']
    },
  },

  nitro: {
    prerender: {
      crawlLinks: true,
      routes: [
        '/',
        '/ranking',
        '/major/software',
        '/major/electronic',
        '/major/broadcast'
      ]
    },
    hooks: {
      'prerender:routes'(routes: Set<string>) {
        for (let i = 1; i <= 33; i++) {
          routes.add(`/works/w${String(i).padStart(3, '0')}`)
        }
      }
    },
    devProxy: {
      '/static': {
        target: 'http://localhost:8080/static',
        changeOrigin: true
      }
    }
  },

  experimental: {
    appManifest: false
  },

  compatibilityDate: '2024-04-26'
})
