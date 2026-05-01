<template>
  <div class="home-page">
    <ExhibitionHeader />

    <!-- 主视觉区 -->
    <section id="hero" class="hero-section min-h-screen flex items-center justify-center relative overflow-hidden">
      <!-- 背景图片 -->
      <div class="absolute inset-0 hero-background"></div>

      <!-- 波浪粒子画布 -->
      <canvas ref="particleCanvas" class="absolute inset-0 z-[1]"></canvas>

      <!-- 装饰图标 -->
      <div class="absolute right-0 top-1/2 transform -translate-y-1/2 hero-icon-container">
        <img src="~/assets/images/hero/hero_icon.png" alt="装饰" class="hero-icon" />
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

    <!-- 专业展区 -->
    <section id="majors" class="py-20 bg-bg-page">
      <div class="container mx-auto px-4">
        <!-- 页面标题 -->
        <div class="text-center mb-16">
          <div class="inline-block px-4 py-1.5 bg-primary-blue/10 text-primary-blue rounded-full text-sm font-medium mb-4">
            PROFESSIONAL ZONES
          </div>
          <h2 class="text-4xl md:text-5xl font-bold mb-4 bg-gradient-to-r from-primary-blue to-primary-cyan bg-clip-text text-transparent">
            专业展区
          </h2>
          <p class="text-lg text-text-secondary">三大专业，无限创意</p>
        </div>

        <!-- 专业卡片 -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl mx-auto mb-20">
          <NuxtLink
            v-for="major in majorsData"
            :key="major.code"
            :to="`/major/${major.code}`"
            class="glass-card rounded-3xl p-8 hover:shadow-2xl transition-all transform hover:-translate-y-3 cursor-pointer block"
          >
            <div class="text-primary-blue text-5xl font-bold mb-4">{{ major.number }}</div>
            <h3 class="text-3xl font-bold mb-2">{{ major.name }}</h3>
            <h4 class="text-xl text-primary-blue font-semibold mb-3">{{ major.theme }}</h4>
            <p class="text-sm text-text-secondary mb-4">{{ major.subtitle }}</p>
            <p class="text-base text-text-secondary leading-relaxed mb-6">{{ major.description }}</p>
            <button class="w-full py-3 bg-primary-blue text-white rounded-full hover:bg-primary-blue-dark transition-all">
              探索作品
            </button>
          </NuxtLink>
        </div>

        <!-- 底部统计 -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-6 max-w-4xl mx-auto">
          <div class="text-center">
            <div class="text-4xl font-bold text-primary-blue mb-2">3</div>
            <div class="text-sm text-text-secondary">大专业</div>
          </div>
          <div class="text-center">
            <div class="text-4xl font-bold text-primary-blue mb-2">12+</div>
            <div class="text-sm text-text-secondary">优秀作品</div>
          </div>
          <div class="text-center">
            <div class="text-4xl font-bold text-primary-blue mb-2">300+</div>
            <div class="text-sm text-text-secondary">毕业生</div>
          </div>
          <div class="text-center">
            <div class="text-4xl font-bold text-primary-blue mb-2">∞</div>
            <div class="text-sm text-text-secondary">创意</div>
          </div>
        </div>
      </div>
    </section>

    <!-- 点赞热榜 -->
    <section id="ranking" class="py-20 bg-white">
      <div class="container mx-auto px-4">
        <!-- 页面标题 -->
        <div class="text-center mb-16">
          <div class="inline-block px-4 py-1.5 bg-primary-blue/10 text-primary-blue rounded-full text-sm font-medium mb-4">
            HOT RANKING
          </div>
          <h2 class="text-4xl md:text-5xl font-bold mb-4 bg-gradient-to-r from-primary-blue to-primary-cyan bg-clip-text text-transparent">
            点赞热榜
          </h2>
          <p class="text-lg text-text-secondary">最受欢迎的毕业设计作品排行榜</p>
        </div>

        <!-- 榜单切换 -->
        <div class="flex justify-center gap-4 mb-12">
          <button
            v-for="tab in tabs"
            :key="tab.value"
            @click="selectedRange = tab.value"
            :class="[
              'px-6 py-3 rounded-xl font-medium transition-all duration-200',
              selectedRange === tab.value
                ? 'bg-primary-blue text-white shadow-lg shadow-primary-blue/30'
                : 'bg-white/80 backdrop-blur-sm text-text-secondary hover:bg-white hover:text-primary-blue hover:shadow-md border border-gray-200'
            ]"
          >
            {{ tab.label }}
          </button>
        </div>

        <!-- 前三名大卡片 -->
        <div v-if="topThree.length > 0" class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12 max-w-6xl mx-auto">
          <div
            v-for="(item, index) in topThree"
            :key="item.work_id"
            :class="[
              'relative overflow-hidden rounded-2xl p-6 transition-all duration-300 hover:-translate-y-2',
              index === 0 ? 'md:col-span-3 bg-gradient-to-br from-yellow-400 to-orange-500 text-white shadow-2xl shadow-yellow-500/30' :
              index === 1 ? 'bg-gradient-to-br from-gray-300 to-gray-400 text-white shadow-xl' :
              'bg-gradient-to-br from-orange-300 to-orange-400 text-white shadow-xl'
            ]"
          >
            <!-- 排名徽章 -->
            <div class="absolute top-4 right-4">
              <div :class="[
                'w-16 h-16 rounded-full flex items-center justify-center text-2xl font-bold',
                index === 0 ? 'bg-white/20 backdrop-blur-sm' : 'bg-white/30 backdrop-blur-sm'
              ]">
                {{ index + 1 }}
              </div>
            </div>

            <!-- 作品信息 -->
            <div class="relative z-10">
              <div class="text-sm opacity-90 mb-2">{{ getWork(item.work_id)?.majorName }}</div>
              <h3 class="text-2xl font-bold mb-3 pr-20">
                {{ getWork(item.work_id)?.title }}
              </h3>
              <p class="text-sm opacity-90 mb-4 line-clamp-2">
                {{ getWork(item.work_id)?.introduction }}
              </p>
              <div class="flex items-center justify-between">
                <div class="flex items-center gap-2">
                  <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                  </svg>
                  <span class="text-2xl font-bold">{{ item.like_count }}</span>
                </div>
                <NuxtLink
                  :to="`/works/${item.work_id}`"
                  class="px-4 py-2 bg-white/20 backdrop-blur-sm hover:bg-white/30 rounded-lg font-medium transition-colors"
                >
                  查看详情
                </NuxtLink>
              </div>
            </div>
          </div>
        </div>

        <!-- 第4名及以后列表 -->
        <div v-if="restRankings.length > 0" class="space-y-4 max-w-4xl mx-auto">
          <NuxtLink
            v-for="(item, index) in restRankings"
            :key="item.work_id"
            :to="`/works/${item.work_id}`"
            class="block bg-white/80 backdrop-blur-sm rounded-xl p-5 hover:shadow-xl hover:-translate-y-1 transition-all duration-300 group border border-gray-200"
          >
            <div class="flex items-center gap-4">
              <!-- 排名 -->
              <div class="w-12 h-12 flex-shrink-0 bg-gradient-to-br from-gray-100 to-gray-200 rounded-xl flex items-center justify-center text-xl font-bold text-text-main">
                {{ index + 4 }}
              </div>

              <!-- 作品信息 -->
              <div class="flex-1 min-w-0">
                <div class="flex items-center gap-2 mb-1">
                  <span class="px-2 py-0.5 bg-primary-blue/10 text-primary-blue text-xs rounded">
                    {{ getWork(item.work_id)?.majorName }}
                  </span>
                </div>
                <h3 class="text-lg font-bold text-text-main mb-1 group-hover:text-primary-blue transition-colors truncate">
                  {{ getWork(item.work_id)?.title }}
                </h3>
                <p class="text-sm text-text-secondary line-clamp-1">
                  {{ getWork(item.work_id)?.introduction }}
                </p>
              </div>

              <!-- 点赞数 -->
              <div class="flex items-center gap-2 text-primary-blue">
                <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z" />
                </svg>
                <span class="text-xl font-bold">{{ item.like_count }}</span>
              </div>

              <!-- 箭头 -->
              <svg class="w-6 h-6 text-text-light group-hover:text-primary-blue group-hover:translate-x-1 transition-all" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
              </svg>
            </div>
          </NuxtLink>
        </div>

        <!-- 空状态 -->
        <div v-if="rankings.length === 0" class="text-center py-20">
          <div class="text-6xl mb-4">📊</div>
          <p class="text-xl text-text-secondary mb-2">暂无排行数据</p>
          <p class="text-text-light">快去给喜欢的作品点赞吧</p>
        </div>
      </div>
    </section>

    <!-- 关于毕展 -->
    <section id="about" class="py-20 bg-bg-page">
      <div class="container mx-auto px-4">
        <!-- 页面标题 -->
        <div class="text-center mb-16">
          <div class="inline-block px-4 py-1.5 bg-primary-blue/10 text-primary-blue rounded-full text-sm font-medium mb-4">
            ABOUT US
          </div>
          <h2 class="text-4xl md:text-5xl font-bold mb-4 bg-gradient-to-r from-primary-blue to-primary-cyan bg-clip-text text-transparent">
            关于毕展
          </h2>
          <p class="text-lg text-text-secondary">智媒融合 · 创启未来</p>
        </div>

        <div class="max-w-4xl mx-auto">
          <!-- 展览介绍 -->
          <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-8 md:p-12 shadow-sm mb-12">
            <h3 class="text-2xl md:text-3xl font-bold text-text-main mb-6">展览介绍</h3>
            <div class="space-y-4 text-text-secondary leading-relaxed">
              <p>
                传媒技术学院 2026 届毕业设计展以"智媒融合 · 创启未来"为主题，聚焦软件工程、电子信息工程、广播电视工程三大专业的创新成果与应用实践。
              </p>
              <p>
                本次展览汇集了 60+ 优秀毕业设计作品，涵盖人工智能、大数据、物联网、虚拟现实等前沿技术领域，展现了 300+ 毕业生的创新思维和实践能力。
              </p>
              <p>
                通过本次展览，我们希望展示传媒技术学院在人才培养、科研创新方面的成果，同时为学生提供一个展示自我、交流学习的平台，推动传媒技术的创新发展与产业融合。
              </p>
            </div>
          </div>

          <!-- 三大专业介绍 -->
          <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
            <div
              v-for="major in aboutMajors"
              :key="major.code"
              class="bg-white/80 backdrop-blur-sm rounded-2xl p-6 shadow-sm hover:shadow-xl transition-all duration-300 hover:-translate-y-2"
            >
              <div class="w-12 h-12 bg-gradient-to-br from-primary-blue to-primary-cyan rounded-xl flex items-center justify-center text-white text-xl font-bold mb-4">
                {{ major.number }}
              </div>
              <h3 class="text-xl font-bold text-text-main mb-2">{{ major.name }}</h3>
              <p class="text-primary-blue font-medium mb-3">{{ major.theme }}</p>
              <p class="text-sm text-text-secondary leading-relaxed">
                {{ major.description }}
              </p>
            </div>
          </div>

          <!-- 学院信息 -->
          <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-8 md:p-12 shadow-sm mb-12">
            <h3 class="text-2xl md:text-3xl font-bold text-text-main mb-6">学院简介</h3>
            <div class="space-y-4 text-text-secondary leading-relaxed">
              <p>
                传媒技术学院致力于培养具有创新精神和实践能力的高素质应用型人才，在人工智能、大数据、物联网、虚拟现实等领域形成了鲜明的办学特色。
              </p>
              <p>
                学院拥有一支高水平的师资队伍，建有先进的实验室和实训基地，与多家知名企业建立了深度合作关系，为学生提供了良好的学习和实践环境。
              </p>
            </div>
          </div>

          <!-- 联系方式 -->
          <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-8 md:p-12 shadow-sm">
            <h3 class="text-2xl md:text-3xl font-bold text-text-main mb-6">联系我们</h3>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div class="flex items-start gap-4">
                <div class="w-12 h-12 bg-primary-blue/10 rounded-xl flex items-center justify-center flex-shrink-0">
                  <svg class="w-6 h-6 text-primary-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                  </svg>
                </div>
                <div>
                  <div class="text-sm text-text-light mb-1">邮箱</div>
                  <div class="text-text-main font-medium">contact@example.com</div>
                </div>
              </div>

              <div class="flex items-start gap-4">
                <div class="w-12 h-12 bg-primary-blue/10 rounded-xl flex items-center justify-center flex-shrink-0">
                  <svg class="w-6 h-6 text-primary-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z" />
                  </svg>
                </div>
                <div>
                  <div class="text-sm text-text-light mb-1">电话</div>
                  <div class="text-text-main font-medium">010-12345678</div>
                </div>
              </div>

              <div class="flex items-start gap-4">
                <div class="w-12 h-12 bg-primary-blue/10 rounded-xl flex items-center justify-center flex-shrink-0">
                  <svg class="w-6 h-6 text-primary-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z" />
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z" />
                  </svg>
                </div>
                <div>
                  <div class="text-sm text-text-light mb-1">地址</div>
                  <div class="text-text-main font-medium">北京市海淀区学院路XX号</div>
                </div>
              </div>

              <div class="flex items-start gap-4">
                <div class="w-12 h-12 bg-primary-blue/10 rounded-xl flex items-center justify-center flex-shrink-0">
                  <svg class="w-6 h-6 text-primary-blue" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9" />
                  </svg>
                </div>
                <div>
                  <div class="text-sm text-text-light mb-1">官网</div>
                  <div class="text-text-main font-medium">www.example.com</div>
                </div>
              </div>
            </div>
          </div>

          <!-- 版权信息 -->
          <div class="mt-12 text-center text-text-secondary text-sm">
            <p>© 2026 传媒技术学院. All rights reserved.</p>
          </div>
        </div>
      </div>
    </section>

    <ExhibitionFooter />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed, watch } from 'vue'
