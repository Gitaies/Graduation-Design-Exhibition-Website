// 作品数据类型定义（与后端 API 返回格式匹配）
export interface Work {
  id: string
  title: string
  major_code: string
  major_name: string
  theme: string
  tags: string[]
  authors: string[]
  teacher: string
  poster_url: string
  video_url: string
  duration: string
  introduction: string
  like_count: number
  comment_count: number
  created_at: string
  updated_at: string
}

export interface WorkListResponse {
  items: Work[]
  page: number
  page_size: number
  total: number
  total_pages: number
}
