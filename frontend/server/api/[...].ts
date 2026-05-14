export default defineEventHandler(async (event) => {
  const path = event.path
  const query = getQuery(event)
  const target = `http://localhost:8080${path}`

  try {
    const body = event.method !== 'GET' && event.method !== 'HEAD'
      ? await readBody(event).catch(() => null)
      : undefined

    const data = await $fetch(target, {
      method: event.method,
      headers: { 'Content-Type': 'application/json' },
      query: Object.keys(query).length > 0 ? query : undefined,
      body,
    })
    return data
  } catch (error: any) {
    if (error?.response?.status) {
      setResponseStatus(event, error.response.status)
      return error.response._data
    }
    throw error
  }
})
