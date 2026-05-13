<template>
  <section id="majors" class="majors-section">
    <div class="container mx-auto px-4 md:px-8 lg:px-12">
      <!-- 标题区 -->
      <div class="majors-header">
        <span class="majors-eyebrow">PROFESSIONAL ZONES</span>
        <h2 class="majors-title">专业展区</h2>
        <p class="majors-lead">三大专业领域 · 三十余件优秀作品 · 共同探索智能媒体新纪元</p>
      </div>

      <!-- 专业卡片 -->
      <div class="majors-grid">
        <NuxtLink
          v-for="(major, index) in majorsData"
          :key="major.code"
          :to="`/major/${major.code}`"
          :class="['major-card group', `major-card--${index}`]"
        >
          <!-- 顶部色条 -->
          <div :class="['major-card-accent', `accent--${index}`]"></div>

          <!-- 内容 -->
          <div class="major-card-body">
            <!-- 编号 + 专业名 -->
            <div class="major-card-top">
              <span :class="['major-num', `num--${index}`]">{{ major.number }}</span>
              <span :class="['major-tag', `tag--${index}`]">{{ major.name }}</span>
            </div>

            <!-- 主题大标题 -->
            <h3 :class="['major-theme', `theme--${index}`]">{{ major.theme }}</h3>

            <!-- 副标题 -->
            <p class="major-subtitle">{{ major.subtitle }}</p>

            <!-- 描述 -->
            <p class="major-desc">{{ major.description }}</p>

            <!-- CTA -->
            <div class="major-cta-wrapper">
              <div
                class="magnetic-field"
                @pointermove="handleMagneticMove($event, index)"
                @pointerleave="handleMagneticLeave(index)"
              >
                <span
                  :ref="el => { if (el) magneticButtons[index] = el }"
                  :class="['major-cta', `cta--${index}`, { 'is-active': magneticStates[index]?.isNear }]"
                  :style="{
                    transform: `translate3d(${magneticStates[index]?.x || 0}px, ${magneticStates[index]?.y || 0}px, 0) scale(${magneticStates[index]?.scale || 1})`
                  }"
                >
                  <span class="cta-text">探索作品</span>
                  <svg class="cta-arrow-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M17 8l4 4m0 0l-4 4m4-4H3"/>
                  </svg>
                </span>
              </div>
            </div>
          </div>

          <!-- 背景装饰图标 -->
          <div :class="['major-card-bg-icon', `bg-icon--${index}`]">
            <img
              :src="index === 0 ? '/images/icon_1.png' : index === 1 ? '/images/icon_2.png' : '/images/icon_3.png'"
              alt=""
              class="bg-icon-img"
            />
          </div>
        </NuxtLink>
      </div>

      <!-- 底部统计 -->
      <div class="majors-stats">
        <div class="stats-group">
          <div class="stats-item">
            <span class="stats-value">3</span>
            <span class="stats-label">大专业</span>
          </div>
          <span class="stats-sep">/</span>
          <div class="stats-item">
            <span class="stats-value">30+</span>
            <span class="stats-label">优秀作品</span>
          </div>
          <span class="stats-sep">/</span>
          <div class="stats-item">
            <span class="stats-value">180+</span>
            <span class="stats-label">毕业生</span>
          </div>
        </div>
        <div class="stats-badge">
          <svg class="stats-badge-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8">
            <path stroke-linecap="round" stroke-linejoin="round" d="M13 2L3 14h9l-2 8 10-12h-9l2-8z"/>
          </svg>
          <div class="stats-badge-text">
            <span class="stats-badge-main">无限可能</span>
            <span class="stats-badge-sub">创启未来</span>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { majorsData } from '~/constants/majors'
import { useMagneticButton } from '~/composables/useMagneticButton'

const { magneticButtons, magneticStates, handleMagneticMove, handleMagneticLeave } = useMagneticButton(3)
</script>

<style scoped>
/* ============================================
   区域容器
   ============================================ */
