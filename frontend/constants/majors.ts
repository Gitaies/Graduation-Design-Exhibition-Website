export interface MajorBase {
  code: string
  number: string
  name: string
  theme: string
}

export interface MajorDetail extends MajorBase {
  subtitle: string
  description: string
}

const MAJORS: MajorDetail[] = [
  {
    code: 'software',
    number: '01',
    name: '软件工程',
    theme: '数智焕新',
    subtitle: '数智驱动 · 焕新应用',
    description: '聚焦软件开发、人工智能与数据应用等方向的创新实践。'
  },
  {
    code: 'electronic',
    number: '02',
    name: '电子信息工程',
    theme: '芯火智造',
    subtitle: '芯火相传 · 智造终端',
    description: '聚焦嵌入式、物联网、通信与硬件系统的创新应用。'
  },
  {
    code: 'broadcast',
    number: '03',
    name: '广播电视工程',
    theme: '虚实共生',
    subtitle: '虚实共生 · 视界新生',
    description: '聚焦视音频制作、虚拟现实与数字媒体的融合创新。'
  }
]

/** Hero section mini cards - basic info only */
export const majors: MajorBase[] = MAJORS.map(({ code, number, name, theme }) => ({
  code, number, name, theme
}))

/** Majors section cards - full detail with subtitle */
export const majorsData: MajorDetail[] = MAJORS

/** About section cards - full detail */
export const aboutMajors: Omit<MajorDetail, 'subtitle'>[] = MAJORS.map(
  ({ subtitle: _, ...rest }) => rest
)
