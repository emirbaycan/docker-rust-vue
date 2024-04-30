export type TaskStatus = 1 | 2 | 3 | 4;

export type TaskPriority = 1 | 2 | 3 | 4;

export type TaskUpdate = {
    id: number,
    writer_id: number,
    text: string,
    created_at: string,
    updated_at: string,
}

export type NewTaskUpdate = {
    writer_id: number,
    text: string,
}

export type TaskVisor = {
    id: number,
    project_id: number,
    user_id: number,
    fullname: string,
    created_at: string,
    updated_at:string,
}

export type NewTaskVisor = {
    project_id: number,
    user_id: number,
    fullname: string,
}

export type Task = {
    id: number
    group_id: number
    name: string
    updates: Array<TaskUpdate>
    supervisors: Array<TaskVisor>,
    visors: Array<TaskVisor>,
    date: string,
    expiration_date: string,
    status: TaskStatus
    priority: TaskPriority
    created_at: string
    updated_at: string
}

export type UpdateTask = {
    id: number
    group_id: number
    name: string
    updates: Array<TaskUpdate>
    supervisors: Array<TaskVisor>,
    visors: Array<TaskVisor>,
    date: string,
    expiration_date: string,
    status: TaskStatus
    priority: TaskPriority
}

export type CreateTask = {
    group_id: number
    name: string
    updates: Array<TaskUpdate>
    supervisors: Array<TaskVisor>,
    visors: Array<TaskVisor>,
    date: string,
    expiration_date: string,
    status: TaskStatus
    priority: TaskPriority
}

export type TaskGroup = {
    id: number
    project_id: number
    owner_id: number,
    title: string
    created_at: string
    updated_at: string
}

export type UpdateTaskGroup = {
    id: number
    project_id: number
    owner_id: number,
    title: string
}

export type CreateTaskGroup = {
    project_id: number
    owner_id: number,
    title: string
}

