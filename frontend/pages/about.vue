<template>
  <div class="about-page min-h-screen bg-bg-page">
    <ExhibitionHeader />

    <!-- 页面标题 -->
    <section class="relative overflow-hidden py-16 md:py-20">
      <div class="absolute inset-0 bg-gradient-to-br from-primary-blue/5 via-primary-cyan/5 to-transparent"></div>

      <div class="container mx-auto px-4 relative z-10 text-center">
        <div class="inline-block px-4 py-1.5 bg-primary-blue/10 text-primary-blue rounded-full text-sm font-medium mb-4">
          ABOUT US
        </div>
        <h1 class="text-4xl md:text-6xl font-bold mb-4 bg-gradient-to-r from-primary-blue to-primary-cyan bg-clip-text text-transparent">
          关于毕展
        </h1>
        <p class="text-lg text-text-secondary max-w-2xl mx-auto">
          智媒融合 · 创启未来
        </p>
      </div>
    </section>

    <!-- 展览介绍 -->
    <section class="py-12">
      <div class="container mx-auto px-4">
        <div class="max-w-4xl mx-auto">
          <div class="bg-white/80 backdrop-blur-sm rounded-2xl p-8 md:p-12 shadow-sm mb-12">
            <h2 class="text-2xl md:text-3xl font-bold text-text-main mb-6">展览介绍</h2>
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
              v-for="major in majors"
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
            <h2 class="text-2xl md:text-3xl font-bold text-text-main mb-6">学院简介</h2>
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
            <h2 class="text-2xl md:text-3xl font-bold text-text-main mb-6">联系我们</h2>
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
const majors = [
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

useHead({
  title: '关于毕展 - 传媒技术学院2026届毕业设计展'
})
</script>
