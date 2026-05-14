<template>
  <section id="hero" class="hero-section min-h-screen flex items-center relative overflow-hidden">
    <!-- 背景图片 -->
    <div class="absolute inset-0 hero-background"></div>

    <!-- 波浪粒子画布 -->
    <canvas ref="particleCanvas" class="absolute inset-0 z-[1]"></canvas>

    <div class="container mx-auto px-4 md:px-8 lg:px-12 relative z-10">
      <div class="hero-inner flex flex-col lg:flex-row items-center lg:items-start justify-between min-h-screen py-20 lg:py-24">
        <!-- 文字内容区 -->
        <div class="text-content max-w-2xl lg:max-w-xl xl:max-w-2xl pt-8 lg:pt-16">
          <!-- 英文小标 -->
          <p class="hero-eyebrow fade-in-up" data-delay="0">
            GRADUATION EXHIBITION 2026
          </p>

          <!-- 主标题 -->
          <h1 class="hero-title fade-in-up" data-delay="100">
            智媒融合·创启未来
          </h1>

          <!-- 副标题 -->
          <p class="hero-subtitle fade-in-up" data-delay="200">
            传媒技术学院2026届毕业设计展
          </p>

          <!-- 英文标题行 -->
          <p class="hero-english fade-in-up" data-delay="250">
            MEDIA FUSION &nbsp;·&nbsp; CREATE THE FUTURE
          </p>

          <!-- 描述文字 -->
          <p class="hero-description fade-in-up" data-delay="300">
            展示软件工程、电子信息工程、广播电视工程三大专业的创新成果与实践探索，揭示未来媒介技术的无限可能，共同迈向智能媒体新纪元。
          </p>

          <!-- 三个专业入口卡片 -->
          <div class="major-cards fade-in-up" data-delay="400">
            <NuxtLink
              v-for="major in majors"
              :key="major.code"
              :to="`/major/${major.code}`"
              class="major-card group"
            >
              <span class="major-num">{{ major.number }}</span>
              <div class="major-divider"></div>
              <div class="major-info">
                <span class="major-name">{{ major.name }}</span>
                <span class="major-theme">{{ major.theme }}</span>
              </div>
              <svg class="major-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
              </svg>
            </NuxtLink>
          </div>

          <!-- CTA 按钮 -->
          <div class="fade-in-up" data-delay="500">
            <a
              href="#majors"
              class="hero-cta group"
              @click.prevent="scrollToSection('#majors')"
            >
              <span class="cta-surface" aria-hidden="true"></span>
              <!-- 科技 icon：六边形节点 -->
              <svg class="cta-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
                <path d="M12 2l8.66 5v10l-8.66 5-8.66-5V7z"/>
                <circle cx="12" cy="12" r="2.5" fill="currentColor" stroke="none"/>
                <path d="M12 9.5v-4M12 18.5v-4M7.7 7.6l2.8 1.6M13.5 14.8l2.8 1.6"/>
              </svg>
              <!-- 分割线 -->
              <span class="cta-sep" aria-hidden="true"></span>
              <!-- 文字 -->
              <span class="cta-text">进入展区</span>
              <!-- 箭头 -->
              <svg class="cta-arrow" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M5 12h14m-4-4l4 4-4 4"/>
              </svg>
            </a>
          </div>
        </div>

        <!-- 3D 装饰图标区 -->
        <div class="hero-visual flex-shrink-0 mt-8 lg:mt-0 lg:ml-8">
          <ClientOnly>
            <HeroIcon3D />
            <template #fallback>
              <img src="~/assets/images/hero/hero_icon.webp" alt="装饰" class="hero-icon-fallback" />
            </template>
          </ClientOnly>
        </div>
      </div>
    </div>

    <!-- 底部滚动指示器 -->
    <div class="absolute bottom-8 left-1/2 transform -translate-x-1/2">
      <div class="scroll-indicator">
        <div class="scroll-dot"></div>
      </div>
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
      y: 40,
      duration: 0.7,
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
  background-image: url('~/assets/images/hero/hero_bg.webp');
  background-size: cover;
  background-position: center top;
  background-repeat: no-repeat;
  z-index: 0;
}

canvas {
  pointer-events: none;
}

/* ============================================
   文字内容区
   ============================================ */

