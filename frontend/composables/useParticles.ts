import { ref, onMounted, onUnmounted } from 'vue'

const THEME_COLORS = [
  { r: 150, g: 195, b: 248 },
  { r: 170, g: 210, b: 250 },
  { r: 130, g: 185, b: 248 },
  { r: 190, g: 220, b: 250 },
  { r: 160, g: 205, b: 250 },
]

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
  color: { r: number; g: number; b: number }

  constructor(canvasWidth: number, canvasHeight: number) {
    this.x = Math.pow(Math.random(), 0.5) * canvasWidth
    this.y = Math.pow(Math.random(), 0.5) * canvasHeight
    this.baseY = this.y
    this.size = Math.random() * 3.5 + 1.5
    this.speedX = (Math.random() - 0.5) * 0.4
    this.speedY = (Math.random() - 0.5) * 0.25
    this.baseOpacity = Math.random() * 0.45 + 0.35
    this.opacity = this.baseOpacity
    this.phase = Math.random() * Math.PI * 2
    this.color = THEME_COLORS[Math.floor(Math.random() * THEME_COLORS.length)]
  }

  update(time: number, canvasWidth: number, canvasHeight: number) {
    this.x += this.speedX

    const waveAmplitude = 30
    const waveFrequency = 0.002
    const waveSpeed = 0.001
    const wave = Math.sin(this.x * waveFrequency + time * waveSpeed + this.phase) * waveAmplitude

    const breathAmplitude = 15
    const breathSpeed = 0.0008
    const breath = Math.sin(time * breathSpeed + this.phase) * breathAmplitude

    this.y = this.baseY + wave + breath + this.speedY

    const opacityBreath = Math.sin(time * 0.001 + this.phase) * 0.15
    this.opacity = Math.max(0, Math.min(1, this.baseOpacity + opacityBreath))

    if (this.x < 0) this.x = canvasWidth
    if (this.x > canvasWidth) this.x = 0
    if (this.y < -50) this.baseY = canvasHeight + 50
    if (this.y > canvasHeight + 50) this.baseY = -50
  }

  draw(ctx: CanvasRenderingContext2D) {
    const { r, g, b } = this.color

    ctx.beginPath()
    ctx.arc(this.x, this.y, this.size, 0, Math.PI * 2)
    ctx.fillStyle = `rgba(${r}, ${g}, ${b}, ${this.opacity})`
    ctx.fill()

    const innerGlow = ctx.createRadialGradient(this.x, this.y, 0, this.x, this.y, this.size * 2.5)
    innerGlow.addColorStop(0, `rgba(${r}, ${g}, ${b}, ${this.opacity * 0.6})`)
    innerGlow.addColorStop(1, `rgba(${r}, ${g}, ${b}, 0)`)
    ctx.fillStyle = innerGlow
    ctx.fill()

    const outerGlow = ctx.createRadialGradient(this.x, this.y, 0, this.x, this.y, this.size * 5)
    outerGlow.addColorStop(0, `rgba(${r}, ${g}, ${b}, ${this.opacity * 0.25})`)
    outerGlow.addColorStop(1, `rgba(${r}, ${g}, ${b}, 0)`)
    ctx.fillStyle = outerGlow
    ctx.fill()
  }
}

export function useParticles(canvasRef: ReturnType<typeof ref<HTMLCanvasElement | null>>) {
  let animationFrameId: number | null = null

  const init = () => {
    const canvas = canvasRef.value
    if (!canvas) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    const resizeCanvas = () => {
      canvas.width = window.innerWidth
      canvas.height = window.innerHeight
    }
    resizeCanvas()
    window.addEventListener('resize', resizeCanvas)

    const particleCount = Math.floor((canvas.width * canvas.height) / 14000)
    const particles: Particle[] = []
    for (let i = 0; i < particleCount; i++) {
      particles.push(new Particle(canvas.width, canvas.height))
    }

    const startTime = Date.now()

    const animate = () => {
      const currentTime = Date.now() - startTime

      ctx.clearRect(0, 0, canvas.width, canvas.height)

      for (const particle of particles) {
        particle.update(currentTime, canvas.width, canvas.height)
        particle.draw(ctx)
      }

      // Draw connection lines
      for (let i = 0; i < particles.length; i++) {
        for (let j = i + 1; j < particles.length; j++) {
          const dx = particles[i].x - particles[j].x
          const dy = particles[i].y - particles[j].y
          const distance = Math.sqrt(dx * dx + dy * dy)

          if (distance < 140) {
            ctx.beginPath()
            ctx.moveTo(particles[i].x, particles[i].y)
            ctx.lineTo(particles[j].x, particles[j].y)
            const ratio = 1 - distance / 140
            const opacity = ratio * 0.22
            const c1 = particles[i].color
            const c2 = particles[j].color
            const mr = Math.round((c1.r + c2.r) / 2)
            const mg = Math.round((c1.g + c2.g) / 2)
            const mb = Math.round((c1.b + c2.b) / 2)
            ctx.strokeStyle = `rgba(${mr}, ${mg}, ${mb}, ${opacity})`
            ctx.lineWidth = 0.4
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

  let cleanup: (() => void) | undefined

  onMounted(() => {
    cleanup = init()
  })

  onUnmounted(() => {
    if (animationFrameId) {
      cancelAnimationFrame(animationFrameId)
    }
    cleanup?.()
  })
}
