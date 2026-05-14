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
    description: '以数据与AI驱动应用创新为核心。12件作品覆盖AI原生应用、数据智能、行业服务平台、工具创新四大矩阵，涵盖MCP协议智能体、RAG大模型、YOLO视觉分析等前沿技术，展现"大模型落地、小场景深耕"的专业特色。'
  },
  {
    code: 'electronic',
    number: '02',
    name: '电子信息工程',
    theme: '芯火智造',
    subtitle: '芯火相传 · 智造终端',
    description: '以芯片级嵌入式技术为根基。11件作品形成智能控制、物联网、嵌入式人机交互三大技术集群，从STM32微控制器到视觉伺服抓取系统，体现"小系统、真场景、硬实现"的终端开发能力。'
  },
  {
    code: 'broadcast',
    number: '03',
    name: '广播电视工程',
    theme: '虚实共生',
    subtitle: '虚实共生 · 视界新生',
    description: '探索虚拟技术与超高清制播的融合创新。10件作品覆盖超高清制播、虚拟制作、智能内容生成、传输覆盖四大方向，以UE5引擎、AI生成、卫星传输等技术重新定义视听传播的边界。'
  }
]

/** Hero section mini cards - basic info only */
export const majors: MajorBase[] = MAJORS.map(({ code, number, name, theme }) => ({
  code, number, name, theme
}))

/** Majors section cards - full detail with subtitle */
export const majorsData: MajorDetail[] = MAJORS

/** About section cards - full detail with subtitle */
export const aboutMajors: MajorDetail[] = MAJORS
