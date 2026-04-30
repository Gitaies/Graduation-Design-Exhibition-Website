<template>
  <footer class="bg-white border-t border-border-light py-12">
    <div class="container mx-auto px-4">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mb-8">
        <!-- 关于 -->
        <div>
          <h3 class="text-lg font-bold mb-4">关于毕业设计展</h3>
          <p class="text-sm text-text-secondary leading-relaxed">
            传媒技术学院2026届毕业设计展，展示三大专业的创新成果与应用实践。
          </p>
        </div>

        <!-- 快速链接 -->
        <div>
          <h3 class="text-lg font-bold mb-4">快速链接</h3>
          <ul class="space-y-2">
            <li><NuxtLink to="/majors" class="text-sm text-text-secondary hover:text-primary-blue transition-colors">专业展区</NuxtLink></li>
            <li><NuxtLink to="/ranking" class="text-sm text-text-secondary hover:text-primary-blue transition-colors">点赞热榜</NuxtLink></li>
            <li><NuxtLink to="/about" class="text-sm text-text-secondary hover:text-primary-blue transition-colors">关于毕展</NuxtLink></li>
          </ul>
        </div>

        <!-- 联系方式 -->
        <div>
          <h3 class="text-lg font-bold mb-4">联系我们</h3>
          <p class="text-sm text-text-secondary">传媒技术学院</p>
          <p class="text-sm text-text-secondary">邮箱: contact@example.com</p>
        </div>
      </div>

      <!-- 版权信息 -->
      <div class="border-t border-border-light pt-8 text-center">
        <p class="text-sm text-text-light">
          © 2026 传媒技术学院. All rights reserved.
        </p>
      </div>
    </div>
  </footer>
</template>
