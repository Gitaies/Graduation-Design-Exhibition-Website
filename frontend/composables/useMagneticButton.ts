import { ref, onMounted, onUnmounted } from 'vue'

const clamp = (value: number, min: number, max: number) => Math.min(Math.max(value, min), max)

interface MagneticState {
  x: number
  y: number
  scale: number
  isNear: boolean
  lightX: number
  lightY: number
  targetX: number
  targetY: number
  targetScale: number
  velocityX: number
  velocityY: number
  scaleVelocity: number
}

function createInitialState(): MagneticState {
  return {
    x: 0, y: 0, scale: 1, isNear: false,
    lightX: 50, lightY: 50,
    targetX: 0, targetY: 0, targetScale: 1,
    velocityX: 0, velocityY: 0, scaleVelocity: 0
  }
}

export function useMagneticButton(count: number) {
  const magneticButtons = ref<(HTMLElement | null)[]>([])
  const magneticStates = ref<MagneticState[]>(
    Array.from({ length: count }, () => createInitialState())
  )

  const handleMagneticMove = (event: PointerEvent, index: number) => {
    if (event.pointerType && event.pointerType !== 'mouse') return

    const button = magneticButtons.value[index]
    if (!button) return

    const state = magneticStates.value[index]
    const rect = button.getBoundingClientRect()

    const currentCenterX = rect.left + rect.width / 2
    const currentCenterY = rect.top + rect.height / 2

    const isInsideCurrentButton =
      event.clientX >= rect.left &&
      event.clientX <= rect.right &&
      event.clientY >= rect.top &&
      event.clientY <= rect.bottom

    if (isInsideCurrentButton) {
      if (!state.isNear) {
        state.isNear = true
      }

      const dx = event.clientX - currentCenterX
      const dy = event.clientY - currentCenterY

      const normalizedX = clamp(dx / (rect.width / 2), -1, 1)
      const normalizedY = clamp(dy / (rect.height / 2), -1, 1)

      state.targetX = dx * 0.28
      state.targetY = dy * 0.34
      state.targetScale = 1.045 - (Math.abs(normalizedX) + Math.abs(normalizedY)) * 0.008

      state.lightX = clamp(((event.clientX - rect.left) / rect.width) * 100, 0, 100)
      state.lightY = clamp(((event.clientY - rect.top) / rect.height) * 100, 0, 100)
    } else if (state.isNear) {
      handleMagneticLeave(index)
    }
  }

  const handleMagneticLeave = (index: number) => {
    const state = magneticStates.value[index]
    state.targetX = 0
    state.targetY = 0
    state.targetScale = 1
    state.isNear = false

    state.velocityX += -state.x * 0.72
    state.velocityY += -state.y * 0.72
    state.scaleVelocity += (1 - state.scale) * 0.82
  }

  let animId: number | null = null

  const animate = () => {
    magneticStates.value.forEach((state) => {
      const returning = !state.isNear && (
        Math.abs(state.x) > 0.01 ||
        Math.abs(state.y) > 0.01 ||
        Math.abs(state.scale - 1) > 0.0005
      )

      const stiffness = returning ? 0.34 : 0.20
      const damping = returning ? 0.58 : 0.72
      const scaleStiffness = returning ? 0.34 : 0.20
      const scaleDamping = returning ? 0.58 : 0.74

      state.velocityX += (state.targetX - state.x) * stiffness
      state.velocityY += (state.targetY - state.y) * stiffness
      state.velocityX *= damping
      state.velocityY *= damping
      state.x += state.velocityX
      state.y += state.velocityY

      state.scaleVelocity += (state.targetScale - state.scale) * scaleStiffness
      state.scaleVelocity *= scaleDamping
      state.scale += state.scaleVelocity

      if (!state.isNear && Math.abs(state.x) < 0.01 && Math.abs(state.y) < 0.01) {
        state.x = 0
        state.y = 0
        state.velocityX = 0
        state.velocityY = 0
      }

      if (!state.isNear && Math.abs(state.scale - 1) < 0.0005) {
        state.scale = 1
        state.scaleVelocity = 0
      }
    })

    animId = requestAnimationFrame(animate)
  }

  onMounted(() => {
    animate()
  })

  onUnmounted(() => {
    if (animId !== null) {
      cancelAnimationFrame(animId)
    }
  })

  return {
    magneticButtons,
    magneticStates,
    handleMagneticMove,
    handleMagneticLeave
  }
}
