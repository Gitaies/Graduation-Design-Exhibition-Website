<template>
  <Teleport to="body">
    <Transition name="share-fade">
      <div v-if="visible" class="share-overlay" @click.self="close">
        <div class="share-panel">
          <!-- 关闭按钮 -->
          <button class="share-close" @click="close" aria-label="关闭">
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M5 5l10 10M15 5L5 15" />
            </svg>
          </button>

          <h3 class="share-title">分享作品</h3>

          <!-- ==================== PC 端 ==================== -->
          <template v-if="!isMobile">
            <div class="copied-badge">
              <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M13 5l-5.5 5.5L4 8" />
              </svg>
              链接已复制到剪贴板
            </div>

            <div class="share-options">
              <!-- 微信 - 二维码 -->
              <div class="share-option">
                <div class="share-option-icon wechat">
                  <svg width="28" height="28" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M8.691 2.188C3.891 2.188 0 5.476 0 9.53c0 2.212 1.17 4.203 3.002 5.55a.59.59 0 0 1 .213.665l-.39 1.48c-.019.07-.048.141-.048.213 0 .163.13.295.29.295a.326.326 0 0 0 .167-.054l1.903-1.114a.864.864 0 0 1 .717-.098 10.16 10.16 0 0 0 2.837.403c.276 0 .543-.027.811-.05-.857-2.578.157-4.972 1.932-6.446 1.703-1.415 3.882-1.98 5.853-1.838-.576-3.583-4.196-6.348-8.596-6.348zM5.785 5.991c.642 0 1.162.529 1.162 1.18a1.17 1.17 0 0 1-1.162 1.178A1.17 1.17 0 0 1 4.623 7.17c0-.651.52-1.18 1.162-1.18zm5.813 0c.642 0 1.162.529 1.162 1.18a1.17 1.17 0 0 1-1.162 1.178 1.17 1.17 0 0 1-1.162-1.178c0-.651.52-1.18 1.162-1.18zm3.09 3.204c-2.599 0-4.66 2.029-4.66 4.577 0 2.549 2.061 4.578 4.66 4.578.409 0 .809-.064 1.188-.16.177-.047.371-.04.543.04l1.47.859a.323.323 0 0 0 .168.054c.16 0 .29-.132.29-.295a.349.349 0 0 0-.048-.213l-.301-1.136a.586.586 0 0 1 .212-.664C20.027 16.55 21 15.426 21 14.163c0-2.549-2.06-4.577-4.66-4.577-.198 0-.396.01-.59.03a4.924 4.924 0 0 0-1.062-2.401zm-1.327 2.22c.427 0 .775.353.775.787a.78.78 0 0 1-.775.787.78.78 0 0 1-.775-.787c0-.434.348-.787.775-.787zm3.877 0c.427 0 .775.353.775.787a.78.78 0 0 1-.775.787.78.78 0 0 1-.775-.787c0-.434.348-.787.775-.787z" />
                  </svg>
                </div>
                <span class="share-option-label">微信</span>
                <img v-if="qrDataUrl" :src="qrDataUrl" alt="微信扫码分享" class="qr-image" />
                <p class="share-hint">打开微信扫一扫，分享给朋友</p>
              </div>

              <!-- QQ -->
              <div class="share-option">
                <div class="share-option-icon qq">
                  <svg width="28" height="28" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12.003 2c-2.265 0-6.29 1.364-6.29 7.325v1.195S3.55 14.96 3.55 17.474v.757c0 1.212.903 1.616 1.708 1.616 1.21 0 1.959-.202 1.959-.202.202 1.07 1.495 2.355 4.786 2.355 3.209 0 4.543-1.285 4.745-2.355 0 0 .748.202 1.96.202.804 0 1.707-.404 1.707-1.616v-.757c0-2.515-2.058-6.956-2.058-6.956v-1.195C18.36 3.364 14.268 2 12.003 2z" />
                  </svg>
                </div>
                <span class="share-option-label">QQ</span>
                <button class="share-action-btn qq-share-btn" @click="shareToQQ">
                  分享到 QQ 好友
                </button>
              </div>
            </div>
          </template>

          <!-- ==================== 移动端 ==================== -->
          <template v-else>
            <!-- 微信内置浏览器 -->
            <div v-if="isWechat" class="mobile-guide wechat-guide">
              <div class="guide-arrow">
                <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
                  <path d="M36 16L24 4L12 16" stroke="#1466ff" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                  <path d="M24 4V28" stroke="#1466ff" stroke-width="3" stroke-linecap="round"/>
                  <circle cx="36" cy="36" r="3" fill="#1466ff" opacity="0.3"/>
                  <circle cx="24" cy="40" r="3" fill="#1466ff" opacity="0.5"/>
                  <circle cx="12" cy="36" r="3" fill="#1466ff" opacity="0.7"/>
                </svg>
              </div>
              <p class="guide-text">点击右上角 <strong>···</strong></p>
              <p class="guide-sub">选择 <strong>分享给朋友</strong> 或 <strong>分享到朋友圈</strong></p>
            </div>

            <!-- QQ 内置浏览器 -->
            <div v-else-if="isQQ" class="mobile-guide qq-guide">
              <div class="guide-arrow">
                <svg width="48" height="48" viewBox="0 0 48 48" fill="none">
                  <path d="M36 16L24 4L12 16" stroke="#1466ff" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>
                  <path d="M24 4V28" stroke="#1466ff" stroke-width="3" stroke-linecap="round"/>
                  <circle cx="36" cy="36" r="3" fill="#1466ff" opacity="0.3"/>
                  <circle cx="24" cy="40" r="3" fill="#1466ff" opacity="0.5"/>
                  <circle cx="12" cy="36" r="3" fill="#1466ff" opacity="0.7"/>
                </svg>
              </div>
              <p class="guide-text">点击右上角 <strong>···</strong></p>
              <p class="guide-sub">选择分享给 QQ 好友或 QQ 空间</p>
            </div>

            <!-- 普通手机浏览器 -->
            <div v-else class="mobile-actions">
              <button class="share-action-btn primary" @click="shareNative">
                <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
                </svg>
                分享给好友
              </button>
            </div>

            <!-- 复制链接（所有移动端通用） -->
            <button class="share-action-btn copy" @click="copyLink">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <rect x="9" y="9" width="13" height="13" rx="2" />
                <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
              </svg>
              复制链接
            </button>
          </template>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import QRCode from 'qrcode'

