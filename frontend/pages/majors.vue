<template>
  <div class="majors-page">
    <ExhibitionHeader />

    <section class="py-20">
      <div class="container mx-auto px-4">
        <!-- 页面标题 -->
        <div class="text-center mb-16">
          <h1 class="text-5xl font-bold mb-4 gradient-text">专业展区</h1>
          <p class="text-xl text-text-secondary">PROFESSIONAL ZONES</p>
        </div>

        <!-- 专业卡片 -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-6xl mx-auto mb-20">
          <div
            v-for="major in majors"
            :key="major.code"
            class="glass-card rounded-3xl p-8 hover:shadow-2xl transition-all transform hover:-translate-y-3 cursor-pointer"
            @click="navigateTo(`/major/${major.code}`)"
          >
            <div class="text-primary-blue text-5xl font-bold mb-4">{{ major.number }}</div>
            <h2 class="text-3xl font-bold mb-2">{{ major.name }}</h2>
            <h3 class="text-xl text-primary-blue font-semibold mb-3">{{ major.theme }}</h3>
            <p class="text-sm text-text-secondary mb-4">{{ major.subtitle }}</p>
            <p class="text-base text-text-secondary leading-relaxed mb-6">{{ major.description }}</p>
            <button class="w-full py-3 bg-primary-blue text-white rounded-full hover:bg-primary-blue-dark transition-all">
              探索作品
            </button>
          </div>
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

    <ExhibitionFooter />
  </div>
</template>

<script setup lang="ts">
const majors = [
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

useHead({
  title: '专业展区 - 传媒技术学院2026届毕业设计展'
})
</script>
