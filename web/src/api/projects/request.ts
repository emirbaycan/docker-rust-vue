import { Project } from './types'

export type Filters = {
  search: string
}

// Simulate API calls
export type Pagination = {
  page: number
  perPage: number
  total: number
}

export type Sorting = {
  sortBy: keyof Array<Project>[number] | undefined
  sortingOrder: 'asc' | 'desc' | null
}

const admin_api_url = import.meta.env.VITE_API_URL + 'api/admin/'

export const getItems = async (filters: Partial<Filters & Pagination & Sorting>) => {
  const response = await fetch(admin_api_url + 'projects?page=' + filters.page + '&limit=' + filters.perPage + "&search=" + filters.search, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
  })

  const result = await response.json()

  const items: Array<Project> = result.items
  const count = result.count

  const { page = 1, perPage = 10 } = filters || {}
  return {
    data: items,
    pagination: {
      page: page,
      perPage: perPage,
      total: count,
    },
  }
}

export const addItem = async (project: Project) => {
  const response = await fetch(admin_api_url + 'projects', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(project),
  })

  const result = await response.json()
  const newItem: Project = result.item

  return {
    ...newItem,
  }
}

export const updateItem = async (project: Project) => {
  const response = await fetch(admin_api_url + 'projects/' + project.id, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(project),
  })

  const result = await response.json()
  const newItem: Project = result.item

  return newItem
}

export const removeItem = async (project: Project) => {
  const response = await fetch(admin_api_url + 'projects/' + project.id, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(project),
  })

  const result = await response.json()

  return result
}
