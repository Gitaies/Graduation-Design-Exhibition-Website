<template>
  <section id="hero" class="hero-section min-h-screen flex items-center justify-center relative overflow-hidden">
    <!-- 背景图片 -->
    <div class="absolute inset-0 hero-background"></div>

    <!-- 波浪粒子画布 -->
    <canvas ref="particleCanvas" class="absolute inset-0 z-[1]"></canvas>

    <!-- 装饰图标（3D 跟随鼠标） -->
    <div class="absolute right-40 top-1/2 transform -translate-y-1/2 hero-icon-container">
      <ClientOnly>
        <HeroIcon3D />
        <template #fallback>
          <img src="~/assets/images/hero/hero_icon.png" alt="装饰" class="hero-icon" />
        </template>
      </ClientOnly>
    </div>

    <div class="container mx-auto px-4 md:px-4 lg:px-4 relative z-10">
      <div class="flex flex-col justify-center items-start min-h-screen py-20">
        <!-- 文字内容 -->
        <div class="text-content max-w-3xl">
          <!-- 主标题 -->
          <h1 class="hero-title mb-4 fade-in-up" data-delay="0">
            <span class="inline-block pr-3 bg-gradient-to-r from-primary-blue via-primary-cyan to-primary-blue bg-clip-text text-transparent animate-gradient relative">
              智媒融合·创启未来
              <!-- 装饰元素 -->
              <span class="title-decoration">
                <!-- 星星 -->
                <svg class="decoration-star" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 2L14.5 9.5L22 12L14.5 14.5L12 22L9.5 14.5L2 12L9.5 9.5L12 2Z" />
                </svg>
                <!-- 长斜线 -->
                <div class="decoration-line decoration-line-long"></div>
                <!-- 短斜线 -->
                <div class="decoration-line decoration-line-short"></div>
              </span>
            </span>
          </h1>

          <!-- 副标题 -->
          <p class="hero-subtitle mb-4 fade-in-up" data-delay="100">
            传媒技术学院2026届毕业设计展
          </p>

          <!-- 英文标题 -->
          <div class="hero-english mb-4 fade-in-up" data-delay="200">
            <div class="english-line">
              <span class="english-decoration-left"></span>
              <p>MEDIA FUSION</p>
            </div>
            <p>CREATE THE FUTURE</p>
            <p>GRADUATION EXHIBITION</p>
            <div class="english-line">
              <p>2026</p>
              <span class="english-decoration-right"></span>
            </div>
          </div>

          <!-- 描述文字 -->
          <p class="hero-description mb-6 fade-in-up" data-delay="300">
            本次毕业设计展以"智媒融合·创启未来"为主题，<br />
            展示软件工程、电子信息工程、广播电视工程三大专业的<br />
            创新成果与实践探索，揭示未来媒介技术的无限可能，<br />
            共同迈向智能媒体新纪元的无限可能。
          </p>

          <!-- 三个专业卡片 -->
          <div class="major-cards mb-8 fade-in-up" data-delay="400">
            <NuxtLink
              v-for="major in majors"
              :key="major.code"
              :to="`/major/${major.code}`"
              class="major-card group"
            >
              <div class="major-icon">
                <svg class="icon-svg" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z" />
                </svg>
              </div>
              <div class="major-info">
                <h3 class="major-name">{{ major.name }}</h3>
                <p class="major-theme">{{ major.theme }}</p>
              </div>
            </NuxtLink>
          </div>

          <!-- 进入展区按钮 -->
          <div class="fade-in-up" data-delay="500">
            <a
              href="#majors"
              class="hero-button group"
              @click.prevent="scrollToSection('#majors')"
            >
              进入展区
              <svg class="button-arrow" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
              </svg>
            </a>
          </div>
        </div>
      </div>
    </div>

    <div class="absolute bottom-8 left-1/2 transform -translate-x-1/2 animate-bounce">
      <div class="w-6 h-10 border-2 border-primary-blue rounded-full flex items-start justify-center p-2">
        <div class="w-1 h-3 bg-primary-blue rounded-full animate-scroll"></div>
      </div>
    </div>

    <!-- 右下角装饰文字 -->
    <div class="corner-decoration">
      <p class="corner-year">2026</p>
      <p class="corner-text">MEDIA TECHNOLOGY</p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { gsap } from 'gsap'
import { majors } from '~/constants/majors'
import { useParticles } from '~/composables/useParticles'

const particleCanvas = ref<HTMLCanvasElement | null>(null)

useParticles(particleCanvas)

const scrollToSection = (path: string) => {
  const sectionId = path.replace('#', '')
  const element = document.getElementById(sectionId)

  if (element) {
    const headerHeight = 80
    const elementPosition = element.getBoundingClientRect().top
    const offsetPosition = elementPosition + window.pageYOffset - headerHeight

    window.scrollTo({
      top: offsetPosition,
      behavior: 'smooth'
    })
  }
}

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
</script>

<style scoped>
.hero-section {
  position: relative;
  min-height: 100vh;
}

.hero-background {
  background-image: url('~/assets/images/hero/hero_bg.png');
  background-size: cover;
  background-position: center top;
  background-repeat: no-repeat;
  z-index: 0;
}

canvas {
  pointer-events: none;
}

.hero-icon-container {
  width: 640px;
  height: 420px;
  z-index: 5;
}

.hero-icon {
  width: 100%;
  height: 100%;
  object-fit: contain;
  animation: float 8s ease-in-out infinite;
}

/* 主标题样式 */
.hero-title {
  font-size: clamp(2.5rem, 5vw, 4.5rem);
  font-weight: 900;
  line-height: 1.1;
  letter-spacing: -0.02em;
  font-family: 'Microsoft YaHei', 'PingFang SC', 'Noto Sans SC', sans-serif;
  font-style: italic;
  transform: skewX(-5deg);
}

