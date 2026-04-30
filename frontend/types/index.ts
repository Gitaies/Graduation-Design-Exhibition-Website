export interface WorkItem {
  id: string
  title: string
  majorCode: 'software' | 'electronic' | 'broadcast'
  majorName: string
  theme: string
  tags: string[]
  authors: string[]
  teacher: string
  posterUrl: string
  videoUrl: string
  duration: string
  introduction: string
}

export interface MajorInfo {
  code: string
  name: string
  theme: string
  subtitle: string
  description: string
}

export interface InteractionData {
  work_id: string
  like_count: number
  comment_count: number
  liked_by_me: boolean
}

export interface Comment {
  id: string
  work_id: string
  visitor_name: string
  content: string
  created_at: string
}

export interface RankingItem {
  rank: number
  work_id: string
  like_count: number
}
