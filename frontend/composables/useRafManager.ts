interface RafObject {
  update?: (time: number, delta: number) => void
}

class RafManager {
  private objects = new Set<RafObject>()
  private animId: number | null = null
  private lastTime = 0
  private running = false

  add(obj: RafObject) {
    this.objects.add(obj)
    if (!this.running) this.start()
  }

  remove(obj: RafObject) {
    this.objects.delete(obj)
    if (this.objects.size === 0) this.stop()
  }

  private start() {
    this.running = true
    this.lastTime = performance.now()
    this.animId = requestAnimationFrame(this.loop)
  }

  private stop() {
    this.running = false
    if (this.animId !== null) {
      cancelAnimationFrame(this.animId)
      this.animId = null
    }
  }

  private loop = (now: number) => {
    const delta = (now - this.lastTime) / 1000
    this.lastTime = now
    for (const obj of this.objects) {
      obj.update?.(now, delta)
    }
    this.animId = requestAnimationFrame(this.loop)
  }
}

let instance: RafManager | null = null

export function useRafManager(): RafManager {
  if (!instance) instance = new RafManager()
  return instance
}
