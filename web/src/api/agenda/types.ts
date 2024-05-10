export type TaskStatus = 0 | 1 | 2 | 3 | 4;

export type TaskPriority = 0 | 1 | 2 | 3 | 4;

export type AllTasks = {
    tasks: Array<Task>,
    groups: Array<TaskGroup>,
    updates: Array<TaskUpdate>,
    supervisors: Array<TaskSuperVisor>,
    visors: Array<TaskVisor>,
}

export type TaskUpdate = {
    task_id: number,
    update_id: number,
    user_id: number,
    text: string,
    created_at: string,
    updated_at: string,
}

export type CreateTaskUpdate = {
    user_id: number,
    text: string,
}

export type TaskVisor = {
    visor_id: number,
    task_id: number,
    email: string,
    fullname: string,
    avatar: string,
    created_at: string,
    updated_at: string,
}

export type CreateTaskVisor = {
    task_id: number,
    email: string,
}

export type UpdateTaskVisor = {
    visor_id: number,
    task_id: number,
    email: string,
}

export type TaskSuperVisor = {
    supervisor_id: number,
    task_id: number,
    email: string,
    fullname: string,
    avatar: string,
    created_at: string,
    updated_at: string,
}

export type CreateTaskSuperVisor = {
    task_id: number,
    email: string
}

export type UpdateTaskSuperVisor = {
    supervisor_id: number,
    task_id: number,
    email: string,
}

export type Task = {
    task_id: number
    group_id: number
    name: string
    updates: Array<TaskUpdate>
    supervisors: Array<TaskSuperVisor>,
    visors: Array<TaskVisor>,
    date: string,
    expiration_date: string,
    status: TaskStatus
    priority: TaskPriority
    created_at: string
    updated_at: string
}

export type UpdateTask = {
    task_id: number
    group_id: number
    name: string
    date: string,
    expiration_date: number,
    status: TaskStatus
    priority: TaskPriority
}

export type CreateTask = {
    group_id: number
    name: string
    date: string,
    expiration_date: string,
    status: TaskStatus
    priority: TaskPriority
}

export type DisplayTaskGroup = {
    group_id: number
    agenda_id: number
    title: string
    tasks: Array<Task>
}

export type TaskGroup = {
    group_id: number
    agenda_id: number
    title: string
    created_at: string
    updated_at: string
}

export type CollectedTaskGroup = {
    group_id: number
    agenda_id: number
    title: string
    tasks: Array<Task> 
}

export type UpdateTaskGroup = {
    group_id: number
    agenda_id: number
    title: string
}

export type CreateTaskGroup = {
    agenda_id: number,
    title: string
}

export type TaskAgenda = {
    agenda_id: number
    title: string
    user_id: string
    created_at: string
    updated_at: string
}

export type UpdateTaskAgenda = {
    agenda_id: number
    title: string
}

export type CreateTaskAgenda = {
    title: string
}
