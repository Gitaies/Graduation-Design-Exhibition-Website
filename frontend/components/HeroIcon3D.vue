<template>
  <div ref="containerRef" class="hero-icon-3d" />
</template>

<script setup lang="ts">
import * as THREE from 'three'
import { OrbitCamera } from '~/composables/useOrbitCamera'
import { useRafManager } from '~/composables/useRafManager'
import heroIconSrc from '~/assets/images/hero/hero_icon.png'

const containerRef = ref<HTMLDivElement | null>(null)

let scene: THREE.Scene | null = null
let renderer: THREE.WebGLRenderer | null = null
let orbitCamera: OrbitCamera | null = null
let renderObj: { update: (t: number, d: number) => void } | null = null
let iconPlane: THREE.Mesh | null = null
let iconTexture: THREE.Texture | null = null
let resizeObserver: ResizeObserver | null = null
let startTime = 0

function buildPlane(w: number, h: number, texture: THREE.Texture) {
  if (!scene) return

  if (iconPlane) {
    scene.remove(iconPlane)
    iconPlane.geometry.dispose()
    ;(iconPlane.material as THREE.Material).dispose()
  }

  const fovRad = 40 * (Math.PI / 180)
  const visibleH = 2 * 5 * Math.tan(fovRad / 2)
  const visibleW = visibleH * (w / Math.max(h, 1))

  const geo = new THREE.PlaneGeometry(visibleW, visibleH)
  const mat = new THREE.MeshBasicMaterial({
    map: texture,
    transparent: true,
    side: THREE.DoubleSide,
  })

  iconPlane = new THREE.Mesh(geo, mat)
  scene.add(iconPlane)
}

function init() {
  if (!containerRef.value) return
  const container = containerRef.value

  const w = container.clientWidth || 640
  const h = container.clientHeight || Math.round(w * 0.5625)

  scene = new THREE.Scene()

  renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true })
  renderer.setSize(w, h)
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2))
  renderer.outputColorSpace = THREE.SRGBColorSpace
  container.appendChild(renderer.domElement)

  orbitCamera = new OrbitCamera({
    radius: 5,
    damping: 0.025,
    maxThetaDeg: 10,
    maxPhiDeg: 6,
    fov: 40,
  })

  const loader = new THREE.TextureLoader()
  iconTexture = loader.load(heroIconSrc)
  iconTexture.colorSpace = THREE.SRGBColorSpace
  iconTexture.minFilter = THREE.LinearMipmapLinearFilter
  iconTexture.magFilter = THREE.LinearFilter
  buildPlane(w, h, iconTexture)

  startTime = performance.now()

  renderObj = {
    update(time: number, _delta: number) {
      if (!scene || !renderer || !orbitCamera || !iconPlane) return
      const t = (time - startTime) * 0.001
      iconPlane.position.y = Math.sin(t * 0.6) * 0.12
      iconPlane.rotation.z = Math.sin(t * 0.4 + 1.5) * 0.02
      renderer.render(scene, orbitCamera.camera)
    },
  }

  useRafManager().add(renderObj)

  resizeObserver = new ResizeObserver((entries) => {
    for (const entry of entries) {
      const { width, height } = entry.contentRect
      if (width > 0 && height > 0 && renderer && orbitCamera) {
        orbitCamera.setAspect(width / height)
        renderer.setSize(width, height)
        if (iconPlane && scene && iconTexture) {
          scene.remove(iconPlane)
          iconPlane.geometry.dispose()
          ;(iconPlane.material as THREE.Material).dispose()
          buildPlane(width, height, iconTexture)
        }
      }
    }
  })
  resizeObserver.observe(container)
}

function onMouseMove(e: MouseEvent) {
  const x = (e.clientX / window.innerWidth) * 2 - 1
  const y = (e.clientY / window.innerHeight) * 2 - 1
  orbitCamera?.setMouse(x, y)
}

onMounted(() => {
  init()
  window.addEventListener('mousemove', onMouseMove, { passive: true })
})

onUnmounted(() => {
  window.removeEventListener('mousemove', onMouseMove)
  resizeObserver?.disconnect()
  if (renderObj) useRafManager().remove(renderObj)
  orbitCamera?.dispose()
  renderer?.dispose()
  renderer?.domElement?.remove()
  if (iconPlane) {
    iconPlane.geometry.dispose()
    ;(iconPlane.material as THREE.Material).dispose()
  }
  scene?.clear()
})
</script>

<style scoped>
.hero-icon-3d {
  width: 100%;
  height: 100%;
}

.hero-icon-3d :deep(canvas) {
  display: block;
}
</style>
