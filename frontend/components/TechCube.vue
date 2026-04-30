<template>
  <div ref="containerRef" class="tech-cube-container"></div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import * as THREE from 'three'

const containerRef = ref<HTMLElement | null>(null)
let scene: THREE.Scene
let camera: THREE.PerspectiveCamera
let renderer: THREE.WebGLRenderer
let cube: THREE.Mesh
let animationId: number

onMounted(() => {
  if (!containerRef.value) return

  // 创建场景
  scene = new THREE.Scene()

  // 创建相机
  camera = new THREE.PerspectiveCamera(
    75,
    containerRef.value.clientWidth / containerRef.value.clientHeight,
    0.1,
    1000
  )
  camera.position.z = 5

  // 创建渲染器
  renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true })
  renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight)
  renderer.setClearColor(0x000000, 0)
  containerRef.value.appendChild(renderer.domElement)

  // 创建立方体几何体
  const geometry = new THREE.BoxGeometry(2, 2, 2)

  // 创建边缘几何体
  const edges = new THREE.EdgesGeometry(geometry)
  const lineMaterial = new THREE.LineBasicMaterial({
    color: 0x1466ff,
    linewidth: 2,
    transparent: true,
    opacity: 0.8
  })
  const wireframe = new THREE.LineSegments(edges, lineMaterial)

  // 创建半透明面
  const material = new THREE.MeshBasicMaterial({
    color: 0x37c8ff,
    transparent: true,
    opacity: 0.1,
    side: THREE.DoubleSide
  })
  cube = new THREE.Mesh(geometry, material)
  cube.add(wireframe)

  // 添加顶点光点
  const pointsGeometry = new THREE.BufferGeometry()
  const vertices = []
  for (let i = 0; i < 8; i++) {
    const x = (i & 1) ? 1 : -1
    const y = (i & 2) ? 1 : -1
    const z = (i & 4) ? 1 : -1
    vertices.push(x, y, z)
  }
  pointsGeometry.setAttribute('position', new THREE.Float32BufferAttribute(vertices, 3))

  const pointsMaterial = new THREE.PointsMaterial({
    color: 0x1466ff,
    size: 0.15,
    transparent: true,
    opacity: 0.9
  })
  const points = new THREE.Points(pointsGeometry, pointsMaterial)
  cube.add(points)

  scene.add(cube)

  // 动画循环
  const animate = () => {
    animationId = requestAnimationFrame(animate)

    // 旋转立方体
    cube.rotation.x += 0.005
    cube.rotation.y += 0.008

    renderer.render(scene, camera)
  }

  animate()

  // 响应窗口大小变化
  const handleResize = () => {
    if (!containerRef.value) return

    camera.aspect = containerRef.value.clientWidth / containerRef.value.clientHeight
    camera.updateProjectionMatrix()
    renderer.setSize(containerRef.value.clientWidth, containerRef.value.clientHeight)
  }

  window.addEventListener('resize', handleResize)

  onUnmounted(() => {
    window.removeEventListener('resize', handleResize)
    if (animationId) {
      cancelAnimationFrame(animationId)
    }
    if (renderer) {
      renderer.dispose()
    }
  })
})
</script>

<style scoped>
.tech-cube-container {
  width: 100%;
  height: 100%;
  min-height: 400px;
}
</style>