/* 英文小标 */
.hero-eyebrow {
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.2em;
  color: #1466ff;
  text-transform: uppercase;
  margin-bottom: 1rem;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
}

/* 主标题 */
.hero-title {
  font-size: clamp(2.5rem, 5vw, 4.5rem);
  font-weight: 900;
  line-height: 1.08;
  letter-spacing: -0.015em;
  color: #0f172a;
  margin-bottom: 0.75rem;
  white-space: nowrap;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
}

/* 副标题 */
.hero-subtitle {
  font-size: clamp(1.125rem, 2vw, 1.375rem);
  font-weight: 500;
  color: #475569;
  letter-spacing: 0.02em;
  margin-bottom: 0.5rem;
}

/* 英文标题 */
.hero-english {
  font-size: clamp(0.875rem, 1.4vw, 1rem);
  font-weight: 500;
  color: #94a3b8;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-bottom: 1.5rem;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
}

/* 描述文字 */
.hero-description {
  font-size: clamp(0.9rem, 1.3vw, 1rem);
  color: #64748b;
  line-height: 1.8;
  max-width: 36rem;
  margin-bottom: 2rem;
}

/* ============================================
   专业入口卡片
   ============================================ */
.major-cards {
  display: flex;
  gap: 0.75rem;
  flex-wrap: wrap;
  margin-bottom: 2rem;
}

.major-card {
  display: flex;
  align-items: center;
  gap: 0;
  padding: 0.75rem 1.25rem;
  border: 1.5px solid #e2e8f0;
  border-radius: 0.75rem;
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(12px);
  transition: all 0.35s cubic-bezier(0.22, 1, 0.36, 1);
  text-decoration: none;
}

.major-card:hover {
  border-color: #1466ff;
  background: rgba(255, 255, 255, 0.9);
  box-shadow: 0 8px 30px rgba(20, 102, 255, 0.12);
  transform: translateY(-2px);
}

/* 编号 */
.major-num {
  font-size: 1.5rem;
  font-weight: 900;
  color: #cbd5e1;
  letter-spacing: -0.03em;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
  transition: color 0.35s ease;
  min-width: 2rem;
}

.major-card:hover .major-num {
  color: #1466ff;
}

/* 分隔线 */
.major-divider {
  width: 1px;
  height: 1.75rem;
  background: #e2e8f0;
  margin: 0 0.75rem;
  transition: background 0.35s ease;
}

.major-card:hover .major-divider {
  background: #bfdbfe;
}

/* 信息 */
.major-info {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
}

.major-name {
  font-size: 0.9rem;
  font-weight: 700;
  color: #0f172a;
  line-height: 1.2;
}

.major-theme {
  font-size: 0.8rem;
  color: #64748b;
  font-weight: 500;
}

/* 箭头 */
.major-arrow {
  width: 1rem;
  height: 1rem;
  color: #cbd5e1;
  margin-left: 0.75rem;
  transition: all 0.35s ease;
  flex-shrink: 0;
}

.major-card:hover .major-arrow {
  color: #1466ff;
  transform: translateX(3px);
}

/* ============================================
   CTA 按钮
   ============================================ */