.majors-section {
  padding: 5rem 0 4rem;
  background: linear-gradient(to bottom, #e1eaf8 0%, #F3FBFB 60%, #F3FBFB 100%);
  position: relative;
}

/* ============================================
   标题区
   ============================================ */
.majors-header {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 2rem 0 5rem 0;
  position: relative;
  background: url('/images/153.png') center center / contain no-repeat;
  border-radius: 1.25rem;
  overflow: hidden;
}

.majors-eyebrow {
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.22em;
  color: #94a3b8;
  text-transform: uppercase;
  margin-bottom: 0.35rem;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
}

.majors-title {
  font-size: clamp(2rem, 4.5vw, 2.8rem);
  font-weight: 700;
  color: #0f172a;
  letter-spacing: 0.03em;
  margin-bottom: 0.25rem;
  padding: 0.75rem 3rem;
  background: url('/images/130.png') center center / contain no-repeat;
  border-radius: 0.75rem;
  display: inline-block;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
}

.majors-lead {
  font-size: 0.9rem;
  color: #64748b;
  letter-spacing: 0.03em;
  font-weight: 450;
  margin-top: 2rem;
}

/* ============================================
   卡片网格
   ============================================ */
.majors-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1.75rem;
  margin-bottom: 3rem;
}

/* ============================================
   单张卡片
   ============================================ */
.major-card {
  display: flex;
  flex-direction: column;
  position: relative;
  background: rgba(255, 255, 255, 0.75);
  backdrop-filter: blur(16px);
  border: 1px solid rgba(255, 255, 255, 0.6);
  border-radius: 1.125rem;
  text-decoration: none;
  overflow: hidden;
  transition: transform 0.4s cubic-bezier(0.22, 1, 0.36, 1),
              box-shadow 0.4s cubic-bezier(0.22, 1, 0.36, 1),
              border-color 0.4s ease;
}

.major-card:hover {
  transform: translateY(-4px);
  border-color: rgba(255, 255, 255, 0.85);
}

.major-card--0:hover { box-shadow: 0 16px 48px oklch(0.55 0.18 265 / 0.13); }
.major-card--1:hover { box-shadow: 0 16px 48px oklch(0.50 0.18 285 / 0.13); }
.major-card--2:hover { box-shadow: 0 16px 48px oklch(0.55 0.10 210 / 0.13); }

/* 顶部色条 */
.major-card-accent {
  width: 100%;
  height: 3px;
  flex-shrink: 0;
}

