<template>
  <div ref="containerRef" class="particle-background"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const containerRef = ref<HTMLElement | null>(null)
let canvas: HTMLCanvasElement
let ctx: CanvasRenderingContext2D
let particles: Particle[] = []
let animationId: number

class Particle {
  x: number
  y: number
  vx: number
  vy: number
  radius: number
  opacity: number

  constructor(width: number, height: number) {
    this.x = Math.random() * width
    this.y = Math.random() * height
    this.vx = (Math.random() - 0.5) * 0.5
    this.vy = (Math.random() - 0.5) * 0.5
    this.radius = Math.random() * 2 + 1
    this.opacity = Math.random() * 0.5 + 0.3
  }

  update(width: number, height: number) {
    this.x += this.vx
    this.y += this.vy

    if (this.x < 0 || this.x > width) this.vx *= -1
    if (this.y < 0 || this.y > height) this.vy *= -1
  }

  draw(ctx: CanvasRenderingContext2D) {
    ctx.beginPath()
    ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2)
    ctx.fillStyle = `rgba(20, 102, 255, ${this.opacity})`
    ctx.fill()
  }
}

onMounted(() => {
  if (!containerRef.value) return

  canvas = document.createElement('canvas')
  canvas.style.position = 'absolute'
  canvas.style.top = '0'
  canvas.style.left = '0'
  canvas.style.width = '100%'
  canvas.style.height = '100%'
  canvas.style.pointerEvents = 'none'

  containerRef.value.appendChild(canvas)

  ctx = canvas.getContext('2d')!

  const resize = () => {
    canvas.width = containerRef.value!.clientWidth
    canvas.height = containerRef.value!.clientHeight

    // 重新生成粒子
    particles = []
    const particleCount = Math.floor((canvas.width * canvas.height) / 15000)
    for (let i = 0; i < particleCount; i++) {
      particles.push(new Particle(canvas.width, canvas.height))
    }
  }

  resize()
  window.addEventListener('resize', resize)

  const animate = () => {
    animationId = requestAnimationFrame(animate)

    ctx.clearRect(0, 0, canvas.width, canvas.height)

    // 更新和绘制粒子
    particles.forEach(particle => {
      particle.update(canvas.width, canvas.height)
      particle.draw(ctx)
    })

    // 绘制连线
    for (let i = 0; i < particles.length; i++) {
      for (let j = i + 1; j < particles.length; j++) {
        const dx = particles[i].x - particles[j].x
        const dy = particles[i].y - particles[j].y
        const distance = Math.sqrt(dx * dx + dy * dy)

        if (distance < 120) {
          ctx.beginPath()
          ctx.moveTo(particles[i].x, particles[i].y)
          ctx.lineTo(particles[j].x, particles[j].y)
          ctx.strokeStyle = `rgba(55, 200, 255, ${0.15 * (1 - distance / 120)})`
          ctx.lineWidth = 0.5
          ctx.stroke()
        }
      }
    }
  }

  animate()

  onUnmounted(() => {
    window.removeEventListener('resize', resize)
    if (animationId) {
      cancelAnimationFrame(animationId)
    }
  })
})
</script>

<style scoped>
.particle-background {
  position: absolute;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
}
</style>
