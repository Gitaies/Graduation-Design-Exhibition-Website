<template>
  <section id="about" class="about-section">
    <div class="about-container">

      <!-- ======== 区域标题：左对齐杂志风 ======== -->
      <div class="about-header">
        <span class="about-eyebrow">About the Exhibition</span>
        <h2 class="about-title">关于毕展</h2>
        <div class="about-header-divider" aria-hidden="true"></div>
        <p class="about-header-sub">传媒技术学院 2026 届毕业设计展以<em>「智媒融合·创启未来」</em>为主题，聚焦三大专业的创新成果。</p>
      </div>

      <!-- ======== 展览介绍：非对称双栏 ======== -->
      <div class="about-intro">
        <div class="about-intro-ornament" aria-hidden="true">
          <span class="ornament-year">2026</span>
        </div>
        <div class="about-intro-body">
          <p class="about-intro-lead">
            本次展览汇集 <strong>30+</strong> 优秀毕业设计作品，涵盖人工智能、大数据、物联网、虚拟现实等前沿技术领域，展现 <strong>180+</strong> 毕业生的创新思维与实践能力。
          </p>
          <p class="about-intro-text">
            通过本次展览，我们呈现传媒技术学院在人才培养与科研创新方面的成果，为学生提供展示自我、交流学习的平台，推动传媒技术的创新发展与产业融合。
          </p>
        </div>
      </div>

      <!-- ======== 数据条：大数字冲击 ======== -->
      <div class="about-stats">
        <div class="about-stat">
          <span class="about-stat-num">3</span>
          <span class="about-stat-label">大专业</span>
        </div>
        <div class="about-stat-divider" aria-hidden="true"></div>
        <div class="about-stat">
          <span class="about-stat-num">30<span class="about-stat-plus">+</span></span>
          <span class="about-stat-label">优秀作品</span>
        </div>
        <div class="about-stat-divider" aria-hidden="true"></div>
        <div class="about-stat">
          <span class="about-stat-num">180<span class="about-stat-plus">+</span></span>
          <span class="about-stat-label">毕业生</span>
        </div>
      </div>

      <!-- ======== 三大专业：编号卡片（非均等网格） ======== -->
      <div class="about-majors">
        <div
          v-for="(major, idx) in aboutMajors"
          :key="major.code"
          :class="['about-major-card', `about-major-card--${idx}`]"
        >
          <span class="about-major-num" aria-hidden="true">{{ major.number }}</span>
          <div class="about-major-content">
            <h3 class="about-major-name">{{ major.name }}</h3>
            <p class="about-major-theme">{{ major.theme }}</p>
            <p class="about-major-desc">{{ major.description }}</p>
          </div>
          <div class="about-major-line" aria-hidden="true"></div>
        </div>
      </div>

      <!-- ======== 学院简介 + 联系方式：自由排版 ======== -->
      <div class="about-info-split">
        <!-- 学院简介 -->
        <div class="about-college">
          <h3 class="about-college-heading">学院简介</h3>
          <p class="about-college-lead">
            传媒技术学院致力于培养具有<em>创新精神</em>和<em>实践能力</em>的高素质应用型人才。
          </p>
          <p class="about-college-text">
            在人工智能、大数据、物联网、虚拟现实等领域形成了鲜明的办学特色。学院拥有一支高水平的师资队伍，建有先进的实验室和实训基地，与多家知名企业建立了深度合作关系，为学生提供了良好的学习和实践环境。
          </p>
        </div>

        <!-- 联系方式 -->
        <div class="about-contact">
          <h3 class="about-contact-heading">联系我们</h3>
          <dl class="about-contact-list">
            <div class="about-contact-item">
              <dt>主办方</dt>
              <dd>武汉传媒学院传媒技术学院</dd>
            </div>
            <div class="about-contact-item">
              <dt>电话</dt>
              <dd>027-81979023</dd>
            </div>
            <div class="about-contact-item">
              <dt>官网</dt>
              <dd>www.whmc.edu.cn</dd>
            </div>
          </dl>
        </div>
      </div>

    </div>
  </section>
</template>

<script setup lang="ts">
import { aboutMajors } from '~/constants/majors'
</script>

<style scoped>
/* ============================================
   关于毕展 — 杂志编辑风
   ============================================ */