const props = defineProps<{
  workTitle: string
  workDescription?: string
  workPoster?: string
}>()

const visible = ref(false)
const isMobile = ref(false)
const isWechat = ref(false)
const isQQ = ref(false)
const qrDataUrl = ref('')

function detectEnv() {
  const ua = navigator.userAgent
  isMobile.value = /Android|iPhone|iPad|iPod/i.test(ua)
  isWechat.value = /MicroMessenger/i.test(ua)
  isQQ.value = /QQ\//i.test(ua) && !isWechat.value
}

async function open() {
  detectEnv()

  const url = window.location.href

  // 移动端普通浏览器：优先尝试 Web Share API
  if (isMobile.value && !isWechat.value && !isQQ.value && navigator.share) {
    try {
      await navigator.share({
        title: props.workTitle,
        text: props.workDescription || '',
        url,
      })
      return // 分享成功，不弹面板
    } catch {
      // 用户取消或不支持，继续弹出面板
    }
  }

  // 先复制链接
  try {
    await navigator.clipboard.writeText(url)
  } catch {
    // 降级：手动选中复制
    manualCopy(url)
  }

  // 桌面端预生成二维码
  if (!isMobile.value) {
    try {
      qrDataUrl.value = await QRCode.toDataURL(url, {
        width: 200,
        margin: 1,
        color: { dark: '#1466ff', light: '#ffffff' },
      })
    } catch {
      // 二维码生成失败
    }
  }

  visible.value = true
}

function close() {
  visible.value = false
}

function shareToQQ() {
  const url = encodeURIComponent(window.location.href)
  const title = encodeURIComponent(props.workTitle)
  const desc = encodeURIComponent(props.workDescription || '')
  const pics = encodeURIComponent(props.workPoster || '')
  const qqUrl = `https://connect.qq.com/widget/shareqq/index.html?url=${url}&title=${title}&desc=${desc}&pics=${pics}&site=${encodeURIComponent('传媒技术学院2026届毕业设计展')}`
  window.open(qqUrl, '_blank', 'width=700,height=520')
}

async function shareNative() {
  const url = window.location.href
  try {
    await navigator.share({
      title: props.workTitle,
      text: props.workDescription || '',
      url,
    })
  } catch {
    // 忽略
  }
}

async function copyLink() {
  const url = window.location.href
  try {
    await navigator.clipboard.writeText(url)
  } catch {
    manualCopy(url)
  }
}

function manualCopy(text: string) {
  const textarea = document.createElement('textarea')
  textarea.value = text
  textarea.style.position = 'fixed'
  textarea.style.opacity = '0'
  document.body.appendChild(textarea)
  textarea.select()
  try {
    document.execCommand('copy')
  } catch {
    // 最终降级
  }
  document.body.removeChild(textarea)
}

