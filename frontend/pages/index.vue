<template>
  <div class="home-page">
    <ExhibitionHeader />
    <HomeHero />
    <HomeMajors />
    <HomeRanking />
    <HomeAbout />
    <ExhibitionFooter />
  </div>
</template>

<script setup lang="ts">
import { onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'

useHead({
  title: '首页 - 传媒技术学院2026届毕业设计展'
})

const route = useRoute()

const scrollToHash = (hash: string) => {
  if (!hash) return
  const sectionId = hash.replace('#', '')
  const element = document.getElementById(sectionId)
  if (element) {
    // 等待 DOM 渲染完成后再滚动
    setTimeout(() => {
      const headerHeight = 80
      const elementPosition = element.getBoundingClientRect().top
      const offsetPosition = elementPosition + window.pageYOffset - headerHeight
      window.scrollTo({ top: offsetPosition, behavior: 'smooth' })
    }, 300)
  }
}

onMounted(() => {
  if (route.hash) {
    scrollToHash(route.hash)
  }
})

watch(() => route.hash, (newHash) => {
  if (newHash) {
    scrollToHash(newHash)
  }
})
</script>