import { gsap } from 'gsap'

const majors = [
  { code: 'software', number: '01', name: '软件工程', theme: '数智焕新' },
  { code: 'electronic', number: '02', name: '电子信息工程', theme: '芯火智造' },
  { code: 'broadcast', number: '03', name: '广播电视工程', theme: '虚实共生' }
]

// 专业展区数据
const majorsData = [
  {
    code: 'software',
    number: '01',
    name: '软件工程',
    theme: '数智焕新',
    subtitle: '数智驱动 · 焕新应用',
    description: '聚焦软件开发、人工智能与数据应用等方向的创新实践。'
  },
  {
    code: 'electronic',
    number: '02',
    name: '电子信息工程',
    theme: '芯火智造',
    subtitle: '芯火相传 · 智造终端',
    description: '聚焦嵌入式、物联网、通信与硬件系统的创新应用。'
  },
  {
    code: 'broadcast',
    number: '03',
    name: '广播电视工程',
    theme: '虚实共生',
    subtitle: '虚实共生 · 视界新生',
    description: '聚焦视音频制作、虚拟现实与数字媒体的融合创新。'
  }
]

// 关于毕展专业数据
const aboutMajors = [
  {
    code: 'software',
    number: '01',
    name: '软件工程',
    theme: '数智焕新',
    description: '聚焦软件开发、人工智能与数据应用等方向的创新实践。'
  },
  {
    code: 'electronic',
    number: '02',
    name: '电子信息工程',
    theme: '芯火智造',
    description: '聚焦嵌入式、物联网、通信与硬件系统的创新应用。'
  },
  {
    code: 'broadcast',
    number: '03',
    name: '广播电视工程',
    theme: '虚实共生',
    description: '聚焦视音频制作、虚拟现实与数字媒体的融合创新。'
  }
]

