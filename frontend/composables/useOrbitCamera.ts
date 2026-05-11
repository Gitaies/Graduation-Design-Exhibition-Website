import * as THREE from 'three'
import { useRafManager } from './useRafManager'

export interface OrbitCameraOptions {
  radius?: number
  damping?: number
  maxThetaDeg?: number
  maxPhiDeg?: number
  fov?: number
  near?: number
  far?: number
}

export class OrbitCamera {
  camera: THREE.PerspectiveCamera
  private radius: number
  private damping: number
  private maxTheta: number
  private maxPhi: number
  private targetTheta = 0
  private targetPhi = 0
  private currentTheta = 0
  private currentPhi = 0

  constructor(options: OrbitCameraOptions = {}) {
    const {
      radius = 8,
      damping = 0.04,
      maxThetaDeg = 14.3,
      maxPhiDeg = 6.9,
      fov = 45,
      near = 0.1,
      far = 100,
    } = options

    this.radius = radius
    this.damping = damping
    this.maxTheta = (maxThetaDeg * Math.PI) / 180
    this.maxPhi = (maxPhiDeg * Math.PI) / 180

    const aspect = window.innerWidth / window.innerHeight
    this.camera = new THREE.PerspectiveCamera(fov, aspect, near, far)

    useRafManager().add(this)
  }

  setMouse(normalizedX: number, normalizedY: number) {
    this.targetTheta = normalizedX * this.maxTheta
    this.targetPhi = -normalizedY * this.maxPhi
  }

  update(_time: number, _delta: number) {
    this.currentTheta += (this.targetTheta - this.currentTheta) * this.damping
    this.currentPhi += (this.targetPhi - this.currentPhi) * this.damping

    const r = this.radius
    const t = this.currentTheta
    const p = this.currentPhi

    this.camera.position.x = r * Math.sin(t) * Math.cos(p)
    this.camera.position.y = r * Math.sin(p)
    this.camera.position.z = r * Math.cos(t) * Math.cos(p)
    this.camera.lookAt(0, 0, 0)
  }

  setAspect(aspect: number) {
    this.camera.aspect = aspect
    this.camera.updateProjectionMatrix()
  }

  dispose() {
    useRafManager().remove(this)
  }
}
