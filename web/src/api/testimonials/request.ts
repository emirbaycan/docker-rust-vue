import { Testimonial } from './types'

// Simulate API calls
export type Pagination = {
  page: number
  perPage: number
  total: number
}

export type Sorting = {
  sortBy: keyof Array<Testimonial>[number] | undefined
  sortingOrder: 'asc' | 'desc' | null
}

const admin_api_url = import.meta.env.VITE_API_URL + 'api/admin/'

export const getItems = async (options: Sorting & Pagination) => {
  const response = await fetch(admin_api_url + 'testimonials', {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
  })

  const result = await response.json()

  const items: Array<Testimonial> = result.items
  const count = result.count
 
  return {
    data: items,
    pagination: {
      page: options.page,
      perPage: options.perPage,
      total: count,
    },
  }
}

export const addItem = async (item: Testimonial) => {
  const response = await fetch(admin_api_url + 'testimonials', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(item),
  })

  const result = await response.json()
  const newItem: Testimonial = result.item

  return {
    ...newItem,
  }
}

export const updateItem = async (item: Testimonial) => {
  const response = await fetch(admin_api_url + 'testimonials/' + item.id, {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
    },
    credentials: 'include',
    body: JSON.stringify(item),
  })

  const result = await response.json()
  const newItem: Testimonial = result.item

  return newItem
}

export const removeItem = async (item: Testimonial) => {
  const response = await fetch(admin_api_url + 'testimonials/' + item.id, {
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