.accent--0 { background: linear-gradient(90deg, #1466ff, #60a5fa); }
.accent--1 { background: linear-gradient(90deg, #7c3aed, #a78bfa); }
.accent--2 { background: linear-gradient(90deg, #0891b2, #22d3ee); }

/* 内容区 */
.major-card-body {
  display: flex;
  flex-direction: column;
  padding: 1.75rem 1.75rem 1.5rem;
  flex: 1;
  position: relative;
  z-index: 2;
}

/* 编号 + 专业标签 */
.major-card-top {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  margin-bottom: 1.5rem;
}

.major-num {
  font-size: 1rem;
  font-weight: 800;
  width: 2rem;
  height: 2rem;
  border-radius: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
  letter-spacing: -0.02em;
  flex-shrink: 0;
}

.num--0 { background: #eff6ff; color: #2563eb; }
.num--1 { background: #f5f3ff; color: #7c3aed; }
.num--2 { background: #ecfeff; color: #0891b2; }

.major-tag {
  font-size: 0.78rem;
  font-weight: 600;
  padding: 0.15rem 0.6rem;
  border-radius: 9999px;
  letter-spacing: 0.02em;
}

.tag--0 { color: #2563eb; background: #eff6ff; }
.tag--1 { color: #7c3aed; background: #f5f3ff; }
.tag--2 { color: #0891b2; background: #ecfeff; }

/* 主题 — 卡片的视觉焦点 */
.major-theme {
  font-size: clamp(2rem, 4.2vw, 2.75rem);
  font-weight: 900;
  line-height: 1.12;
  letter-spacing: 0.015em;
  margin-bottom: 0.625rem;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
}

.theme--0 { color: #1e40af; }
.theme--1 { color: #6d28d9; }
.theme--2 { color: #0e7490; }

/* 副标题 */
.major-subtitle {
  font-size: 0.95rem;
  font-weight: 600;
  color: #475569;
  margin-bottom: 0.5rem;
  letter-spacing: 0.025em;
}

/* 描述 */
.major-desc {
  font-size: 0.85rem;
  color: #64748b;
  line-height: 1.65;
  margin-bottom: 1.75rem;
  max-width: 36ch;
}

/* CTA 按钮区 */
.major-cta-wrapper {
  margin-top: auto;
}

.magnetic-field {
  position: relative;
  display: grid;
  place-items: center;
  width: 100%;
  touch-action: none;
}

.major-cta {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.625rem 1.5rem;
  border-radius: 9999px;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  transition: box-shadow 0.22s ease;
  will-change: transform;
  user-select: none;
  position: relative;
}

.cta--0 { background: #1466ff; color: white; box-shadow: 0 2px 12px oklch(0.5 0.18 265 / 0.22); }
.cta--1 { background: #7c3aed; color: white; box-shadow: 0 2px 12px oklch(0.45 0.18 285 / 0.22); }
.cta--2 { background: #0891b2; color: white; box-shadow: 0 2px 12px oklch(0.5 0.10 210 / 0.22); }

.major-cta.is-active {
  box-shadow: 0 8px 30px oklch(0.5 0.15 265 / 0.28);
}

.cta--0.is-active { box-shadow: 0 8px 32px oklch(0.5 0.18 265 / 0.30); }
.cta--1.is-active { box-shadow: 0 8px 32px oklch(0.45 0.18 285 / 0.30); }
.cta--2.is-active { box-shadow: 0 8px 32px oklch(0.5 0.10 210 / 0.30); }

.cta-arrow-icon {
  width: 1rem;
  height: 1rem;
  transition: transform 0.3s ease;
  flex-shrink: 0;
}

.major-cta.is-active .cta-arrow-icon {
  transform: translateX(3px);
}

/* 背景装饰图标 */
.major-card-bg-icon {
  position: absolute;
  bottom: -0.5rem;
  right: -0.5rem;
  width: 160px;
  height: 160px;
  opacity: 0.1;
  pointer-events: none;
  z-index: 1;
  transition: opacity 0.4s ease;
}

.major-card:hover .major-card-bg-icon {
  opacity: 0.2;
}

.bg-icon-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

/* ============================================
   底部统计 — 左右排版
   ============================================ */
.majors-stats {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 2rem;
}

.stats-group {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.stats-item {
  display: flex;
  flex-direction: column;
  gap: 0.1rem;
}

.stats-value {
  font-size: 1.5rem;
  font-weight: 800;
  color: #0f172a;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
  letter-spacing: -0.02em;
  line-height: 1;
}

.stats-label {
  font-size: 0.72rem;
  font-weight: 500;
  color: #64748b;
  letter-spacing: 0.04em;
}

.stats-sep {
  font-size: 1.1rem;
  color: #cbd5e1;
  font-weight: 300;
  line-height: 1;
}

.stats-badge {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  padding: 0.6rem 1.25rem 0.6rem 1rem;
  background: linear-gradient(135deg, oklch(0.97 0.01 265) 0%, oklch(0.98 0.008 255) 100%);
  border: 1px solid oklch(0.82 0.04 260 / 0.45);
  border-radius: 9999px;
}

.stats-badge-icon {
  width: 1.15rem;
  height: 1.15rem;
  color: #1466ff;
  flex-shrink: 0;
}

.stats-badge-text {
  display: flex;
  flex-direction: column;
  gap: 0;
  line-height: 1.2;
}

.stats-badge-main {
  font-size: 0.95rem;
  font-weight: 700;
  color: #0f172a;
  letter-spacing: 0.05em;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
}

.stats-badge-sub {
  font-size: 0.68rem;
  font-weight: 500;
  color: #1466ff;
  letter-spacing: 0.08em;
  font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif;
}

/* ============================================
   响应式
   ============================================ */
@media (max-width: 1024px) {
  .majors-grid {
    gap: 1.25rem;
  }

  .major-card-body {
    padding: 1.25rem 1.25rem 1.25rem;
  }

  .major-theme {
    font-size: clamp(1.5rem, 3.5vw, 2rem);
  }

  .major-card-bg-icon {
    width: 120px;
    height: 120px;
  }
}

@media (max-width: 768px) {
  .majors-grid {
    grid-template-columns: 1fr;
    gap: 1rem;
    max-width: 420px;
    margin-left: auto;
    margin-right: auto;
  }

  .majors-header {
    padding: 1.5rem 0 3.5rem;
    background-size: contain;
  }

  .majors-title {
    font-size: clamp(1.5rem, 6vw, 2rem);
    padding: 0.5rem 2rem;
  }

  .majors-lead {
    font-size: 0.8rem;
    text-align: center;
    padding: 0 1rem;
    margin-top: 1.5rem;
  }

  .majors-stats {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .stats-group {
    gap: 0.4rem;
  }

  .stats-value {
    font-size: 1rem;
  }

  .stats-label {
    font-size: 0.62rem;
  }

  .stats-sep {
    font-size: 0.7rem;
    margin: 0 0.1rem;
  }

  .stats-badge {
    padding: 0.35rem 0.75rem 0.35rem 0.6rem;
    gap: 0.35rem;
  }

  .stats-badge-icon {
    width: 0.8rem;
    height: 0.8rem;
  }

  .stats-badge-main {
    font-size: 0.72rem;
  }

  .stats-badge-sub {
    font-size: 0.55rem;
  }
}

@media (max-width: 640px) {
  .majors-section {
    padding: 3rem 0 2.5rem;
  }

  .majors-header {
    padding: 1rem 0 2.5rem;
    margin-bottom: 2rem;
    background-size: 80% auto;
  }

  .majors-eyebrow {
    font-size: 0.62rem;
    letter-spacing: 0.16em;
  }

  .majors-title {
    font-size: clamp(1.35rem, 7vw, 1.8rem);
    padding: 0.4rem 1.5rem;
  }

  .majors-lead {
    font-size: 0.75rem;
    margin-top: 1rem;
  }

  .majors-stats {
    flex-direction: row;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    flex-wrap: nowrap;
  }

  .stats-group {
    gap: 0.25rem;
    flex-wrap: nowrap;
  }

  .stats-item {
    gap: 0;
  }

  .stats-value {
    font-size: 0.9rem;
  }

  .stats-label {
    font-size: 0.55rem;
    letter-spacing: 0.02em;
  }

  .stats-sep {
    font-size: 0.6rem;
    margin: 0 0.05rem;
  }

  .stats-badge {
    padding: 0.25rem 0.5rem 0.25rem 0.4rem;
    gap: 0.2rem;
    flex-shrink: 0;
  }

  .stats-badge-icon {
    width: 0.65rem;
    height: 0.65rem;
  }

  .stats-badge-text {
    gap: 0;
    line-height: 1.1;
  }

  .stats-badge-main {
    font-size: 0.62rem;
  }

  .stats-badge-sub {
    font-size: 0.48rem;
  }

  .major-card-body {
    padding: 0.875rem;
  }

  .major-theme {
    font-size: clamp(1.3rem, 6vw, 1.6rem);
  }

  .major-desc {
    font-size: 0.8rem;
    margin-bottom: 1.25rem;
  }

  .major-cta {
    padding: 0.5rem 1.25rem;
    font-size: 0.8rem;
  }

  .major-card-bg-icon {
    width: 100px;
    height: 100px;
  }
}

@media (max-width: 480px) {
  .majors-section {
    padding: 2.5rem 0 2rem;
  }

  .majors-header {
    padding: 0.75rem 0 2rem;
    margin-bottom: 1.5rem;
  }

  .majors-title {
    font-size: 1.25rem;
    padding: 0.35rem 1.25rem;
  }

  .majors-grid {
    max-width: 100%;
    gap: 0.75rem;
  }

  .major-theme {
    font-size: 1.2rem;
  }

  .major-subtitle {
    font-size: 0.85rem;
  }

  .stats-badge {
    padding: 0.4rem 0.75rem;
  }
}
</style>