// 排行榜数据
interface RankingItem {
  rank: number
  work_id: string
  like_count: number
}

const tabs = [
  { label: '总榜', value: 'all' },
  { label: '本周热榜', value: 'week' },
  { label: '今日热榜', value: 'today' }
]

const selectedRange = ref('all')
const rankings = ref<RankingItem[]>([])

// 加载作品数据
const { data: worksData } = await useFetch<{ works: any[] }>('/data/works.json')
const allWorks = computed(() => worksData.value?.works || [])

const getWork = (workId: string) => {
  return allWorks.value.find((w: any) => w.id === workId)
}

const topThree = computed(() => rankings.value.slice(0, 3))
const restRankings = computed(() => rankings.value.slice(3))

// 加载排行榜数据
const loadRankings = async () => {
  try {
    await new Promise(resolve => setTimeout(resolve, 500))

    // 生成模拟排行榜
    const mockRankings: RankingItem[] = allWorks.value
      .map((work: any, index: number) => ({
        rank: index + 1,
        work_id: work.id,
        like_count: Math.floor(Math.random() * 1000) + 100
      }))
      .sort((a: RankingItem, b: RankingItem) => b.like_count - a.like_count)
      .slice(0, 10)

    rankings.value = mockRankings
  } catch (error) {
    console.error('加载排行榜失败:', error)
  }
}

