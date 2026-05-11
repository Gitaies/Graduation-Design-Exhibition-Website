<template>
  <section id="majors" class="majors-section py-4 relative">
    <!-- 背景装饰图片 -->
    <div class="absolute top-[130px] right-[-180px] z-0 overflow-hidden transform rotate-45 w-[900px]">
      <img
        src="/images/nav.png"
        alt="背景装饰"
        class="w-full h-full object-cover"
      />
    </div>

    <div class="container mx-auto px-4 relative z-10">
      <!-- 页面标题 - 左对齐 -->
      <div class="mb-8">
        <h2 class="text-3xl md:text-4xl font-bold text-text-main inline-block">
          专业展区
          <span class="text-sm md:text-base text-text-secondary tracking-wider ml-3">
            / PROFESSIONAL ZONES
          </span>
        </h2>
      </div>

      <!-- 专业卡片 - 紧凑版 -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 md:gap-24 mb-1">
        <NuxtLink
          v-for="(major, index) in majorsData"
          :key="major.code"
          :to="`/major/${major.code}`"
          :class="[
            'major-card-compact group block rounded-2xl p-5 md:p-6 transition-all duration-500 transform hover:-translate-y-2 cursor-pointer relative overflow-hidden shadow-lg hover:shadow-2xl',
            index === 0 ? 'bg-gradient-to-br from-blue-100 to-blue-50 hover:from-blue-200 hover:to-blue-100' : '',
            index === 1 ? 'bg-gradient-to-br from-purple-100 to-purple-50 hover:from-purple-200 hover:to-purple-100' : '',
            index === 2 ? 'bg-gradient-to-br from-cyan-100 to-cyan-50 hover:from-cyan-200 hover:to-cyan-100' : ''
          ]"
        >
          <!-- 背景装饰 -->
          <div class="absolute top-0 right-0 w-60 h-60 transform translate-x-0 -translate-y-6 opacity-60 group-hover:opacity-100">
            <img
              :src="index === 0 ? '/images/icon_1.png' : index === 1 ? '/images/icon_2.png' : '/images/icon_3.png'"
              :alt="`${major.name}装饰`"
              class="w-full h-full object-contain"
            />
          </div>

          <!-- 内容 -->
          <div class="relative z-10">
            <!-- 编号 -->
            <div :class="[
              'text-5xl md:text-6xl font-black mb-4 opacity-20',
              index === 0 ? 'text-blue-500' : '',
              index === 1 ? 'text-purple-500' : '',
              index === 2 ? 'text-cyan-500' : ''
            ]">
              {{ major.number }}
            </div>

            <!-- 专业名称 -->
            <h3 :class="[
            'text-xl md:text-2xl font-bold mb-8',
              index === 0 ? 'text-blue-600' : '',
              index === 1 ? 'text-purple-600' : '',
              index === 2 ? 'text-cyan-600' : ''
            ]">
              {{ major.name }}
            </h3>

            <!-- 主题 -->
            <h4 :class="[
              'text-3xl md:text-[56px] font-bold mb-10',
              index === 0 ? 'text-blue-600' : '',
              index === 1 ? 'text-purple-600' : '',
              index === 2 ? 'text-cyan-600' : ''
            ]">
              {{ major.theme }}
            </h4>

            <!-- 副标题 -->
            <p class="text-sm md:text-[32px] text-text-secondary mb-8 font-medium">
              {{ major.subtitle }}
            </p>

            <!-- 描述 -->
            <p class="text-xs md:text-sm text-text-secondary leading-relaxed mb-6">
              {{ major.description }}
            </p>

            <!-- 探索按钮 -->
            <div class="flex justify-center mt-16">
              <div
                class="magnetic-field"
                @pointermove="handleMagneticMove($event, index)"
                @pointerleave="handleMagneticLeave(index)"
              >
                <div
                  :ref="el => { if (el) magneticButtons[index] = el }"
                  :class="[
                    'magnetic-btn inline-flex items-center gap-2 px-12 py-4 rounded-full text-sm font-medium transition-shadow duration-300 cursor-pointer relative',
                    index === 0 ? 'bg-gradient-to-r from-blue-500 to-blue-600' : '',
                    index === 1 ? 'bg-gradient-to-r from-purple-500 to-purple-600' : '',
                    index === 2 ? 'bg-gradient-to-r from-cyan-500 to-cyan-600' : '',
                    magneticStates[index]?.isNear ? 'is-magnetic-active shadow-2xl' : 'shadow-lg'
                  ]"
                  :style="{
                    transform: `translate3d(${magneticStates[index]?.x || 0}px, ${magneticStates[index]?.y || 0}px, 0) scale(${magneticStates[index]?.scale || 1})`,
                    '--light-x': `${magneticStates[index]?.lightX || 50}%`,
                    '--light-y': `${magneticStates[index]?.lightY || 50}%`
                  }"
                >
                  <span class="relative z-10 text-white font-bold">探索作品</span>
                  <svg class="w-4 h-4 relative z-10 text-white transition-transform duration-300" :class="{ 'translate-x-1': magneticStates[index]?.isNear }" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                  </svg>
                </div>
              </div>
            </div>
          </div>
        </NuxtLink>
      </div>

      <!-- 底部统计 -->
      <div class="flex items-center justify-center gap-4 md:gap-8">
        <!-- 左侧装饰图案 -->
        <img src="/images/icon1.png" alt="" class="w-16 h-16 md:w-20 md:h-20 opacity-60" />

        <!-- 统计数据 -->
        <div class="flex items-center gap-4 md:gap-16">
          <div class="text-center">
            <div class="text-1xl md:text-[18px] font-bold text-[#6C7FB7] mb-1">3 大专业</div>
          </div>

          <div class="text-1xl md:text-[18px] text-text-light font-bold">/</div>

          <div class="text-center">
            <div class="text-1xl md:text-[18px] font-bold text-[#6C7FB7] mb-1">30+ 优秀作品</div>
          </div>

          <div class="text-1xl md:text-[18px] text-text-light font-bold">/</div>

          <div class="text-center">
            <div class="text-1xl md:text-[18px] font-bold text-[#6C7FB7] mb-1">180+ 毕业生</div>
          </div>

          <div class="text-1xl md:text-[18px] text-text-light font-bold">/</div>

          <div class="text-center">
            <div class="text-1xl md:text-[18px] font-bold text-[#6C7FB7] mb-1">无限可能</div>
          </div>
        </div>

        <!-- 右侧装饰图案 -->
        <img src="/images/icon1.png" alt="" class="w-16 h-16 md:w-20 md:h-20 opacity-60" />
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
/* 专业展区背景渐变 */
.majors-section {
  background: linear-gradient(to bottom, #edf4fe 0%, #F3FBFB 100%);
}

/* 专业展区紧凑卡片 */
.major-card-compact {
  backdrop-filter: blur(16px);
  border: 1px solid rgba(255, 255, 255, 0.5);
  min-height: 540px;
}

.major-card-compact:hover {
  border-color: rgba(255, 255, 255, 0.8);
}

/* 磁性按钮样式 */
.magnetic-field {
  position: relative;
  display: grid;
  place-items: center;
  width: 100%;
  height: auto;
  touch-action: none;
}

.magnetic-btn {
  position: relative;
  will-change: transform;
  transition: box-shadow 220ms ease;
  user-select: none;
}
</style>
