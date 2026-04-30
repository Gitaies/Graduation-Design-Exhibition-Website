<template>
  <div class="home-page">
    <ExhibitionHeader />

    <!-- 主视觉区 -->
    <section class="hero-section min-h-screen flex items-center justify-center relative overflow-hidden">
      <!-- 粒子背景 -->
      <ParticleBackground />

      <!-- 背景装饰 -->
      <div class="absolute inset-0 bg-gradient-to-br from-blue-50 via-cyan-50 to-blue-50"></div>

      <div class="container mx-auto px-4 relative z-10">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
          <!-- 左侧：文字内容 -->
          <div class="text-content">
            <div class="mb-6 fade-in-up" data-delay="0">
              <p class="text-lg text-text-secondary mb-2">传媒技术学院</p>
              <p class="text-sm text-text-light tracking-wider">SCHOOL OF MEDIA TECHNOLOGY</p>
            </div>

            <h1 class="text-5xl md:text-7xl font-bold mb-6 fade-in-up" data-delay="100">
              <span class="block bg-gradient-to-r from-primary-blue via-primary-cyan to-primary-blue bg-clip-text text-transparent animate-gradient">
                智媒融合 · 创启未来
              </span>
            </h1>

            <p class="text-xl md:text-2xl text-text-secondary mb-8 fade-in-up" data-delay="200">
              传媒技术学院 2026 届毕业设计展
            </p>

            <p class="text-base md:text-lg text-text-secondary max-w-2xl mb-12 leading-relaxed fade-in-up" data-delay="300">
              本次毕业设计展以"智媒融合 · 创启未来"为主题，聚焦三大专业的创新成果与应用实践，展现未来媒介技术的无限潜力。
            </p>

            <div class="flex flex-col sm:flex-row gap-4 mb-12 fade-in-up" data-delay="400">
              <NuxtLink
                to="/majors"
                class="group px-8 py-4 bg-gradient-to-r from-primary-blue to-primary-cyan text-white rounded-full text-lg font-medium hover:shadow-2xl hover:shadow-primary-blue/40 transition-all transform hover:scale-105 flex items-center justify-center gap-2"
              >
                进入展区
                <svg class="w-5 h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
                </svg>
              </NuxtLink>
              <NuxtLink
                to="/ranking"
                class="px-8 py-4 bg-white/80 backdrop-blur-sm border-2 border-primary-blue/20 text-primary-blue rounded-full text-lg font-medium hover:bg-white hover:shadow-xl transition-all flex items-center justify-center gap-2"
              >
                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                </svg>
                点赞热榜
              </NuxtLink>
            </div>

            <div class="grid grid-cols-3 gap-4 fade-in-up" data-delay="500">
              <NuxtLink
                v-for="major in majors"
                :key="major.code"
                :to="`/major/${major.code}`"
                class="group flex flex-col items-center p-4 bg-white/60 backdrop-blur-sm rounded-2xl hover:bg-white hover:shadow-xl transition-all transform hover:-translate-y-2"
              >
                <div class="w-12 h-12 bg-gradient-to-br from-primary-blue to-primary-cyan rounded-xl flex items-center justify-center text-white text-xl font-bold mb-2 group-hover:scale-110 transition-transform">
                  {{ major.number }}
                </div>
                <h3 class="text-sm font-bold text-text-main mb-1">{{ major.name }}</h3>
                <p class="text-xs text-text-secondary">{{ major.theme }}</p>
              </NuxtLink>
            </div>
          </div>

          <div class="cube-container hidden lg:block fade-in-up" data-delay="600">
            <TechCube />
          </div>
        </div>
      </div>

      <div class="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce">
        <div class="w-6 h-10 border-2 border-primary-blue rounded-full flex items-start justify-center p-2">
          <div class="w-1 h-3 bg-primary-blue rounded-full animate-scroll"></div>
        </div>
      </div>
    </section>

    <ExhibitionFooter />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { gsap } from 'gsap'

const majors = [
  { code: 'software', number: '01', name: '软件工程', theme: '数智焕新' },
  { code: 'electronic', number: '02', name: '电子信息工程', theme: '芯火智造' },
  { code: 'broadcast', number: '03', name: '广播电视工程', theme: '虚实共生' }
]

onMounted(() => {
  const elements = document.querySelectorAll('.fade-in-up')
  elements.forEach((el) => {
    const delay = parseInt(el.getAttribute('data-delay') || '0')
    gsap.from(el, {
      opacity: 0,
      y: 50,
      duration: 0.8,
      delay: delay / 1000,
      ease: 'power3.out'
    })
  })
})

useHead({
  title: '首页 - 传媒技术学院2026届毕业设计展'
})
</script>

<style scoped>
.hero-section {
  position: relative;
  min-height: 100vh;
}

.cube-container {
  width: 100%;
  height: 500px;
}

@keyframes gradient {
  0%, 100% { background-position: 0% 50%; }
  50% { background-position: 100% 50%; }
}

.animate-gradient {
  background-size: 200% auto;
  animation: gradient 3s ease infinite;
}

@keyframes scroll {
  0% { transform: translateY(0); opacity: 1; }
  100% { transform: translateY(12px); opacity: 0; }
}

.animate-scroll {
  animation: scroll 1.5s ease-in-out infinite;
}
</style>
