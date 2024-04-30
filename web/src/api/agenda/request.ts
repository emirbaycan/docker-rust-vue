import { CreateTask, CreateTaskGroup, NewTaskVisor, Task, TaskGroup, TaskUpdate, TaskVisor, UpdateTask, UpdateTaskGroup } from './types';

// Simulate API calls

export type Sorting = {
    sortBy: keyof Task | undefined
    sortingOrder: 'asc' | 'desc' | null
}

export type Filters = {
    date: string,
    project_id: number,
    visor: number,
}

const user_api_url = import.meta.env.VITE_API_URL + 'api/user/'

export const getItems = async (filters: Partial<Filters & Sorting>) => {

    const response = await fetch(user_api_url +
        'tasks?project_id=' + filters.project_id +
        "&date=" + filters.date +
        "&visor=" + filters.visor, {
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

export const addTaskGroup = async (item: CreateTaskGroup) => {
    const response = await fetch(user_api_url + 'tasks/add_group', {
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

export const addTask = async (item: CreateTask) => {
    const response = await fetch(user_api_url + 'tasks/add', {
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

export const updateTask = async (item: UpdateTask) => {
    const response = await fetch(user_api_url + 'tasks/' + item.id, {
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
    const response = await fetch(user_api_url + 'tasks/' + item.id, {
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

export const removeTask = async (item: Task) => {
    const response = await fetch(user_api_url + 'tasks/' + item.id, {
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


export const removeTaskGroup = async (item: TaskGroup) => {
    const response = await fetch(user_api_url + 'tasks/group' + item.id, {
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
