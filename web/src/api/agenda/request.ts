import { CreateTask, CreateTaskAgenda, CreateTaskGroup, CreateTaskSuperVisor, CreateTaskVisor, Task, TaskAgenda, TaskGroup, TaskSuperVisor, TaskUpdate, TaskVisor, UpdateTask, UpdateTaskAgenda, UpdateTaskGroup, UpdateTaskSuperVisor, UpdateTaskVisor } from './types';

// Simulate API calls

export type Sorting = {
    sortBy: keyof Task | undefined
    sortingOrder: 'asc' | 'desc' | null
}

export type Filters = {
    agenda_id: number,    
}

const user_api_url = import.meta.env.VITE_API_URL + 'api/user/'

export const getTasks = async (filters: Partial<Filters & Sorting>) => {

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

export const getTaskGroups = async () => {

    const response = await fetch(user_api_url +
        'tasks/groups', {
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

export const getTaskVisors = async () => {

    const response = await fetch(user_api_url +
        'tasks/visors', {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()
    const items: Array<TaskVisor> = result.items

    return {
        data: items
    }
}

export const getTaskSupervisors = async () => {

    const response = await fetch(user_api_url +
        'tasks/supervisors', {
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

export const getTaskAgendas = async () => {

    const response = await fetch(user_api_url +
        'tasks/agendas', {
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
    const newItem: Task = result.item

    return {
        ...newItem,
    }
}

export const addTaskGroup = async (item: CreateTaskGroup) => {
    const response = await fetch(user_api_url + 'tasks/groups', {
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

export const addTaskSupervisor = async (item: CreateTaskSuperVisor) => {
    const response = await fetch(user_api_url + 'tasks/supervisors', {
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

export const addTaskVisor = async (item: CreateTaskVisor) => {
    const response = await fetch(user_api_url + 'tasks/visors', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskVisor = result.item

    return {
        ...newItem,
    }
}

export const addTaskAgenda = async (item: CreateTaskAgenda) => {
    const response = await fetch(user_api_url + 'tasks/agendas', {
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

export const updateTaskSupervisor = async (item: UpdateTaskSuperVisor) => {
    const response = await fetch(user_api_url + 'tasks/supervisors/' + item.supervisor_id, {
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

export const updateTaskVisor = async (item: UpdateTaskVisor) => {
    const response = await fetch(user_api_url + 'tasks/visors/' + item.visor_id, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskVisor = result.item

    return newItem
}

export const updateTaskAgenda = async (item: UpdateTaskAgenda) => {
    const response = await fetch(user_api_url + 'tasks/agendas/' + item.agenda_id, {
        method: 'PATCH',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
        body: JSON.stringify(item),
    })

    const result = await response.json()
    const newItem: TaskAgenda = result.item

    return newItem
}
 
export const removeTask = async (item: Task) => {
    const response = await fetch(user_api_url + 'tasks/' + item.id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include'
    })

    const result = await response.json()

    return result
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

export const removeTaskSupervisor = async (item: TaskSuperVisor) => {
    const response = await fetch(user_api_url + 'tasks/supervisors/' + item.supervisor_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()

    return result
}

export const removeTaskVisor = async (item: TaskVisor) => {
    const response = await fetch(user_api_url + 'tasks/visors/' + item.visor_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()

    return result
}

export const removeTaskAgenda = async (item: TaskVisor) => {
    const response = await fetch(user_api_url + 'tasks/agendas/' + item.visor_id, {
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json',
        },
        credentials: 'include',
    })

    const result = await response.json()

    return result
}