.hero-cta {
  position: relative;
  display: inline-flex;
  align-items: center;
  gap: 0;
  padding: 0.92rem 1.75rem;
  background: linear-gradient(135deg, #1d5cf0 0%, #1466ff 60%, #0f47d6 100%);
  border: none;
  border-radius: 0.75rem;
  font-size: 1rem;
  font-weight: 600;
  letter-spacing: 0.05em;
  text-decoration: none;
  isolation: isolate;
  overflow: hidden;
  color: #ffffff;
  transition: transform 0.25s cubic-bezier(0.22, 1, 0.36, 1),
              box-shadow 0.25s ease;
  box-shadow: 0 2px 14px rgba(20, 102, 255, 0.28),
              0 1px 3px rgba(0, 0, 0, 0.06);
}

/* ---- 表面高光线 ---- */
.cta-surface {
  position: absolute;
  inset: 1px;
  border-radius: inherit;
  background: linear-gradient(180deg,
    rgba(255,255,255,0.15) 0%,
    rgba(255,255,255,0.03) 35%,
    transparent 50%
  );
  pointer-events: none;
  z-index: 0;
}

/* ---- 科技 icon：六边形节点 ---- */
.cta-icon {
  position: relative;
  z-index: 1;
  width: 1.25rem;
  height: 1.25rem;
  color: #ffffff;
  flex-shrink: 0;
  margin-right: 0.65rem;
  transition: opacity 0.25s ease;
}

.hero-cta:hover .cta-icon {
  opacity: 0.85;
}

/* ---- 分割线 ---- */
.cta-sep {
  position: relative;
  z-index: 1;
  width: 2px;
  align-self: stretch;
  margin: 2px 0.65rem 2px 0;
  border-radius: 1px;
  background: rgba(255, 255, 255, 0.3);
  transition: background 0.25s ease;
}

.hero-cta:hover .cta-sep {
  background: rgba(255, 255, 255, 0.55);
}

/* ---- 文字 ---- */
.cta-text {
  position: relative;
  z-index: 1;
  line-height: 1;
  margin-right: 0.55rem;
}

/* ---- 箭头 ---- */
.cta-arrow {
  position: relative;
  z-index: 1;
  width: 1.15rem;
  height: 1.15rem;
  flex-shrink: 0;
  color: rgba(255, 255, 255, 0.8);
  transition: transform 0.25s cubic-bezier(0.22, 1, 0.36, 1),
              color 0.25s ease;
}

/* ---- 悬停 ---- */
.hero-cta:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 30px rgba(20, 102, 255, 0.40),
              0 2px 6px rgba(0, 0, 0, 0.08);
}

.hero-cta:hover .cta-arrow {
  transform: translateX(4px);
  color: #ffffff;
}

/* ---- 点击 ---- */
.hero-cta:active {
  transform: translateY(0);
  transition: transform 0.08s ease;
}

/* ============================================
   3D 视觉区
   ============================================ */
.hero-visual {
  width: clamp(345px, 40vw, 560px);
  height: clamp(240px, 30vw, 420px);
}

.hero-icon-fallback {
  width: 100%;
  height: 100%;
  object-fit: contain;
  animation: float 8s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-20px); }
}

/* ============================================
   底部滚动指示器
   ============================================ */
.scroll-indicator {
  width: 1.5rem;
  height: 2.5rem;
  border: 2px solid #cbd5e1;
  border-radius: 9999px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
  padding-top: 0.5rem;
}

.scroll-dot {
  width: 0.25rem;
  height: 0.5rem;
  background: #94a3b8;
  border-radius: 9999px;
  animation: scrollPulse 1.8s ease-in-out infinite;
}

@keyframes scrollPulse {
  0% { transform: translateY(0); opacity: 0.4; }
  50% { transform: translateY(8px); opacity: 1; }
  100% { transform: translateY(16px); opacity: 0; }
}

/* ============================================
   响应式
   ============================================ */

/* 平板横屏 — 缩小视觉区 */
@media (max-width: 1024px) {
  .hero-inner {
    padding-top: 6rem;
    padding-bottom: 5rem;
  }

  .hero-visual {
    width: 280px;
    height: 210px;
  }

  .major-cards {
    gap: 0.6rem;
  }

  .major-card {
    padding: 0.65rem 1rem;
  }
}

/* 平板竖屏 — 保持左右但缩小间距 */
@media (max-width: 900px) {
  .hero-visual {
    width: 240px;
    height: 180px;
  }

  .hero-title {
    font-size: clamp(2rem, 6vw, 3rem);
  }
}

/* 手机横屏 / 小平板 — 堆叠布局 */
@media (max-width: 768px) {
  .hero-background {
    background-image: url('~/assets/images/hero/hero_bg1.webp');
    background-size: auto 100%;
    background-position: center top;
  }

  .hero-section .container {
    padding-left: 0.5rem;
    padding-right: 0.5rem;
  }

  .hero-inner {
    flex-direction: column !important;
    justify-content: flex-start;
    padding-top: 0;
    padding-bottom: 2rem;
  }

  .hero-visual {
    width: 280px;
    height: 210px;
    margin-top: 2rem;
  }

  .text-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    max-width: 100%;
    padding-left: 0;
    padding-right: 0;
    padding-top: 0;
  }

  .hero-description {
    max-width: 90%;
    text-align: center;
  }

  /* 三张入口卡片保持一行 */
  .major-cards {
    flex-direction: row;
    gap: 0.5rem;
    align-items: stretch;
  }

  .major-card {
    flex: 1;
    min-width: 0;
    justify-content: center;
    padding: 0.6rem 0.6rem;
  }

  .hero-cta {
    margin: 1rem auto;
  }
}

