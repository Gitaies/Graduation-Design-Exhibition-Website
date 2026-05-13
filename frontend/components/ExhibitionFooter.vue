<template>
  <footer ref="footerRoot" class="site-footer">
    <!-- Accent rule at top -->
    <div class="footer-accent" aria-hidden="true"></div>

    <div class="footer-container">
      <div class="footer-grid">
        <!-- Identity block -->
        <div class="footer-identity">
          <div class="footer-year" aria-hidden="true">2026</div>
          <h2 class="footer-college">传媒技术学院</h2>
          <p class="footer-exhibition">2026 届毕业设计展</p>
          <p class="footer-theme">智媒融合 · 创启未来</p>
        </div>

        <!-- Majors -->
        <nav class="footer-majors" aria-label="专业方向">
          <p class="footer-label">专业方向 <span class="footer-label-en">/ MAJORS</span></p>
          <ul class="footer-major-list">
            <li class="footer-major-item">
              <span class="footer-major-name">软件工程</span>
              <span class="footer-major-theme">数智焕新</span>
            </li>
            <li class="footer-major-item">
              <span class="footer-major-name">电子信息工程</span>
              <span class="footer-major-theme">芯火智造</span>
            </li>
            <li class="footer-major-item">
              <span class="footer-major-name">广播电视工程</span>
              <span class="footer-major-theme">虚实共生</span>
            </li>
          </ul>
        </nav>

        <!-- Contact -->
        <div class="footer-contact">
          <p class="footer-label">联系我们 <span class="footer-label-en">/ CONTACT</span></p>
          <p class="footer-info-item">主办方：武汉传媒学院传媒技术学院</p>
          <p class="footer-info-item">联系方式：027-81979023</p>
        </div>
      </div>

      <!-- Bottom bar -->
      <div class="footer-bottom">
        <p class="footer-copy">©2026 传媒技术学院 毕业作品设计展 版权所有</p>
        <p class="footer-rights">未经授权 禁止转载与商用</p>
      </div>
    </div>

    <!-- Subtle dot-grid pattern overlay -->
    <div class="footer-pattern" aria-hidden="true"></div>

    <!-- Corner geometric decoration -->
    <svg class="footer-geo" viewBox="0 0 120 120" fill="none" aria-hidden="true">
      <rect x="4" y="4" width="36" height="36" rx="3" stroke="currentColor" stroke-width="1.2" opacity="0.18" />
      <rect x="12" y="12" width="36" height="36" rx="3" stroke="currentColor" stroke-width="1.2" opacity="0.12" />
      <line x1="48" y1="22" x2="78" y2="22" stroke="currentColor" stroke-width="1" opacity="0.14" />
      <line x1="48" y1="40" x2="66" y2="40" stroke="currentColor" stroke-width="1" opacity="0.10" />
      <circle cx="96" cy="18" r="2.5" stroke="currentColor" stroke-width="1" opacity="0.15" />
      <circle cx="102" cy="30" r="1.5" fill="currentColor" opacity="0.10" />
    </svg>
  </footer>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { gsap } from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'

const footerRoot = ref<HTMLElement | null>(null)

onMounted(() => {
  gsap.registerPlugin(ScrollTrigger)

  if (!footerRoot.value) return

  const ctx = gsap.context(() => {
    const tl = gsap.timeline({
      scrollTrigger: {
        trigger: footerRoot.value,
        start: 'top bottom',
        toggleActions: 'play none none none',
      },
    })

    tl.from('.footer-accent', {
      scaleX: 0,
      transformOrigin: 'left center',
      duration: 0.8,
      ease: 'expo.out',
    })
    .from('.footer-year', {
      y: 40,
      opacity: 0,
      duration: 0.7,
      ease: 'expo.out',
    }, '-=0.4')
    .from('.footer-college, .footer-exhibition, .footer-theme', {
      y: 24,
      opacity: 0,
      duration: 0.6,
      stagger: 0.08,
      ease: 'expo.out',
    }, '-=0.3')
    .from('.footer-label', {
      y: 16,
      opacity: 0,
      duration: 0.5,
      stagger: 0.06,
      ease: 'power3.out',
    }, '-=0.2')
    .from('.footer-major-item, .footer-info-item', {
      y: 12,
      opacity: 0,
      duration: 0.45,
      stagger: 0.05,
      ease: 'power3.out',
    }, '-=0.15')
    .from('.footer-bottom', {
      opacity: 0,
      duration: 0.5,
      ease: 'power2.out',
    }, '-=0.1')
    .from('.footer-geo', {
      opacity: 0,
      scale: 0.85,
      duration: 0.7,
      ease: 'expo.out',
    }, '-=0.3')
  }, footerRoot.value)

  onUnmounted(() => {
    ctx.revert()
  })
})
</script>