/* 标题装饰元素容器 */
.title-decoration {
  position: absolute;
  right: -120px;
  top: -20px;
  width: 100px;
  height: 120px;
  pointer-events: none;
}

/* 星星装饰 */
.decoration-star {
  position: absolute;
  width: 36px;
  height: 36px;
  color: #1466ff;
  top: -15px;
  left: 0;
  animation: twinkle 2s ease-in-out infinite;
}

/* 长斜线 */
.decoration-line-long {
  position: absolute;
  width: 4px;
  height: 100px;
  background: linear-gradient(135deg, #37c8ff 0%, rgba(55, 200, 255, 0) 100%);
  top: 10px;
  right: 10px;
  transform: rotate(20deg);
  border-radius: 2px;
}

/* 短斜线 */
.decoration-line-short {
  position: absolute;
  width: 3px;
  height: 60px;
  background: linear-gradient(135deg, #1466ff 0%, rgba(20, 102, 255, 0) 100%);
  bottom: -20px;
  left: 10px;
  transform: rotate(23deg);
  border-radius: 2px;
}

/* 星星闪烁动画 */
@keyframes twinkle {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.6;
    transform: scale(0.9);
  }
}

/* 副标题样式 */
.hero-subtitle {
  font-size: clamp(1.125rem, 2vw, 2rem);
  font-weight: 500;
  color: oklch(0.3 0.01 250);
  letter-spacing: 0.01em;
}

/* 英文标题样式 */
.hero-english {
  font-size: 0.6875rem;
  font-weight: 500;
  color: oklch(0.55 0.005 250);
  letter-spacing: 0.15em;
  text-transform: uppercase;
  line-height: 1.6;
  font-family: 'Inter', -apple-system, sans-serif;
}

.english-line {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.english-decoration-left {
  width: 15px;
  height: 3px;
  background: linear-gradient(80deg, transparent 0%, #37c8ff 100%);
  transform: rotate(-65deg);
}

.english-decoration-right {
  width: 20px;
  height: 3px;
  background: linear-gradient(90deg, #1466ff 0%, transparent 100%);
  transform: rotate(-65deg);
}

/* 描述文字样式 */
.hero-description {
  font-size: clamp(0.875rem, 1.5vw, 1rem);
  color: oklch(0.4 0.008 250);
  line-height: 1.75;
  max-width: 38rem;
}

/* 专业卡片容器 */
.major-cards {
  display: flex;
  gap: 1.5rem;
  flex-wrap: wrap;
}

/* 专业卡片 */
.major-card {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.875rem 1.125rem;
  border: 2px solid rgba(20, 102, 255, 0.3);
  border-radius: 0.75rem;
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(8px);
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.major-card:hover {
  transform: translateY(-4px);
  border-color: #1466ff;
  background: rgba(255, 255, 255, 0.8);
  box-shadow: 0 8px 24px rgba(20, 102, 255, 0.2);
}

/* 专业图标 */
.major-icon {
  width: 3rem;
  height: 3rem;
  border: 2px solid #1466ff;
  border-radius: 0.625rem;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  transition: all 0.3s ease;
}

.major-card:hover .major-icon {
  background: #1466ff;
  box-shadow: 0 8px 24px rgba(20, 102, 255, 0.3);
}

.icon-svg {
  width: 1.5rem;
  height: 1.5rem;
  color: #1466ff;
  transition: color 0.3s ease;
}

.major-card:hover .icon-svg {
  color: white;
}

/* 专业信息 */
.major-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.major-name {
  font-size: 1rem;
  font-weight: 700;
  color: oklch(0.15 0.01 250);
  letter-spacing: -0.01em;
}

.major-theme {
  font-size: 0.875rem;
  color: #1466ff;
  font-weight: 500;
}

/* 进入展区按钮 */
.hero-button {
  display: inline-flex;
  align-items: center;
  gap: 0.625rem;
  padding: 0.875rem 2rem;
  background: #1466ff;
  color: white;
  border-radius: 0.5rem;
  font-size: 1rem;
  font-weight: 600;
  transition: all 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  box-shadow: 0 4px 16px rgba(20, 102, 255, 0.2);
}

.hero-button:hover {
  background: #37c8ff;
  transform: scale(1.05);
  box-shadow: 0 12px 32px rgba(55, 200, 255, 0.4);
}

.button-arrow {
  width: 1.25rem;
  height: 1.25rem;
  transition: transform 0.3s ease;
}

.hero-button:hover .button-arrow {
  transform: translateX(0.25rem);
}

@keyframes float {
  0%, 100% {
    transform: translateY(0px);
  }
  50% {
    transform: translateY(-20px);
  }
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

/* 响应式调整 */
@media (max-width: 768px) {
  .major-cards {
    flex-direction: column;
    gap: 1rem;
  }

  .hero-description br {
    display: none;
  }

  .corner-decoration {
    display: none;
  }
}

/* 右下角装饰文字 */
.corner-decoration {
  position: absolute;
  bottom: 2rem;
  right: 2rem;
  text-align: right;
  z-index: 10;
}

.corner-year {
  font-size: 1.25rem;
  font-weight: 700;
  color: oklch(41.225% 0.00216 248.372);
  letter-spacing: 0.05em;
  margin-bottom: 0.25rem;
}

.corner-text {
  font-size: 0.625rem;
  font-weight: 600;
  color: oklch(41.225% 0.00216 248.372);
  letter-spacing: 0.2em;
  text-transform: uppercase;
  font-family: 'Inter', -apple-system, sans-serif;
}
</style>