defineExpose({ open, close })
</script>

<style scoped>
/* ========== 遮罩 ========== */
.share-overlay {
  position: fixed;
  inset: 0;
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.45);
  backdrop-filter: blur(6px);
  -webkit-backdrop-filter: blur(6px);
  padding: 1rem;
}

/* ========== 面板 ========== */
.share-panel {
  position: relative;
  width: 100%;
  max-width: 400px;
  background: rgba(255, 255, 255, 0.92);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 20px;
  padding: 2rem 1.5rem 1.75rem;
  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.18), 0 0 0 1px rgba(255, 255, 255, 0.5);
}

/* ========== 关闭按钮 ========== */
.share-close {
  position: absolute;
  top: 14px;
  right: 14px;
  width: 34px;
  height: 34px;
  border-radius: 50%;
  border: none;
  background: rgba(0, 0, 0, 0.05);
  color: #666;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.share-close:hover {
  background: rgba(0, 0, 0, 0.1);
  color: #333;
}

/* ========== 标题 ========== */
.share-title {
  font-size: 1.25rem;
  font-weight: 700;
  color: #1a1a2e;
  margin-bottom: 1.25rem;
  text-align: center;
}

/* ========== 已复制提示 ========== */
.copied-badge {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 8px 16px;
  background: linear-gradient(135deg, #e8fff0, #d4f5e0);
  color: #1a8a4a;
  border-radius: 10px;
  font-size: 0.875rem;
  font-weight: 500;
  margin-bottom: 1.25rem;
}

/* ========== PC 分享选项 ========== */
.share-options {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.share-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1.25rem;
  background: rgba(0, 0, 0, 0.02);
  border-radius: 14px;
  border: 1px solid rgba(0, 0, 0, 0.06);
}

.share-option-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.share-option-icon.wechat {
  background: #e8fce8;
  color: #07c160;
}

.share-option-icon.qq {
  background: #e8f0ff;
  color: #12b7f5;
}

.share-option-label {
  font-size: 0.875rem;
  font-weight: 600;
  color: #333;
}

/* 二维码 */
.qr-image {
  width: 160px;
  height: 160px;
  border-radius: 10px;
  border: 1px solid rgba(0, 0, 0, 0.08);
}

.share-hint {
  font-size: 0.8rem;
  color: #999;
  margin: 0;
}

/* ========== 分享按钮通用 ========== */
.share-action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  padding: 12px 20px;
  border-radius: 12px;
  border: none;
  font-size: 0.95rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.share-action-btn.qq-share-btn {
  background: linear-gradient(135deg, #12b7f5, #0d9ed9);
  color: #fff;
  max-width: 220px;
}

.share-action-btn.qq-share-btn:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 14px rgba(18, 183, 245, 0.4);
}

/* ========== 移动端 ========== */
.mobile-guide {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem 0;
}

.guide-arrow {
  margin-bottom: 0.25rem;
}

.guide-text {
  font-size: 1.1rem;
  font-weight: 600;
  color: #1a1a2e;
  margin: 0;
}

.guide-sub {
  font-size: 0.9rem;
  color: #666;
  margin: 0;
  text-align: center;
}

.mobile-actions {
  margin-bottom: 0.75rem;
}

.share-action-btn.primary {
  background: linear-gradient(135deg, #1466ff, #37c8ff);
  color: #fff;
}

.share-action-btn.primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 14px rgba(20, 102, 255, 0.4);
}

.share-action-btn.primary:active {
  transform: scale(0.98);
}

.share-action-btn.copy {
  background: rgba(0, 0, 0, 0.05);
  color: #333;
}

.share-action-btn.copy:hover {
  background: rgba(0, 0, 0, 0.08);
}

.share-action-btn.copy:active {
  background: rgba(0, 0, 0, 0.12);
  transform: scale(0.98);
}

/* ========== 过渡动画 ========== */
.share-fade-enter-active,
.share-fade-leave-active {
  transition: opacity 0.25s ease;
}

.share-fade-enter-from,
.share-fade-leave-to {
  opacity: 0;
}

/* ========== 响应式 ========== */
@media (max-width: 480px) {
  .share-panel {
    margin: 0 0.5rem;
    padding: 1.5rem 1.125rem 1.25rem;
    border-radius: 16px;
  }

  .share-title {
    font-size: 1.125rem;
    margin-bottom: 1rem;
  }
}
</style>
