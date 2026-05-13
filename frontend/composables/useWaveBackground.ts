/**
 * Sine wave background animation — vanilla TS, no jQuery.
 * Adapted for Vue/Nuxt composable pattern.
 */
interface WaveOptions {
  lineWidth: number
  amplitude: number
  wavelength: number
  strokeStyle?: string | CanvasGradient
  timeModifier?: number
}

interface WaveGeneratorOptions {
  el: HTMLCanvasElement
  waves: WaveOptions[]
  speed?: number
  height?: number
}

const PI2 = Math.PI * 2
const HALFPI = Math.PI / 2

export function useWaveBackground(options: WaveGeneratorOptions) {
  const { el, waves, speed = 0.5, height = 300 } = options
  const ctx = el.getContext('2d')!
  let dpr = window.devicePixelRatio || 1
  let width = 0
  let waveWidth = 0
  let waveLeft = 0
  let time = 0
  let rafId = 0
  let gradient: CanvasGradient

  function resize() {
    dpr = window.devicePixelRatio || 1
    width = window.innerWidth * dpr
    el.width = width
    el.height = height * dpr
    el.style.width = window.innerWidth + 'px'
    el.style.height = height + 'px'
    waveWidth = width * 0.95
    waveLeft = width * 0.025

    // Rebuild gradient
    gradient = ctx.createLinearGradient(0, 0, width, 0)
    gradient.addColorStop(0.1, 'rgba(20,102,255,0)')
    gradient.addColorStop(0.5, 'rgba(20,102,255,0.06)')
    gradient.addColorStop(0.7, 'rgba(55,200,255,0.08)')
    gradient.addColorStop(0.9, 'rgba(20,102,255,0)')
  }

  function ease(percent: number, amplitude: number): number {
    return amplitude * (Math.sin(percent * PI2 - HALFPI) + 1) * 0.5
  }

  function drawSine(t: number, opts: WaveOptions) {
    const amp = opts.amplitude
    const wl = opts.wavelength
    const lw = opts.lineWidth
    const style = opts.strokeStyle || gradient
    const segLen = 10
    const yAxis = (height * dpr) / 2

    ctx.lineWidth = lw * dpr
    ctx.strokeStyle = style
    ctx.lineCap = 'round'
    ctx.lineJoin = 'round'
    ctx.beginPath()

    ctx.moveTo(0, yAxis)
    ctx.lineTo(waveLeft, yAxis)

    for (let i = 0; i < waveWidth; i += segLen) {
      const x = t * speed + (-yAxis + i) / wl
      const y = Math.sin(x)
      const easedAmp = ease(i / waveWidth, amp)
      ctx.lineTo(i + waveLeft, easedAmp * y + yAxis)
    }

    ctx.lineTo(width, yAxis)
    ctx.stroke()
  }

  function update() {
    time -= 0.007
    ctx.clearRect(0, 0, width, el.height)
    for (const w of waves) {
      drawSine(time * (w.timeModifier || 1), w)
    }
  }

  function loop() {
    update()
    rafId = requestAnimationFrame(loop)
  }

  // Init
  resize()
  window.addEventListener('resize', resize)
  loop()

  // Cleanup
  function destroy() {
    cancelAnimationFrame(rafId)
    window.removeEventListener('resize', resize)
  }

  return { destroy }
}
