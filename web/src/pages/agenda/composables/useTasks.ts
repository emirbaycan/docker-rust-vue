import { Ref, ref, unref, watch } from 'vue'
import { 
    getTasks, 
    addTask, addTaskGroup, addTaskSupervisor, addTaskVisor, addTaskAgenda,
    updateTask, updateTaskGroup, updateTaskSupervisor, updateTaskVisor, updateTaskAgenda,
    removeTask, removeTaskGroup, removeTaskSupervisor, removeTaskVisor, removeTaskAgenda,
    type Filters, Sorting, 
    getAllTasks} from '../../../api/agenda/request'
import { AllTasks, Task } from '../../../api/agenda/types'

const makeFiltersRef = () => ref<Partial<Filters>>({ agenda_id: 1 })
const makeSortingRef = () => ref<Sorting>({ sortBy: 'name', sortingOrder: null })

export const useItems = (options?: {
  sorting?: Ref<Sorting>
  filters?: Ref<Partial<Filters>>
}) => {
  const isLoading = ref(false)
  const items = ref<AllTasks>()

  const { filters = makeFiltersRef(), sorting = makeSortingRef()} = options || {}

  const fetch = async () => {
    isLoading.value = true
    const { data } = await getAllTasks({
      ...unref(filters),
    })
    items.value = data

    isLoading.value = false
  }

  watch(
    filters,
    () => {
      fetch()
    },
    { deep: true },
  )

  fetch()

  return {
    isLoading,

    sorting,
    filters,

    items,

    fetch,

    async addTask(task: Task) {
      isLoading.value = true
      await addTask(task)
      await fetch()
      isLoading.value = false
    },

    async updateTask(task: Task) {
      isLoading.value = true
      await updateTask(task)
      await fetch()
      isLoading.value = false
    },

    async removeTask(task: Task) {
      isLoading.value = true
      await removeTask(task)
      await fetch()
      isLoading.value = false
    },
  }
}