watch(selectedRange, () => {
  loadRankings()
})

// 滚动到指定区域
const scrollToSection = (path: string) => {
  const sectionId = path.replace('#', '')
  const element = document.getElementById(sectionId)

  if (element) {
    const headerHeight = 80 // 导航栏高度
    const elementPosition = element.getBoundingClientRect().top
    const offsetPosition = elementPosition + window.pageYOffset - headerHeight

    window.scrollTo({
      top: offsetPosition,
      behavior: 'smooth'
    })
  }
}

const particleCanvas = ref<HTMLCanvasElement | null>(null)
let animationFrameId: number | null = null

// 粒子类
class Particle {
  x: number
  y: number
  baseY: number
  size: number
  speedX: number
  speedY: number
  opacity: number
  baseOpacity: number
  phase: number

  constructor(canvasWidth: number, canvasHeight: number) {
    this.x = Math.random() * canvasWidth
    this.y = Math.random() * canvasHeight
    this.baseY = this.y
    this.size = Math.random() * 2 + 1
    this.speedX = (Math.random() - 0.5) * 0.3
    this.speedY = (Math.random() - 0.5) * 0.2
    this.baseOpacity = Math.random() * 0.4 + 0.2
    this.opacity = this.baseOpacity
    this.phase = Math.random() * Math.PI * 2
  }

