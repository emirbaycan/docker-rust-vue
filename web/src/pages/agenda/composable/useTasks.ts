import { Ref, ref, unref, watch } from 'vue'
import { 
    getTasks, 
    addTask, addTaskGroup, addTaskSupervisor, addTaskVisor, addTaskAgenda,
    updateTask, updateTaskGroup, updateTaskSupervisor, updateTaskVisor, updateTaskAgenda,
    removeTask, removeTaskGroup, removeTaskSupervisor, removeTaskVisor, removeTaskAgenda,
    type Filters, Sorting } from '../../../api/agenda/request'
import { Task } from '../../../api/agenda/types'

const makeFiltersRef = () => ref<Partial<Filters>>({ agenda_id: 1 })

export const useItems = (options?: {
  sorting?: Ref<Sorting>
  filters?: Ref<Partial<Filters>>
}) => {
  const isLoading = ref(false)
  const tasks = ref<Task[]>([])

  const { filters = makeFiltersRef()} = options || {}

  const fetch = async () => {
    isLoading.value = true
    const { data } = await getTasks({
      ...unref(filters),
    })
    tasks.value = data

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

    filters,

    tasks,

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
