import { User } from './types'

// Simulate API calls

export type Pagination = {
  page: number
  perPage: number
  total: number
}

export type Sorting = {
  sortBy: keyof User | undefined
  sortingOrder: 'asc' | 'desc' | null
}

export type Filters = {
  search: string
}

const admin_api_url = import.meta.env.VITE_API_URL + 'api/admin/'

export const getItems = async (filters: Partial<Filters & Pagination & Sorting>) => {

  const response = await fetch(admin_api_url + 'users', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
  })

  const result = await response.json()

  const items: Array<User> = result.items
  const count = result.count

  const { page = 1, perPage = 10 } = filters || {}
  return {
    data: items,
    pagination: {
      page,
      perPage,
      total: count,
    },
  }
}

export const addItem = async (item: User) => {
  const response = await fetch(admin_api_url + 'users', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(item),
  })

  const result = await response.json()
  const newItem: User = result.item

  return {
    ...newItem,
  }
}

export const updateItem = async (item: User) => {
  const response = await fetch(admin_api_url + 'users/' + item.id, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(item),
  })

  const result = await response.json()
  const newItem: User = result.item

  return newItem
}

export const removeItem = async (item: User) => {
  const response = await fetch(admin_api_url + 'users/' + item.id, {
    method: 'DELETE',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(item),
  })

  const result = await response.json()

  return result
}