  update(time: number, canvasWidth: number, canvasHeight: number) {
    // 水平移动
    this.x += this.speedX

    // 波浪效果：使用正弦波
    const waveAmplitude = 30
    const waveFrequency = 0.002
    const waveSpeed = 0.001
    const wave = Math.sin(this.x * waveFrequency + time * waveSpeed + this.phase) * waveAmplitude

    // 呼吸效果：整体上下浮动
    const breathAmplitude = 15
    const breathSpeed = 0.0008
    const breath = Math.sin(time * breathSpeed + this.phase) * breathAmplitude

    this.y = this.baseY + wave + breath + this.speedY

    // 透明度呼吸
    const opacityBreath = Math.sin(time * 0.001 + this.phase) * 0.2
    this.opacity = this.baseOpacity + opacityBreath

    // 边界检测
    if (this.x < 0) this.x = canvasWidth
    if (this.x > canvasWidth) this.x = 0
    if (this.y < -50) this.baseY = canvasHeight + 50
    if (this.y > canvasHeight + 50) this.baseY = -50
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.beginPath()
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2)
    // #F6FAF9 = rgb(246, 250, 249)
    ctx.fillStyle = `rgba(246, 250, 249, ${this.opacity})`
    ctx.fill()

    // 添加光晕效果
    const gradient = ctx.createRadialGradient(this.x, this.y, 0, this.x, this.y, this.size * 3)
    gradient.addColorStop(0, `rgba(246, 250, 249, ${this.opacity * 0.3})`)
    gradient.addColorStop(1, 'rgba(246, 250, 249, 0)')
    ctx.fillStyle = gradient
    ctx.fill()
  }
}

// 初始化粒子系统
const initParticles = () => {
  if (!particleCanvas.value) return

  const canvas = particleCanvas.value
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  // 设置画布尺寸
  const resizeCanvas = () => {
    canvas.width = window.innerWidth
    canvas.height = window.innerHeight
  }
  resizeCanvas()
  window.addEventListener('resize', resizeCanvas)

  // 创建粒子
  const particleCount = Math.floor((canvas.width * canvas.height) / 8000)
  const particles: Particle[] = []
  for (let i = 0; i < particleCount; i++) {
    particles.push(new Particle(canvas.width, canvas.height))
  }

  // 动画循环
  let startTime = Date.now()
  const animate = () => {
    const currentTime = Date.now() - startTime

    ctx.clearRect(0, 0, canvas.width, canvas.height)

    particles.forEach(particle => {
      particle.update(currentTime, canvas.width, canvas.height)
      particle.draw(ctx)
    })

    // 绘制连线（距离近的粒子）
    for (let i = 0; i < particles.length; i++) {
      for (let j = i + 1; j < particles.length; j++) {
        const dx = particles[i].x - particles[j].x
        const dy = particles[i].y - particles[j].y
        const distance = Math.sqrt(dx * dx + dy * dy)

        if (distance < 120) {
          ctx.beginPath()
          ctx.moveTo(particles[i].x, particles[i].y)
          ctx.lineTo(particles[j].x, particles[j].y)
          const opacity = (1 - distance / 120) * 0.15
          // #F6FAF9 = rgb(246, 250, 249)
          ctx.strokeStyle = `rgba(246, 250, 249, ${opacity})`
          ctx.lineWidth = 0.1
          ctx.stroke()
        }
      }
    }

    animationFrameId = requestAnimationFrame(animate)
  }

  animate()

  return () => {
    window.removeEventListener('resize', resizeCanvas)
  }
}

onMounted(() => {
  // 文字动画
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

  // 初始化粒子系统
  const cleanup = initParticles()

  // 加载排行榜数据
  loadRankings()

  onUnmounted(() => {
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId)
    }
    cleanup?.()
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
  width: 1900px;
  height: 1200px;
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

/* 玻璃拟态卡片 */
.glass-card {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.3);
}

/* 排行榜样式 */
.line-clamp-1 {
  display: -webkit-box;
  -webkit-line-clamp: 1;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

/* 滚动行为 */
html {
  scroll-behavior: smooth;
}

/* 隐藏滚动条 */
html::-webkit-scrollbar {
  display: none;
}

html {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}
image.png
/* 区域间距 */
section {
  scroll-margin-top: 80px;
}
</style>