.about-section {
  padding: 6rem 0 3rem;
  background-color: oklch(0.975 0.002 250);
  background-image: url('/images/bga.jpg');
  background-size: 100% auto;
  background-position: bottom center;
  background-repeat: no-repeat;
}

.about-container {
  max-width: 95rem;
  margin: 0 auto;
  padding: 0 1rem;
}
@media (min-width: 1024px) {
  .about-container {
    padding: 0 2rem;
  }
}

/* --- 区域标题：左对齐 + 装饰线 --- */
.about-header {
  margin-bottom: clamp(3rem, 6vw, 5rem);
  max-width: 42rem;
}

.about-eyebrow {
  display: block;
  font-size: 0.6875rem;
  font-weight: 700;
  letter-spacing: 0.22em;
  text-transform: uppercase;
  color: oklch(0.55 0.04 255);
  margin-bottom: 0.75rem;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
}

.about-title {
  font-size: clamp(2.25rem, 5vw, 3.75rem);
  font-weight: 900;
  color: oklch(0.18 0.008 255);
  letter-spacing: -0.025em;
  line-height: 1.05;
  margin-bottom: 1rem;
}

.about-header-divider {
  width: clamp(3rem, 8vw, 6rem);
  height: 3px;
  background: oklch(0.55 0.16 265);
  border-radius: 2px;
  margin-bottom: 1.25rem;
}

.about-header-sub {
  font-size: clamp(0.95rem, 1.5vw, 1.1rem);
  color: oklch(0.38 0.015 255);
  line-height: 1.65;
  max-width: 36rem;
}

.about-header-sub em {
  font-style: normal;
  font-weight: 700;
  color: oklch(0.22 0.04 260);
}

/* --- 展览介绍：非对称双栏 --- */
.about-intro {
  display: grid;
  grid-template-columns: 1fr 1.4fr;
  gap: clamp(2rem, 6vw, 4rem);
  align-items: center;
  margin-bottom: clamp(4rem, 7vw, 6rem);
}

.about-intro-ornament {
  display: flex;
  align-items: center;
  justify-content: center;
}

.ornament-year {
  font-size: clamp(6rem, 12vw, 10rem);
  font-weight: 900;
  color: oklch(0.92 0.012 255);
  letter-spacing: -0.04em;
  line-height: 0.85;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
  user-select: none;
}

.about-intro-lead {
  font-size: clamp(1.05rem, 1.6vw, 1.2rem);
  color: oklch(0.25 0.012 255);
  line-height: 1.7;
  margin-bottom: 1.25rem;
}

.about-intro-lead strong {
  font-weight: 800;
  color: oklch(0.45 0.16 265);
}

.about-intro-text {
  font-size: 0.95rem;
  color: oklch(0.42 0.01 255);
  line-height: 1.75;
  max-width: 34rem;
}

/* --- 数据条 --- */
.about-stats {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: clamp(1.5rem, 5vw, 4rem);
  padding: clamp(2rem, 5vw, 3.5rem) 0;
  margin-bottom: clamp(4rem, 7vw, 6rem);
  border-top: 1px solid oklch(0.88 0.008 255);
  border-bottom: 1px solid oklch(0.88 0.008 255);
}

.about-stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.35rem;
}

.about-stat-num {
  font-size: clamp(2.5rem, 6vw, 4rem);
  font-weight: 900;
  color: oklch(0.18 0.008 255);
  letter-spacing: -0.03em;
  line-height: 1;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
}

.about-stat-plus {
  color: oklch(0.48 0.16 265);
  font-size: 0.6em;
  vertical-align: super;
}

.about-stat-label {
  font-size: 0.825rem;
  font-weight: 500;
  color: oklch(0.45 0.02 255);
  letter-spacing: 0.03em;
}

.about-stat-divider {
  width: 1px;
  height: 3rem;
  background: oklch(0.85 0.008 255);
}

/* --- 三大专业：非均等卡片 --- */
.about-majors {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: clamp(0.75rem, 1.5vw, 1.25rem);
  margin-bottom: clamp(4rem, 7vw, 6rem);
}

.about-major-card {
  position: relative;
  padding: clamp(1.5rem, 3vw, 2.5rem);
  border-radius: 0.75rem;
  background: white;
  border: 1px solid oklch(0.9 0.006 255);
  transition: transform 0.3s cubic-bezier(0.34, 1.56, 0.64, 1),
              box-shadow 0.3s ease;
}