/* 大屏手机 — 首屏撑满 */
@media (max-width: 640px) {
  .hero-section {
    min-height: 100dvh;
    display: flex;
    align-items: center;
  }

  .hero-section .container {
    padding-left: 0.25rem;
    padding-right: 0.25rem;
  }

  .hero-inner {
    padding-top: 0;
    padding-bottom: 1.25rem;
    min-height: auto;
  }

  .hero-eyebrow {
    font-size: 0.65rem;
    letter-spacing: 0.15em;
    margin-bottom: 0.75rem;
  }

  .hero-title {
    font-size: clamp(1.8rem, 8vw, 2.5rem);
    text-align: center;
    margin-bottom: 0.6rem;
  }

  .hero-subtitle {
    font-size: 0.9rem;
    text-align: center;
    margin-bottom: 0.5rem;
  }

  .hero-english {
    font-size: 0.68rem;
    text-align: center;
    margin-bottom: 1rem;
  }

  .hero-description {
    font-size: 0.8rem;
    line-height: 1.65;
    max-width: 22rem;
    margin-left: auto;
    margin-right: auto;
    padding: 0 0.5rem;
    margin-bottom: 1.25rem;
  }

  /* 3D 图标 */
  .hero-visual {
    width: 100%;
    max-width: 400px;
    height: auto;
    aspect-ratio: 4 / 3;
    margin-top: 1.5rem;
  }

  /* 三张入口卡片保持一行 */
  .major-cards {
    flex-direction: row;
    gap: 0.4rem;
    margin-bottom: 1rem;
  }

  .major-card {
    flex: 1;
    min-width: 0;
    justify-content: center;
    padding: 0.6rem 0.5rem;
    border-radius: 0.6rem;
  }

  .major-num {
    font-size: 1.1rem;
    min-width: auto;
  }

  .major-name {
    font-size: 0.78rem;
    width: 68px;
  }

  .major-theme {
    font-size: 0.7rem;
  }

  .major-divider {
    margin: 0 0.4rem;
    height: 1.25rem;
  }

  .major-arrow {
    display: none;
  }

  .hero-cta {
    width: 100%;
    justify-content: center;
    padding: 0.75rem 1.5rem;
    font-size: 0.9rem;
  }

  .cta-arrow {
    width: 1rem;
    height: 1rem;
  }

  .scroll-indicator {
    display: none;
  }
}

/* 小屏手机 */
@media (max-width: 480px) {
  .hero-section {
    min-height: 100dvh;
    overflow-x: hidden;
  }

  .hero-section .container {
    padding-left: 0.25rem;
    padding-right: 0.25rem;
  }

  .hero-inner {
    padding-top: 0;
    padding-bottom: 1rem;
  }

  .hero-title {
    font-size: clamp(1.35rem, 6.5vw, 1.8rem);
    margin-bottom: 0.5rem;
  }

  .hero-subtitle {
    font-size: 0.8rem;
    margin-bottom: 0.4rem;
  }

  .hero-description {
    font-size: 0.75rem;
    line-height: 1.6;
    max-width: 20rem;
    margin-left: auto;
    margin-right: auto;
    padding: 0 0.35rem;
    margin-bottom: 1rem;
  }

  .hero-visual {
    width: 100%;
    max-width: 480px;
    height: auto;
    aspect-ratio: 4 / 3;
    margin-top: 3rem;
  }

  .major-cards {
    gap: 0.35rem;
  }

  .major-card {
    padding: 0.55rem 0.5rem;
    border-radius: 0.5rem;
  }

  .major-num {
    font-size: 0.95rem;
  }

  .major-name {
    font-size: 0.7rem;
  }

  .major-theme {
    font-size: 0.6rem;
    white-space: nowrap;
  }

  .major-divider {
    margin: 0 0.25rem;
    height: 1.1rem;
  }
}
</style>
