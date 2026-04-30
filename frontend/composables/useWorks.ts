import type { Work, WorkListResponse } from '~/types/work'

export const useWorks = () => {
  const config = useRuntimeConfig()
  const apiBase = config.public.apiBase

  const fetchWorks = async (params?: {
    page?: number
    page_size?: number
    major?: string
  }): Promise<WorkListResponse> => {
    const query = new URLSearchParams()
    if (params?.page) query.append('page', params.page.toString())
    if (params?.page_size) query.append('page_size', params.page_size.toString())
    if (params?.major) query.append('major', params.major)

    const url = `${apiBase}/works${query.toString() ? '?' + query.toString() : ''}`
    const response = await $fetch<{ code: number; message: string; data: WorkListResponse }>(url)

    if (response.code !== 0) {
      throw new Error(response.message)
    }

    return response.data
  }

  const fetchWorkDetail = async (id: string): Promise<Work> => {
    const response = await $fetch<{ code: number; message: string; data: Work }>(
      `${apiBase}/works/${id}`
    )

    if (response.code !== 0) {
      throw new Error(response.message)
    }

    return response.data
  }

  return {
    fetchWorks,
    fetchWorkDetail,
  }
}
