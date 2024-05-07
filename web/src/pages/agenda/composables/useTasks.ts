import { Ref, ref, unref, watch } from 'vue'
import { 
    getTasks, 
    addTask, addTaskGroup, addTaskSupervisor, addTaskVisor, addTaskAgenda,
    updateTask, updateTaskGroup, updateTaskSupervisor, updateTaskVisor, updateTaskAgenda,
    removeTask, removeTaskGroup, removeTaskSupervisor, removeTaskVisor, removeTaskAgenda,
    getAllTasks,
    AgendaFilters,
    TaskFilters} from '../../../api/agenda/request'
import { AllTasks, Task } from '../../../api/agenda/types'

const makeFiltersRef = () => ref<Partial<AgendaFilters & TaskFilters>>({ agenda_id: 1, task_id: 0 })

export const useItems = (options?: {
  filters?: Ref<Partial<AgendaFilters & TaskFilters>>
}) => {
  const isLoading = ref(false)
  const items = ref<AllTasks>()

  const { filters = makeFiltersRef()} = options || {}

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