<style scoped>
/* ============================================
   Footer — Architectural Editorial
   ============================================ */

.site-footer {
  position: relative;
  background-color: oklch(0.985 0.002 252);
  background-image: url('/images/ftbg1.png');
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  padding: 80px 0 32px;
  overflow: hidden;
  isolation: isolate;
}

/* ---- Accent rule ---- */
.footer-accent {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(
    90deg,
    oklch(0.52 0.22 265) 0%,
    oklch(0.65 0.18 235) 40%,
    transparent 100%
  );
  transform-origin: left center;
}

/* ---- Container ---- */
.footer-container {
  position: relative;
  z-index: 2;
  max-width: 95rem;
  margin: 0 auto;
  padding: 0 1rem;
}
@media (min-width: 1024px) {
  .footer-container {
    padding: 0 2rem;
  }
}

/* ---- Grid ---- */
.footer-grid {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: clamp(24px, 4vw, 48px);
  margin-bottom: 56px;
}

/* ---- Identity ---- */
.footer-identity {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.footer-year {
  font-size: clamp(5rem, 9vw, 7.5rem);
  font-weight: 200;
  line-height: 0.85;
  letter-spacing: -0.04em;
  color: oklch(0.82 0.015 255);
  margin-bottom: 12px;
  user-select: none;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
  font-variant-numeric: tabular-nums;
}

.footer-college {
  font-size: clamp(1.125rem, 2vw, 1.35rem);
  font-weight: 700;
  color: oklch(0.18 0.008 255);
  letter-spacing: 0.01em;
}

.footer-exhibition {
  font-size: 0.925rem;
  color: oklch(0.45 0.01 255);
  font-weight: 450;
}

.footer-theme {
  font-size: 0.875rem;
  color: oklch(0.52 0.18 260);
  font-weight: 500;
  margin-top: 4px;
}

/* ---- Labels ---- */
.footer-label {
  font-size: 0.75rem;
  font-weight: 700;
  color: oklch(0.35 0.008 255);
  letter-spacing: 0.06em;
  text-transform: uppercase;
  margin-bottom: 16px;
}

.footer-label-en {
  font-weight: 400;
  color: oklch(0.6 0.005 255);
  letter-spacing: 0.08em;
}

/* ---- Majors ---- */
.footer-major-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.footer-major-item {
  display: flex;
  align-items: baseline;
  gap: 8px;
}

.footer-major-name {
  font-size: 0.925rem;
  font-weight: 500;
  color: oklch(0.25 0.01 255);
}

.footer-major-theme {
  font-size: 0.775rem;
  color: oklch(0.55 0.04 260);
  font-weight: 450;
}

/* ---- Contact ---- */
.footer-contact {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.footer-info-item {
  font-size: 0.9rem;
  color: oklch(0.35 0.008 255);
}

/* ---- Bottom bar ---- */
.footer-bottom {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 20px;
  border-top: 1px solid oklch(0.88 0.006 255);
}

.footer-copy {
  font-size: 0.8rem;
  color: oklch(0.5 0.005 255);
}

.footer-rights {
  font-size: 0.75rem;
  font-weight: 500;
  color: oklch(0.55 0.005 255);
  letter-spacing: 0.04em;
}

/* ---- Dot grid pattern ---- */
.footer-pattern {
  position: absolute;
  inset: 0;
  z-index: 0;
  pointer-events: none;
  opacity: 0.35;
  background-image: radial-gradient(
    circle,
    oklch(0.6 0.04 255) 1px,
    transparent 1px
  );
  background-size: 28px 28px;
  mask-image: linear-gradient(
    180deg,
    transparent 0%,
    black 20%,
    black 80%,
    transparent 100%
  );
}

/* ---- Geometric corner decoration ---- */
.footer-geo {
  position: absolute;
  bottom: 18px;
  right: 4rem;
  width: 200px;
  height: 200px;
  z-index: 1;
  pointer-events: none;
  color: oklch(0.52 0.22 265);
}

/* ---- Responsive ---- */
@media (max-width: 1024px) {
  .footer-grid {
    gap: clamp(20px, 3vw, 32px);
  }

  .footer-geo {
    right: 1.5rem;
  }
}

@media (max-width: 768px) {
  .site-footer {
    padding: 40px 0 20px;
    background-size: cover;
  }

  .footer-grid {
    display: flex;
    flex-direction: column;
    gap: 0;
    margin-bottom: 24px;
  }

  /* 标识区 — 紧凑横排 */
  .footer-identity {
    display: flex;
    flex-direction: row;
    align-items: baseline;
    gap: 0.5rem;
    flex-wrap: wrap;
    padding-bottom: 16px;
    margin-bottom: 16px;
    border-bottom: 1px solid oklch(0.88 0.006 255);
  }

  .footer-year {
    font-size: 1.5rem;
    font-weight: 700;
    line-height: 1;
    color: oklch(0.55 0.20 265);
    margin-bottom: 0;
    margin-right: 0.25rem;
  }

  .footer-college {
    font-size: 0.95rem;
  }

  .footer-exhibition {
    font-size: 0.82rem;
  }

  .footer-theme {
    font-size: 0.78rem;
    margin-top: 0;
    margin-left: 0.25rem;
  }

  /* 专业 + 联系 — 紧凑横排 */
  .footer-majors,
  .footer-contact {
    padding: 0;
    margin-bottom: 12px;
  }

  .footer-label {
    font-size: 0.62rem;
    margin-bottom: 6px;
    letter-spacing: 0.08em;
  }

  .footer-label-en {
    display: none;
  }

  .footer-major-list {
    flex-direction: row;
    gap: 0;
    flex-wrap: wrap;
  }

  .footer-major-item {
    gap: 4px;
  }

  .footer-major-item:not(:last-child)::after {
    content: '/';
    color: oklch(0.7 0.01 255);
    margin: 0 6px;
    font-size: 0.7rem;
  }

  .footer-major-name {
    font-size: 0.78rem;
  }

  .footer-major-theme {
    display: none;
  }

  .footer-contact {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0;
    flex-wrap: wrap;
  }

  .footer-contact .footer-label {
    margin-bottom: 0;
    margin-right: 8px;
  }

  .footer-info-item {
    font-size: 0.75rem;
  }

  .footer-info-item:first-of-type {
    margin-right: 12px;
  }

  /* 底栏 — 单行 */
  .footer-bottom {
    flex-direction: row;
    justify-content: space-between;
    padding-top: 12px;
    gap: 4px;
  }

  .footer-copy {
    font-size: 0.68rem;
  }

  .footer-rights {
    font-size: 0.65rem;
  }

  /* 装饰 — 缩小 */
  .footer-geo {
    width: 60px;
    height: 60px;
    bottom: 8px;
    right: 4px;
    opacity: 0.5;
  }

  .footer-pattern {
    opacity: 0.2;
  }
}

@media (max-width: 480px) {
  .site-footer {
    padding: 28px 0 16px;
  }

  .footer-identity {
    padding-bottom: 12px;
    margin-bottom: 12px;
    gap: 0.35rem;
  }

  .footer-year {
    font-size: 1.25rem;
  }

  .footer-college {
    font-size: 0.85rem;
  }

  .footer-exhibition,
  .footer-theme {
    font-size: 0.7rem;
  }

  .footer-majors {
    margin-bottom: 8px;
  }

  .footer-major-name {
    font-size: 0.7rem;
  }

  .footer-info-item {
    font-size: 0.68rem;
  }

  .footer-bottom {
    flex-direction: column;
    gap: 2px;
    align-items: center;
    text-align: center;
    padding-top: 10px;
  }

  .footer-copy {
    font-size: 0.62rem;
  }

  .footer-rights {
    font-size: 0.6rem;
  }

  .footer-geo {
    display: none;
  }
}
</style>
