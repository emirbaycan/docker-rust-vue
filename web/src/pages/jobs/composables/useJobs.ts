import { Ref, ref, unref } from 'vue'
import { getItems, addItem, updateItem, removeItem,
  type Filters, Sorting, Pagination } from '../../../api/jobs/request'
import { Job } from '../../../api/jobs/types'
import { watchIgnorable } from '@vueuse/core'

const makePaginationRef = () => ref<Pagination>({ page: 1, perPage: 10, total: 0 })
const makeSortingRef = () => ref<Sorting>({ sortBy: 'created_at', sortingOrder: 'desc' })
const makeFiltersRef = () => ref<Partial<Filters>>({ search: '' })

export const useItems = (options?: {
  sorting?: Ref<Sorting>;
  pagination?: Ref<Pagination>;
  filters?: Ref<Partial<Filters>>
}) => {
  const isLoading = ref(false)
  const items = ref<Job[]>([])

  const { filters = makeFiltersRef(), sorting = makeSortingRef(), pagination = makePaginationRef() } = options ?? {}

  const fetch = async () => {
    isLoading.value = true
    const { data, pagination: newPagination } = await getItems({
      ...unref(filters),
      ...unref(sorting),
      ...unref(pagination),
    })
    items.value = data as Job[]

    ignoreUpdates(() => {
      pagination.value = newPagination
    })

    isLoading.value = false
  }

  const { ignoreUpdates } = watchIgnorable([pagination, sorting], fetch, { deep: true })

  fetch()

  return {
    isLoading,

    filters,
    pagination,
    sorting,

    items,

    fetch,

    async add(item: Job) {
      isLoading.value = true
      await addItem({
        ...item,
      })
      await fetch()
      isLoading.value = false
    },

    async update(item: Job) {
      isLoading.value = true
      await updateItem({
        ...item,
      })
      await fetch()
      isLoading.value = false
    },

    async remove(item: Job) {
      isLoading.value = true
      await removeItem({
        ...item,
      })
      await fetch()
      isLoading.value = false
    },

  }
}