.about-major-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 16px 40px oklch(0.6 0.04 255 / 0.12);
}

.about-major-num {
  display: block;
  font-size: 3.5rem;
  font-weight: 900;
  color: oklch(0.92 0.015 255);
  letter-spacing: -0.03em;
  line-height: 1;
  margin-bottom: 0.75rem;
  font-family: 'Inter', 'Geist', -apple-system, sans-serif;
}

.about-major-name {
  font-size: 1.25rem;
  font-weight: 800;
  color: oklch(0.18 0.008 255);
  margin-bottom: 0.35rem;
  letter-spacing: -0.01em;
}

.about-major-theme {
  font-size: 0.85rem;
  font-weight: 600;
  color: oklch(0.48 0.16 265);
  margin-bottom: 0.75rem;
}

.about-major-desc {
  font-size: 0.85rem;
  color: oklch(0.42 0.01 255);
  line-height: 1.65;
}

.about-major-line {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: oklch(0.55 0.16 265);
  border-radius: 0 0 4px 4px;
  transform: scaleX(0);
  transform-origin: left;
  transition: transform 0.4s cubic-bezier(0.22, 1, 0.36, 1);
}

.about-major-card:hover .about-major-line {
  transform: scaleX(1);
}

/* 卡片变体：第二张略微上移，打破单调 */
.about-major-card--1 {
  transform: translateY(-0.75rem);
}
.about-major-card--1:hover {
  transform: translateY(calc(-0.75rem - 4px));
}

/* --- 学院简介 + 联系方式：自由排版（无框） --- */
.about-info-split {
  display: grid;
  grid-template-columns: 1.3fr 0.7fr;
  gap: clamp(2.5rem, 7vw, 5rem);
  margin-bottom: clamp(3rem, 6vw, 5rem);
  padding-top: clamp(2rem, 4vw, 3rem);
  border-top: 1px solid oklch(0.88 0.008 255);
}

/* -- 学院简介 -- */
.about-college-heading {
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: oklch(0.5 0.04 255);
  margin-bottom: 1.25rem;
}

.about-college-lead {
  font-size: clamp(1.15rem, 1.8vw, 1.4rem);
  font-weight: 500;
  color: oklch(0.2 0.01 255);
  line-height: 1.6;
  letter-spacing: -0.005em;
  margin-bottom: 1rem;
  max-width: 32rem;
}

.about-college-lead em {
  font-style: normal;
  font-weight: 700;
  color: oklch(0.45 0.16 265);
}

.about-college-text {
  font-size: 0.9rem;
  color: oklch(0.42 0.01 255);
  line-height: 1.8;
  max-width: 34rem;
}

/* -- 联系方式 -- */
.about-contact-heading {
  font-size: 0.75rem;
  font-weight: 700;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  color: oklch(0.5 0.04 255);
  margin-bottom: 1.25rem;
}

.about-contact-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.about-contact-item {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
}

.about-contact-item dt {
  font-size: 0.725rem;
  font-weight: 600;
  color: oklch(0.5 0.03 255);
  letter-spacing: 0.06em;
  min-width: 2.5rem;
}

.about-contact-item dt::after {
  content: '';
}

.about-contact-item dd {
  font-size: 0.9rem;
  font-weight: 500;
  color: oklch(0.22 0.01 255);
  margin: 0;
}

/* --- 响应式 --- */
@media (max-width: 900px) {
  .about-intro {
    grid-template-columns: 1fr;
  }

  .about-intro-ornament {
    display: none;
  }

  .about-majors {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .about-major-card--1 {
    transform: none;
  }

  .about-major-card--1:hover {
    transform: translateY(-4px);
  }

  .about-info-split {
    grid-template-columns: 1fr;
    gap: 2.5rem;
  }

  .about-stats {
    gap: 1.25rem;
    flex-wrap: wrap;
  }

  .about-stat-divider {
    display: none;
  }
}

@media (max-width: 640px) {
  .about-section {
    padding: 4rem 0 2rem;
  }

  .about-contact-list {
    grid-template-columns: 1fr;
  }

  .about-stats {
    gap: 1rem;
    padding: 1.5rem 0;
  }

  .about-stat-num {
    font-size: 2rem;
  }

  .about-stat-label {
    font-size: 0.75rem;
  }
}
</style>
