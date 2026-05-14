<template>
  <section id="about" class="about-section">
    <div class="about-container">

      <!-- ======== 区域标题 ======== -->
      <div class="about-header">
        <span class="about-eyebrow">About the Exhibition</span>
        <h2 class="about-title">关于毕展</h2>
        <div class="about-header-divider" aria-hidden="true"></div>
        <p class="about-header-sub">
          "智媒融合·创启未来" — 智能技术与传媒技术深度融合，跨专业协同创新，<br class="about-header-br" />以33件优秀毕业设计作品开启传媒技术人才培养的新篇章。
        </p>
      </div>

      <!-- ======== 主题诠释 ======== -->
      <div class="about-theme-interp">
        <div class="theme-interp-item">
          <span class="theme-interp-char">智媒</span>
          <p class="theme-interp-text">
            人工智能、大模型、物联网、虚拟现实等前沿技术正重塑传媒行业生态格局。三大专业虽技术路径各异，却共同指向<em>"智能赋能传媒"</em>的时代命题。
          </p>
        </div>
        <div class="theme-interp-item">
          <span class="theme-interp-char">融合</span>
          <p class="theme-interp-text">
            跨专业协同、技术与艺术交融、产业与教育对接。本届作品展是学院<em>"技术为基、传媒为魂"</em>办学理念的生动实践。
          </p>
        </div>
        <div class="theme-interp-item">
          <span class="theme-interp-char">创启</span>
          <p class="theme-interp-text">
            以创新开启未来。每一件作品都是学生面向真实问题、运用所学技术、提出创新方案的完整历练，承载着应用型高校服务社会的使命担当。
          </p>
        </div>
      </div>

      <!-- ======== 展览介绍 ======== -->
      <div class="about-intro">
        <div class="about-intro-ornament" aria-hidden="true">
          <span class="ornament-year">2026</span>
        </div>
        <div class="about-intro-body">
          <p class="about-intro-lead">
            本届毕业设计作品展汇聚传媒技术学院<strong>电子信息工程</strong>、<strong>广播电视工程</strong>、<strong>软件工程</strong>三个专业共 <strong>33 件</strong>优秀毕业设计作品，涵盖智能硬件系统、超高清视听技术、AI 应用平台等多个前沿方向。
          </p>
          <p class="about-intro-text">
            展览下设三个子主题板块，分别对应三大专业的技术特色与育人成果。作品选题紧密对接行业需求，涉及智慧家居、虚拟演播、AI 内容生成、健康医疗、交通分析等真实应用场景，充分展现我院"厚基础、重实践、强应用"的人才培养特色。
          </p>
        </div>
      </div>

      <!-- ======== 数据条 ======== -->
      <div class="about-stats">
        <div class="about-stat">
          <span class="about-stat-num">3</span>
          <span class="about-stat-label">大专业</span>
        </div>
        <div class="about-stat-divider" aria-hidden="true"></div>
        <div class="about-stat">
          <span class="about-stat-num">33</span>
          <span class="about-stat-label">优秀作品</span>
        </div>
        <div class="about-stat-divider" aria-hidden="true"></div>
        <div class="about-stat">
          <span class="about-stat-num">180<span class="about-stat-plus">+</span></span>
          <span class="about-stat-label">毕业生</span>
        </div>
      </div>

      <!-- ======== 三大专业卡片（悬停倾斜 + 点击展开） ======== -->
      <div class="about-majors">
        <div
          v-for="(major, idx) in aboutMajors"
          :key="major.code"
          :ref="el => setCardRef(el, major.code)"
          :class="['about-major-card', `about-major-card--${idx}`, { 'is-dimmed': expandedCode && expandedCode !== major.code }]"
          @mousemove="onCardMouseMove($event, major.code)"
          @mouseleave="onCardMouseLeave(major.code)"
          @click="expandCard(major.code)"
        >
          <div class="about-major-card-front" :style="tiltStyle(major.code)">
            <div class="card-front-top">
              <span class="card-front-num">{{ major.number }}</span>
              <span :class="['card-front-major-tag', `major-tag--${idx}`]">{{ major.name }}</span>
            </div>
            <h3 :class="['card-front-theme', `theme-color--${idx}`]">{{ major.theme }}</h3>
            <p class="card-front-subtitle">{{ major.subtitle }}</p>
            <p class="card-front-desc">{{ major.description }}</p>
            <span class="card-front-hint-text">点击查看详情</span>
          </div>
        </div>
      </div>

      <!-- ======== 三大亮点 ======== -->
      <div class="about-highlights">
        <div class="about-highlights-header">
          <span class="about-highlights-eyebrow">Exhibition Highlights</span>
          <h3 class="about-highlights-title">特色亮点</h3>
        </div>
        <div class="about-highlights-grid">
          <div class="about-highlight-card" v-for="hl in highlights" :key="hl.num">
            <span class="about-highlight-num">{{ hl.num }}</span>
            <div class="about-highlight-content">
              <h4 class="about-highlight-title">{{ hl.title }}</h4>
              <p class="about-highlight-desc">{{ hl.desc }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- ======== 学院简介 + 联系方式 ======== -->
      <div class="about-info-split">
        <div class="about-college">
          <h3 class="about-college-heading">学院简介</h3>
          <p class="about-college-lead">
            传媒技术学院致力于培养具有<em>创新精神</em>和<em>实践能力</em>的高素质应用型人才。
          </p>
          <p class="about-college-text">
            在人工智能、大数据、物联网、虚拟现实等领域形成了鲜明的办学特色。学院拥有一支高水平的师资队伍，建有先进的实验室和实训基地，与多家知名企业建立了深度合作关系，为学生提供了良好的学习和实践环境。
          </p>
        </div>

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

    <!-- ======== 展开浮层：Web Animations API FLIP ======== -->
    <Teleport to="body">
      <div
        v-if="expandedCode"
        class="card-expand-overlay"
        @click.self="closeCard"
      >
        <div ref="wrapperRef" class="card-expand-wrapper">
          <div ref="innerRef" class="card-expand-inner">
            <div class="card-expand-front">
              <div class="expand-front-top">
                <span class="expand-front-num">{{ expandedMajor?.number }}</span>
                <span :class="['expand-front-tag', `major-tag--${expandedIdx}`]">{{ expandedMajor?.name }}</span>
              </div>
              <h3 :class="['expand-front-theme', `theme-color--${expandedIdx}`]">{{ expandedMajor?.theme }}</h3>
              <p class="expand-front-subtitle">{{ expandedMajor?.subtitle }}</p>
              <p class="expand-front-desc">{{ expandedMajor?.description }}</p>
            </div>
            <div class="card-expand-back">
              <div class="expand-back-scroll">
                <div class="expand-back-header">
                  <span :class="['expand-back-theme-tag', `major-tag--${expandedIdx}`]">{{ expandedMajor?.theme }}</span>
                  <h3 class="expand-back-title">{{ expandedMajor?.name }}</h3>
                </div>
                <div class="expand-back-section">
                  <h4 class="expand-back-section-title">主题解读</h4>
                  <p class="expand-back-text">{{ backContent?.interpretation }}</p>
                </div>
                <div class="expand-back-section">
                  <h4 class="expand-back-section-title">专业作品特色</h4>
                  <p class="expand-back-text">{{ backContent?.characteristics }}</p>
                </div>
                <div class="expand-back-clusters">
                  <span v-for="c in backContent?.clusters ?? []" :key="c" :class="['expand-back-cluster-tag', `cluster-tag--${expandedIdx}`]">{{ c }}</span>
                </div>
                <p class="expand-back-motto">"{{ backContent?.motto }}"</p>
              </div>
            </div>
          </div>
          <button class="card-expand-close" @click.stop="closeCard" aria-label="返回">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.2" stroke-linecap="round"><path d="M19 12H5m7-7l-7 7 7 7"/></svg>
            <span>返回</span>
          </button>
        </div>
      </div>
    </Teleport>
  </section>
</template>

<script setup lang="ts">
import { ref, computed, reactive, nextTick } from 'vue'
import { gsap } from 'gsap'
import { aboutMajors } from '~/constants/majors'

// ============ 展开状态 ============
const expandedCode = ref<string | null>(null)
const isAnimating = ref(false)
const wrapperRef = ref<HTMLElement | null>(null)
const innerRef = ref<HTMLElement | null>(null)
let expandTl: gsap.core.Timeline | null = null

// ============ 悬停倾斜 ============
const tiltState = reactive<Record<string, { rx: number; ry: number }>>({})

function initTilt(code: string) { if (!tiltState[code]) tiltState[code] = { rx: 0, ry: 0 } }
function onCardMouseMove(e: MouseEvent, code: string) {
  initTilt(code)
  const el = e.currentTarget as HTMLElement
  const rect = el.getBoundingClientRect()
  tiltState[code].rx = -((e.clientY - rect.top) / rect.height - 0.5) * 8
  tiltState[code].ry = ((e.clientX - rect.left) / rect.width - 0.5) * 8
}
function onCardMouseLeave(code: string) {
  initTilt(code)
  tiltState[code].rx = 0
  tiltState[code].ry = 0
}
function tiltStyle(code: string) {
  initTilt(code)
  const { rx, ry } = tiltState[code]
  return {
    transform: `perspective(1200px) rotateX(${rx}deg) rotateY(${ry}deg)`,
    transition: rx === 0 && ry === 0 ? 'transform 0.5s cubic-bezier(0.22, 1, 0.36, 1)' : 'transform 0.1s ease-out'
  }
}

// ============ 卡片 ref ============
const cardRefs: Record<string, HTMLElement> = {}
function setCardRef(el: unknown, code: string) {
  if (el) cardRefs[code] = el as HTMLElement
}

// ============ 展开 — GSAP FLIP 动画 ============

function calcTarget() {
  const vw = window.innerWidth
  const vh = window.innerHeight
  const isMobile = vw < 768
  const w = isMobile ? Math.min(vw - 24, 400) : Math.min(vw * 0.88, 832)
  const h = isMobile ? Math.min(vh - 80, w * 1.4) : Math.min(vh * 0.85, w * 1.2)
  return { w, h, x: (vw - w) / 2, y: (vh - h) / 2 }
}

async function expandCard(code: string) {
  if (expandedCode.value || isAnimating.value) return

  const cardEl = cardRefs[code]
  if (!cardEl) return
  const sr = cardEl.getBoundingClientRect()

  expandedCode.value = code
  isAnimating.value = true
  await nextTick()
  // 等一帧让浏览器完成布局，避免移动端首帧卡顿
  await new Promise(r => requestAnimationFrame(r))

  const wrapper = wrapperRef.value
  const inner = innerRef.value
  if (!wrapper || !inner) { isAnimating.value = false; return }

  if (expandTl) expandTl.kill()

  const tgt = calcTarget()
  const sx = sr.width / tgt.w
  const sy = sr.height / tgt.h
  const dx = (sr.left + sr.width / 2) - (tgt.x + tgt.w / 2)
  const dy = (sr.top + sr.height / 2) - (tgt.y + tgt.h / 2)

  // 初始：定在目标位置，再用 transform 缩回到网格位置
  gsap.set(wrapper, {
    position: 'fixed',
    width: tgt.w,
    top: tgt.y,
    left: tgt.x,
    zIndex: 1001,
    overflow: 'hidden',
    pointerEvents: 'none',
    x: dx,
    y: dy,
    scaleX: sx,
    scaleY: sy,
    borderRadius: '1rem',
  })
  gsap.set(inner, { rotateY: 0 })

  // 覆盖层淡入
  const overlay = wrapper.parentElement!
  gsap.set(overlay, { opacity: 0 })
  gsap.to(overlay, { opacity: 1, duration: 0.35, ease: 'power2.out' })

  // 位移到中央 + 放大 + 翻转（0.05s 微延迟让 GPU 准备好）
  expandTl = gsap.timeline({
    onComplete: () => {
      gsap.set(wrapper, { overflow: 'visible', pointerEvents: 'auto' })
      isAnimating.value = false
    }
  })

  expandTl.to(wrapper, {
    x: 0, y: 0, scaleX: 1, scaleY: 1,
    duration: 0.85, ease: 'power3.inOut',
  }, 0.05)

  expandTl.to(inner, {
    rotateY: 180,
    duration: 0.75, ease: 'power2.inOut',
  }, 0.2)
}

async function closeCard() {
  if (isAnimating.value) return

  const wrapper = wrapperRef.value
  const inner = innerRef.value
  if (!wrapper || !inner) { expandedCode.value = null; return }

  const code = expandedCode.value
  if (!code) return
  const cardEl = cardRefs[code]
  if (!cardEl) { expandedCode.value = null; return }
  const sr = cardEl.getBoundingClientRect()

  isAnimating.value = true
  if (expandTl) expandTl.kill()

  const tgt = calcTarget()
  const sx = sr.width / tgt.w
  const sy = sr.height / tgt.h
  const dx = (sr.left + sr.width / 2) - (tgt.x + tgt.w / 2)
  const dy = (sr.top + sr.height / 2) - (tgt.y + tgt.h / 2)

  gsap.set(wrapper, { overflow: 'hidden', pointerEvents: 'none' })

  const closeTl = gsap.timeline({
    onComplete: () => {
      expandedCode.value = null
      isAnimating.value = false
    }
  })

  closeTl.to(inner, {
    rotateY: 0,
    duration: 0.55, ease: 'power2.inOut',
  }, 0)

  closeTl.to(wrapper, {
    x: dx, y: dy, scaleX: sx, scaleY: sy,
    duration: 0.7, ease: 'power3.inOut',
  }, 0.08)

  // 覆盖层淡出
  const overlay = wrapper.parentElement!
  gsap.to(overlay, { opacity: 0, duration: 0.35, ease: 'power2.in', delay: 0.2 })
}

// ============ 展开数据 ============
const expandedMajor = computed(() => expandedCode.value ? aboutMajors.find(m => m.code === expandedCode.value) ?? null : null)
const expandedIdx = computed(() => {
  const map: Record<string, number> = { software: 0, electronic: 1, broadcast: 2 }
  return expandedCode.value ? map[expandedCode.value] ?? 0 : 0
})

const backContentMap: Record<string, { interpretation: string; characteristics: string; clusters: string[]; motto: string }> = {
  software: {
    interpretation: '"数智焕新"揭示了软件专业以数据和智能驱动应用创新的核心路径。"数智驱动"——数据驱动与智能赋能，从底层的数据清洗、特征工程，到上层的模型训练、智能推理，学生完整经历"数据→信息→知识→智能"的价值转化链条。"焕新应用"——焕新体验、焕新效率、焕新价值，每一件作品都面向真实用户、解决真实问题、创造真实价值。',
    characteristics: '12件作品形成"AI原生应用—数据智能—行业服务平台—工具创新"四大矩阵。AI原生应用矩阵涵盖LangChain4j、MCP协议、RAG+DeepSeek、语音识别等前沿技术，站在大模型应用浪潮最前沿；数据智能矩阵展现从YOLO+大数据到随机森林+Spark的完整决策链路；行业服务平台矩阵以软件服务医疗、教育、社区等民生领域；工具创新矩阵聚焦效率提升与体验优化。作品特色在于"大模型落地、小场景深耕"——既有MCP协议、RAG等前沿技术探索，也有微信小程序、SpringBoot/Vue等成熟技术的扎实应用。',
    clusters: ['AI原生应用', '数据智能', '行业服务平台', '工具创新'],
    motto: '大模型落地，小场景深耕'
  },
  electronic: {
    interpretation: '"芯火智造"四字道尽电信专业的技术底色与工匠精神。"芯火相传"——以STM32、MSPM0等微控制器为核心，从底层驱动到上层应用，锤炼学生"从0到1"的硬件开发能力；"火"是薪火相传的传承感，从智能小车到气象站，从节能照明到快递柜控制，每一件作品都是技术火种在真实场景中的点燃与传递。"智造终端"——面向真实需求的完整终端系统解决方案，追光系统优化能源利用，人脸识别门禁提升安防效率，多功能手表整合健康监测与信息交互。',
    characteristics: '11件作品形成"智能控制—物联网—嵌入式人机交互"三大技术集群。智能控制集群：视觉伺服抓取系统、智能小车巡线、双车跟随控制、太阳能追光系统；物联网集群：气象数据采集与无线传输、智能节能照明、智能快递柜控制；嵌入式人机交互集群：智能桌面宠物机器人、多功能手表、旋转LED显示屏、人脸识别门禁。作品特色在于"小系统、真场景、硬实现"——从电路设计到代码编写，从结构搭建到调试优化，培养"能焊板子、会写代码、懂系统联调"的复合型工程能力。',
    clusters: ['智能控制', '物联网', '嵌入式人机交互'],
    motto: '小系统、真场景、硬实现'
  },
  broadcast: {
    interpretation: '"虚实共生"精准概括了广电专业在数字时代的技术方位与内容追求。"虚"——Unity 3D虚拟演播厅、UE5虚拟植入包装、智能背景合成、沉浸式音视频场馆，构建完全数字化的内容生产空间；"实"——超高清实况制作、卫星传输、图像增强、广播电视信号处理，扎根真实传媒产业的技术底座。共生意味着虚实并非对立，而是相互依存、彼此赋能、融合生长。',
    characteristics: '10件作品形成"超高清制播—虚拟制作—智能内容生成—传输覆盖"四大方向。超高清制播方向：多讯道超高清EFP视频系统、广播电视图像增强、历史影像高动态重建，对接行业4K/8K升级需求；虚拟制作方向：3D交互式虚拟演播厅、虚拟演播室背景合成、UE5晚会虚拟植入包装、沉浸式音视频场馆；智能内容生成方向：基于知识增强语言模型的海报自动化生成，探索AIGC在传媒领域的落地应用；传输覆盖方向：基于eNodeX30B平台的卫星广播电视传输系统、声控遥控系统。作品特色在于"传媒技术+内容艺术"的交叉创新，培养"懂技术、懂内容、懂流程"的传媒技术工程师。',
    clusters: ['超高清制播', '虚拟制作', '智能内容生成', '传输覆盖'],
    motto: '技术赋能内容，虚实重塑传播'
  }
}

const backContent = computed(() => expandedCode.value ? backContentMap[expandedCode.value] ?? null : null)

const highlights = [
  { num: '01', title: '技术前沿，紧跟产业脉搏', desc: '从 MCP 协议智能体、RAG 大模型应用，到 UE5 沉浸式场馆、超高清 EFP 系统，作品技术栈与行业最新趋势高度同步，体现"所学即所用"的应用型导向。' },
  { num: '02', title: '传媒底色，技术人文并重', desc: '区别于纯工科院校的毕业设计，我院作品始终保有"传媒基因"：智能系统服务智慧生活，视听技术深耕内容生产，软件应用关怀医疗、公益、校友服务等民生领域，技术有温度，创新有情怀。' },
  { num: '03', title: '软硬兼备，全链路覆盖', desc: '从底层嵌入式硬件、传感器数据采集，到中间层算法模型、信号处理，再到上层交互界面、平台系统，三大专业形成"感知—传输—处理—呈现—应用"的完整技术链条，展现学院协同育人的体系优势。' }
]
</script>

<style scoped>
/* ============================================
   关于毕展 — 杂志编辑风
   ============================================ */
.about-section {
  padding: 6rem 0 3rem;
  background-color: oklch(0.975 0.002 250);
  background-image: url('/images/bga.webp');
  background-size: 100% auto;
  background-position: bottom center;
  background-repeat: no-repeat;
}
.about-container { max-width: 95rem; margin: 0 auto; padding: 0 1rem; }
@media (min-width: 1024px) { .about-container { padding: 0 2rem; } }

.about-header { margin-bottom: clamp(3rem, 6vw, 5rem); max-width: 42rem; }
.about-eyebrow { display: block; font-size: 0.6875rem; font-weight: 700; letter-spacing: 0.22em; text-transform: uppercase; color: oklch(0.55 0.04 255); margin-bottom: 0.75rem; font-family: 'Inter', 'Geist', -apple-system, sans-serif; }
.about-title { font-size: clamp(2.25rem, 5vw, 3.75rem); font-weight: 900; color: oklch(0.18 0.008 255); letter-spacing: -0.025em; line-height: 1.05; margin-bottom: 1rem; }
.about-header-divider { width: clamp(3rem, 8vw, 6rem); height: 3px; background: oklch(0.55 0.16 265); border-radius: 2px; margin-bottom: 1.25rem; }
.about-header-sub { font-size: clamp(0.95rem, 1.5vw, 1.1rem); color: oklch(0.38 0.015 255); line-height: 1.75; max-width: 38rem; }
.about-header-br { display: none; }
@media (min-width: 640px) { .about-header-br { display: inline; } }

.about-theme-interp { display: grid; grid-template-columns: repeat(3, 1fr); gap: clamp(1.5rem, 3vw, 2.5rem); margin-bottom: clamp(4rem, 7vw, 6rem); padding: clamp(2rem, 4vw, 3rem); background: oklch(0.99 0.002 255); border-radius: 1rem; border: 1px solid oklch(0.9 0.006 255); }
.theme-interp-item { display: flex; flex-direction: column; gap: 0.75rem; }
.theme-interp-char { font-size: clamp(2rem, 3.5vw, 2.75rem); font-weight: 900; color: oklch(0.22 0.04 260); letter-spacing: 0.04em; line-height: 1; font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif; }
.theme-interp-text { font-size: 0.875rem; color: oklch(0.4 0.012 255); line-height: 1.8; }
.theme-interp-text em { font-style: normal; font-weight: 600; color: oklch(0.45 0.16 265); }

.about-intro { display: grid; grid-template-columns: 1fr 1.4fr; gap: clamp(2rem, 6vw, 4rem); align-items: center; margin-bottom: clamp(4rem, 7vw, 6rem); }
.about-intro-ornament { display: flex; align-items: center; justify-content: center; }
.ornament-year { font-size: clamp(6rem, 12vw, 10rem); font-weight: 900; color: oklch(0.92 0.012 255); letter-spacing: -0.04em; line-height: 0.85; font-family: 'Inter', 'Geist', -apple-system, sans-serif; user-select: none; }
.about-intro-lead { font-size: clamp(1.05rem, 1.6vw, 1.2rem); color: oklch(0.25 0.012 255); line-height: 1.7; margin-bottom: 1.25rem; }
.about-intro-lead strong { font-weight: 800; color: oklch(0.45 0.16 265); }
.about-intro-text { font-size: 0.95rem; color: oklch(0.42 0.01 255); line-height: 1.75; max-width: 34rem; }

.about-stats { display: flex; align-items: center; justify-content: center; gap: clamp(1.5rem, 5vw, 4rem); padding: clamp(2rem, 5vw, 3.5rem) 0; margin-bottom: clamp(4rem, 7vw, 6rem); border-top: 1px solid oklch(0.88 0.008 255); border-bottom: 1px solid oklch(0.88 0.008 255); }
.about-stat { display: flex; flex-direction: column; align-items: center; gap: 0.35rem; }
.about-stat-num { font-size: clamp(2.5rem, 6vw, 4rem); font-weight: 900; color: oklch(0.18 0.008 255); letter-spacing: -0.03em; line-height: 1; font-family: 'Inter', 'Geist', -apple-system, sans-serif; }
.about-stat-plus { color: oklch(0.48 0.16 265); font-size: 0.6em; vertical-align: super; }
.about-stat-label { font-size: 0.825rem; font-weight: 500; color: oklch(0.45 0.02 255); letter-spacing: 0.03em; }
.about-stat-divider { width: 1px; height: 3rem; background: oklch(0.85 0.008 255); }

/* ============ 网格卡片 ============ */
.about-majors { display: grid; grid-template-columns: repeat(3, 1fr); gap: clamp(1rem, 2vw, 1.5rem); margin-bottom: clamp(4rem, 7vw, 6rem); }
.about-major-card { display: flex; background: transparent; border: none; min-height: 24rem; cursor: pointer; transition: opacity 0.4s ease; }
/* 三张卡片等高，不再偏移 */
.about-major-card.is-dimmed { opacity: 0.35; pointer-events: none; }

.about-major-card-front {
  background: oklch(0.995 0.002 255);
  border: 1px solid oklch(0.88 0.008 255);
  border-radius: 1rem;
  padding: clamp(1.5rem, 3vw, 2.25rem);
  display: flex; flex-direction: column;
  flex: 1;
  box-shadow: 0 1px 3px oklch(0.6 0.02 255 / 0.06);
}

.card-front-top { display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.25rem; }
.card-front-num { font-size: 0.85rem; font-weight: 800; color: oklch(0.75 0.02 255); letter-spacing: -0.02em; font-family: 'Inter', 'Geist', -apple-system, sans-serif; }
.card-front-major-tag { font-size: 0.7rem; font-weight: 700; padding: 0.2rem 0.7rem; border-radius: 9999px; letter-spacing: 0.06em; }
.major-tag--0 { color: #2563eb; background: #eff6ff; }
.major-tag--1 { color: #7c3aed; background: #f5f3ff; }
.major-tag--2 { color: #0891b2; background: #ecfeff; }

.card-front-theme { font-size: clamp(2.5rem, 5vw, 3.25rem); font-weight: 900; line-height: 1.08; letter-spacing: 0.03em; margin-bottom: 0.4rem; font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif; }
.theme-color--0 { color: oklch(0.45 0.18 265); }
.theme-color--1 { color: oklch(0.42 0.18 285); }
.theme-color--2 { color: oklch(0.48 0.11 210); }

.card-front-subtitle { font-size: 0.9rem; font-weight: 600; color: oklch(0.48 0.03 255); letter-spacing: 0.04em; margin-bottom: 1.1rem; padding-bottom: 1.1rem; border-bottom: 1px solid oklch(0.9 0.006 255); }
.card-front-desc { font-size: 0.825rem; color: oklch(0.42 0.01 255); line-height: 1.72; flex: 1; }
.card-front-hint-text {
  align-self: flex-end;
  font-size: 0.72rem; font-weight: 600;
  color: oklch(0.5 0.1 260);
  margin-top: 0.75rem; padding-bottom: 7px;
  opacity: 0;
  transition: opacity 0.35s ease;
  position: relative;
  cursor: pointer;
}
/* 触摸设备始终显示提示文字 */
@media (hover: none) and (pointer: coarse) {
  .card-front-hint-text {
    opacity: 0.7;
  }
}
/* 动态横线：中间展开 + 持续流光 */
.card-front-hint-text::after {
  content: '';
  position: absolute; bottom: 0; left: 50%;
  width: 0; height: 2px;
  border-radius: 2px;
  transform: translateX(-50%);
  background: linear-gradient(90deg,
    oklch(0.55 0.16 265) 0%,
    oklch(0.65 0.22 250) 30%,
    oklch(0.7 0.25 240) 50%,
    oklch(0.65 0.22 250) 70%,
    oklch(0.55 0.16 265) 100%
  );
  background-size: 300% 100%;
  transition: width 0.55s cubic-bezier(0.22, 1, 0.36, 1);
}
.about-major-card:hover .card-front-hint-text {
  opacity: 1;
}
.about-major-card:hover .card-front-hint-text::after {
  width: 100%;
  animation: hint-line-flow 1.6s linear infinite 0.55s;
}
@keyframes hint-line-flow {
  0%   { background-position: 300% 0; }
  100% { background-position: -300% 0; }
}

/* ============ 展开浮层 ============ */
.card-expand-overlay {
  position: fixed; inset: 0; z-index: 1000;
  background: oklch(0.12 0.005 255 / 0.55);
  backdrop-filter: blur(12px);
  display: flex; align-items: center; justify-content: center;
  padding: 2rem;
}
/* 移动端去掉 backdrop-filter，避免 GPU 卡顿 */
@media (max-width: 768px) {
  .card-expand-overlay {
    backdrop-filter: none;
    background: oklch(0.12 0.005 255 / 0.7);
  }
}

.card-expand-wrapper { will-change: transform; }

.card-expand-inner {
  position: relative; display: grid;
  transform-style: preserve-3d;
  min-height: 30rem;
  border-radius: 1.25rem;
}

.card-expand-front,
.card-expand-back {
  grid-area: 1 / 1;
  backface-visibility: hidden;
  border-radius: 1.25rem;
  background: oklch(0.995 0.002 255);
  border: 1px solid oklch(0.88 0.008 255);
  box-shadow: 0 24px 80px oklch(0.3 0.02 255 / 0.18);
  overflow: hidden;
}

.card-expand-front {
  padding: clamp(2rem, 4vw, 3rem);
  display: flex; flex-direction: column; justify-content: center;
}

.expand-front-top { display: flex; align-items: center; justify-content: space-between; margin-bottom: 1.5rem; }
.expand-front-num { font-size: 1rem; font-weight: 800; color: oklch(0.72 0.02 255); letter-spacing: -0.02em; font-family: 'Inter', 'Geist', -apple-system, sans-serif; }
.expand-front-tag { font-size: 0.8rem; font-weight: 700; padding: 0.25rem 0.85rem; border-radius: 9999px; letter-spacing: 0.06em; }
.expand-front-theme { font-size: clamp(3rem, 7vw, 4.5rem); font-weight: 900; line-height: 1.06; letter-spacing: 0.04em; margin-bottom: 0.5rem; font-family: 'PingFang SC', 'Microsoft YaHei', 'Noto Sans SC', sans-serif; }
.expand-front-subtitle { font-size: 1.1rem; font-weight: 600; color: oklch(0.45 0.03 255); letter-spacing: 0.05em; margin-bottom: 1.5rem; padding-bottom: 1.5rem; border-bottom: 1px solid oklch(0.88 0.008 255); }
.expand-front-desc { font-size: 0.95rem; color: oklch(0.4 0.012 255); line-height: 1.78; }

.card-expand-back { transform: rotateY(180deg); }
.expand-back-scroll {
  padding: clamp(1.5rem, 3vw, 2.5rem);
  max-height: 80vh; overflow-y: auto; overscroll-behavior: contain;
  display: flex; flex-direction: column; align-items: center;
}
.expand-back-header { display: flex; align-items: center; gap: 0.75rem; margin-bottom: 2rem; padding-bottom: 1.25rem; border-bottom: 1px solid oklch(0.9 0.006 255); width: 100%; }
.expand-back-theme-tag { font-size: 0.85rem; font-weight: 700; padding: 0.25rem 0.9rem; border-radius: 9999px; letter-spacing: 0.05em; flex-shrink: 0; }
.expand-back-title { font-size: 1.35rem; font-weight: 800; color: oklch(0.18 0.008 255); letter-spacing: 0.02em; }
.expand-back-section { margin-bottom: 2rem; width: 100%; max-width: 75ch; }
.expand-back-section-title { font-size: 0.72rem; font-weight: 700; letter-spacing: 0.14em; text-transform: uppercase; color: oklch(0.5 0.04 255); margin-bottom: 0.75rem; }
.expand-back-text { font-size: 0.875rem; color: oklch(0.38 0.012 255); line-height: 1.85; }
.expand-back-clusters { display: flex; flex-wrap: wrap; justify-content: center; gap: 0.5rem; margin-bottom: 1.25rem; }
.expand-back-cluster-tag { font-size: 0.78rem; font-weight: 600; padding: 0.35rem 0.9rem; border-radius: 9999px; letter-spacing: 0.03em; }
.cluster-tag--0 { color: #2563eb; background: #eff6ff; }
.cluster-tag--1 { color: #7c3aed; background: #f5f3ff; }
.cluster-tag--2 { color: #0891b2; background: #ecfeff; }
.expand-back-motto { font-size: 0.95rem; font-weight: 600; color: oklch(0.3 0.025 255); letter-spacing: 0.05em; font-style: italic; text-align: center; padding-top: 1rem; border-top: 1px solid oklch(0.92 0.005 255); width: 100%; }

.card-expand-close {
  display: flex; align-items: center; gap: 0.4rem;
  margin: 1rem auto 0; padding: 0.55rem 1.4rem;
  background: oklch(0.97 0.003 255);
  border: 1px solid oklch(0.85 0.008 255);
  border-radius: 9999px;
  font-size: 0.82rem; font-weight: 600; color: oklch(0.4 0.02 255);
  cursor: pointer; transition: all 0.25s ease; letter-spacing: 0.03em;
}
.card-expand-close:hover { background: oklch(0.93 0.005 255); border-color: oklch(0.7 0.04 260); color: oklch(0.25 0.04 260); }

/* ============ 三大亮点 ============ */
.about-highlights { margin-bottom: clamp(4rem, 7vw, 6rem); }
.about-highlights-header { margin-bottom: clamp(2rem, 4vw, 3rem); }
.about-highlights-eyebrow { display: block; font-size: 0.6875rem; font-weight: 700; letter-spacing: 0.22em; text-transform: uppercase; color: oklch(0.55 0.04 255); margin-bottom: 0.5rem; font-family: 'Inter', 'Geist', -apple-system, sans-serif; }
.about-highlights-title { font-size: clamp(1.5rem, 2.5vw, 2rem); font-weight: 800; color: oklch(0.18 0.008 255); letter-spacing: -0.015em; }
.about-highlights-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: clamp(1rem, 2vw, 1.5rem); }
.about-highlight-card { display: flex; gap: 1.25rem; padding: clamp(1.5rem, 2.5vw, 2rem); background: white; border-radius: 0.75rem; border: 1px solid oklch(0.9 0.006 255); transition: box-shadow 0.3s ease; }
.about-highlight-card:hover { box-shadow: 0 8px 30px oklch(0.6 0.04 255 / 0.1); }
.about-highlight-num { font-size: 2rem; font-weight: 900; color: oklch(0.9 0.02 255); letter-spacing: -0.02em; line-height: 1; flex-shrink: 0; font-family: 'Inter', 'Geist', -apple-system, sans-serif; }
.about-highlight-content { display: flex; flex-direction: column; gap: 0.5rem; }
.about-highlight-title { font-size: 0.95rem; font-weight: 700; color: oklch(0.22 0.02 255); letter-spacing: 0.01em; }
.about-highlight-desc { font-size: 0.825rem; color: oklch(0.42 0.01 255); line-height: 1.7; }

.about-info-split { display: grid; grid-template-columns: 1.3fr 0.7fr; gap: clamp(2.5rem, 7vw, 5rem); margin-bottom: clamp(3rem, 6vw, 5rem); padding-top: clamp(2rem, 4vw, 3rem); border-top: 1px solid oklch(0.88 0.008 255); }
.about-college-heading { font-size: 0.75rem; font-weight: 700; letter-spacing: 0.18em; text-transform: uppercase; color: oklch(0.5 0.04 255); margin-bottom: 1.25rem; }
.about-college-lead { font-size: clamp(1.15rem, 1.8vw, 1.4rem); font-weight: 500; color: oklch(0.2 0.01 255); line-height: 1.6; letter-spacing: -0.005em; margin-bottom: 1rem; max-width: 32rem; }
.about-college-lead em { font-style: normal; font-weight: 700; color: oklch(0.45 0.16 265); }
.about-college-text { font-size: 0.9rem; color: oklch(0.42 0.01 255); line-height: 1.8; max-width: 34rem; }
.about-contact-heading { font-size: 0.75rem; font-weight: 700; letter-spacing: 0.18em; text-transform: uppercase; color: oklch(0.5 0.04 255); margin-bottom: 1.25rem; }
.about-contact-list { display: flex; flex-direction: column; gap: 1rem; }
.about-contact-item { display: flex; align-items: baseline; gap: 0.5rem; }
.about-contact-item dt { font-size: 0.725rem; font-weight: 600; color: oklch(0.5 0.03 255); letter-spacing: 0.06em; min-width: 2.5rem; }
.about-contact-item dd { font-size: 0.9rem; font-weight: 500; color: oklch(0.22 0.01 255); margin: 0; }

/* ============ 响应式 ============ */
@media (max-width: 900px) {
  .about-theme-interp { grid-template-columns: 1fr; gap: 2rem; }
  .about-intro { grid-template-columns: 1fr; }
  .about-intro-ornament { display: none; }
  .about-majors { grid-template-columns: 1fr; gap: 1rem; }
  .about-major-card { min-height: auto; }
  .about-highlights-grid { grid-template-columns: 1fr; gap: 1rem; }
  .about-info-split { grid-template-columns: 1fr; gap: 2.5rem; }
  .about-stats { gap: 1.25rem; flex-wrap: wrap; }
  .about-stat-divider { display: none; }
  .card-expand-overlay { padding: 0.5rem; }
  .expand-back-scroll { max-height: 70vh; }
  .expand-front-theme { font-size: clamp(2rem, 6vw, 3.5rem); }
  .expand-front-subtitle { font-size: 0.95rem; }
  .expand-front-desc { font-size: 0.85rem; }
}

@media (max-width: 640px) {
  .about-section { padding: 4rem 0 2rem; }
  .about-theme-interp { padding: 1.5rem; border-radius: 0.75rem; }
  .about-highlight-card { flex-direction: column; gap: 0.5rem; }
  .about-stats { flex-direction: row; align-items: center; justify-content: space-around; gap: 0.25rem; padding: 1rem 0; flex-wrap: nowrap; }
  .about-stat { flex-direction: row; align-items: baseline; gap: 0.25rem; }
  .about-stat-num { font-size: 1.25rem; line-height: 1; }
  .about-stat-plus { font-size: 0.55em; }
  .about-stat-label { font-size: 0.65rem; white-space: nowrap; }
  .about-stat-divider { height: 1.5rem; }
  .card-expand-overlay { padding: 0; align-items: flex-end; }
  .card-expand-front { padding: 1.5rem; }
  .expand-front-theme { font-size: clamp(1.75rem, 7vw, 2.5rem); }
  .expand-front-subtitle { font-size: 0.85rem; }
  .expand-front-desc { font-size: 0.8rem; }
  .expand-back-scroll { padding: 1rem; max-height: 65vh; }
  .expand-back-section { margin-bottom: 1.25rem; }
  .expand-back-text { font-size: 0.8rem; line-height: 1.7; }
  .expand-back-clusters { gap: 0.35rem; }
  .expand-back-cluster-tag { font-size: 0.7rem; padding: 0.25rem 0.65rem; }
  .card-expand-close { margin: 0.75rem auto 0; padding: 0.65rem 1.5rem; }
}

.expand-back-scroll::-webkit-scrollbar { width: 4px; }
.expand-back-scroll::-webkit-scrollbar-track { background: transparent; }
.expand-back-scroll::-webkit-scrollbar-thumb { background: oklch(0.85 0.01 255); border-radius: 2px; }
</style>
