import { AllTasks, CreateTask, CreateTaskAgenda, CreateTaskGroup, CreateTaskSuperVisor, CreateTaskUpdate, CreateTaskAgendaVisor, RemoveTask, Task, TaskAgenda, TaskGroup, TaskSuperVisor, TaskUpdate, TaskAgendaVisor, UpdateTask, UpdateTaskAgendaTitle, UpdateTaskAgendaDescription, UpdateTaskGroup, UpdateTaskSuperVisor, UpdateTaskAgendaVisor } from './types';

export type AgendaFilters = {
    agenda_id: number,
}

export type TaskFilters = {
    task_id: number,
}

const user_api_url = import.meta.env.VITE_API_URL + 'api/user/'

export const getAllTasks = async (filters: Partial<AgendaFilters>) => {

    const response = await fetch(user_api_url +
        'all_tasks?agenda_id=' + filters.agenda_id, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const items: AllTasks = result.data

    return {
        data: items
    }
}

export const getTasks = async (filters: Partial<AgendaFilters>) => {

    const response = await fetch(user_api_url +
        'tasks?agenda_id=' + filters.agenda_id, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const items: Array<Task> = result.items

    return {
        data: items
    }
}

export const addTask = async (item: CreateTask) => {
    const response = await fetch(user_api_url + 'tasks', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: Task = result.data.item

    return {
        ...newItem,
    }
}

export const updateTask = async (item: UpdateTask) => {
    const response = await fetch(user_api_url + 'tasks/' + item.task_id, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: Task = result.item

    return newItem
}

export const removeTask = async (item: RemoveTask) => {
    const response = await fetch(user_api_url + 'tasks/' + item.task_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include'
    })

    const result = await response.json()

    return result
}

export const getTaskAgenda = async (filters: Partial<AgendaFilters>) => {

    const response = await fetch(user_api_url +
        'task_agendas/' + filters.agenda_id, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const item: TaskAgenda = result.item

    return item
}

export const getTaskAgendas = async () => {

    const response = await fetch(user_api_url +
        'task_agendas', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const items: Array<TaskAgenda> = result.items

    return {
        data: items
    }
}

export const addTaskAgenda = async (item: CreateTaskAgenda) => {
    const response = await fetch(user_api_url + 'task_agendas', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskAgenda = result.item

    return {
        ...newItem,
    }
}

export const updateTaskAgendaTitle = async (item: UpdateTaskAgendaTitle) => {
    const response = await fetch(user_api_url + 'task_agendas/' + item.agenda_id, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskAgenda = result.data.item

    return newItem
}

export const updateTaskAgendaDescription = async (item: UpdateTaskAgendaDescription) => {
    const response = await fetch(user_api_url + 'task_agendas/' + item.agenda_id, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskAgenda = result.data.item

    return newItem
}

export const removeTaskAgenda = async (item: TaskAgenda) => {
    const response = await fetch(user_api_url + 'task_agendas' + item.agenda_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()

    return result
}

export const getTaskGroups = async (filters: Partial<AgendaFilters>) => {

    const response = await fetch(user_api_url +
        'task_groups?agenda_id=' + filters.agenda_id, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const items: Array<TaskGroup> = result.items

    return {
        data: items
    }
}

export const addTaskGroup = async (item: CreateTaskGroup) => {
    const response = await fetch(user_api_url + 'task_groups', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskGroup = result.item

    return {
        ...newItem,
    }
}

export const updateTaskGroup = async (item: UpdateTaskGroup) => {
    const response = await fetch(user_api_url + 'tasks/groups/' + item.group_id, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskGroup = result.item

    return newItem
}

export const removeTaskGroup = async (item: TaskGroup) => {
    const response = await fetch(user_api_url + 'tasks/groups/' + item.group_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()

    return result
}

export const getTaskUpdates = async (filters: Partial<TaskFilters>) => {

    const response = await fetch(user_api_url + 'task_updates?task_id=' + filters.task_id, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const items: Array<TaskUpdate> = result.items

    return {
        data: items
    }
}

export const addTaskUpdate = async (item: CreateTaskUpdate) => {
    const response = await fetch(user_api_url + 'task_updates', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskUpdate = result.item

    return {
        ...newItem,
    }
}

export const removeTaskUpdate = async (item: TaskUpdate) => {
    const response = await fetch(user_api_url + 'task_updates?update_id=' + item.update_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()

    return result
}

export const getTaskAgendaVisors = async (filters: Partial<AgendaFilters>) => {

    const response = await fetch(user_api_url + 'task_agenda_visors?agenda_id=' + filters.agenda_id, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const items: Array<TaskAgendaVisor> = result.items

    return {
        data: items
    }
}

export const addTaskAgendaVisor = async (item: CreateTaskAgendaVisor) => {
    const response = await fetch(user_api_url + 'task_agenda_visors', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskAgendaVisor = result.item

    return {
        ...newItem,
    }
}

export const updateTaskAgendaVisor = async (item: UpdateTaskAgendaVisor) => {
    const response = await fetch(user_api_url + 'task_agenda_visors?visor_id=' + item.visor_id, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskAgendaVisor = result.item

    return newItem
}

export const removeTaskAgendaVisor = async (item: TaskAgendaVisor) => {
    const response = await fetch(user_api_url + 'task_agenda_visors?visor_id=' + item.visor_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()

    return result
}

export const getTaskSupervisors = async (filters: Partial<TaskFilters>) => {

    const response = await fetch(user_api_url + 'task_supervisors?task_id=' + filters.task_id, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const items: Array<TaskSuperVisor> = result.items

    return {
        data: items
    }
}


export const addTaskSupervisor = async (item: CreateTaskSuperVisor) => {
    const response = await fetch(user_api_url + 'task_supervisors', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskSuperVisor = result.item

    return {
        ...newItem,
    }
}

export const updateTaskSupervisor = async (item: UpdateTaskSuperVisor) => {
    const response = await fetch(user_api_url + 'task_supervisors?supervisor_id' + item.supervisor_id, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskSuperVisor = result.item

    return newItem
}

export const removeTaskSupervisor = async (item: TaskSuperVisor) => {
    const response = await fetch(user_api_url + 'task_supervisors?supervisor_id' + item.supervisor_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()

    return result
}
