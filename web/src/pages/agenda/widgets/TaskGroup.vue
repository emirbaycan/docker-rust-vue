<script setup lang="ts">
import { ref } from 'vue'
import Tasks from './Tasks.vue';
import { DataTableItem } from 'vuestic-ui'
import { PropType } from 'vue';
import { CollectedTaskGroup, CreateTask, Task, TaskGroup, UpdateTaskGroup } from '../../../api/agenda/types';
import { addTask, removeTask, updateTaskGroup } from '../../../api/agenda/request';

const props = defineProps({
    loading: {
        type: Boolean as PropType<boolean>,
    },
    group: {
        type: Object as PropType<CollectedTaskGroup>,
        required: true,
    },
})

const emit = defineEmits<{
    (event: 'update-task-group-title',value:string, group_id:number): void
    (event: 'add-task-group'): void    
    (event: 'delete-task-group', group_id: number): void
    (event: 'add-task', item: Task | DataTableItem): void
    (event: 'edit-task', item: Task): void
    (event: 'delete-task', task_id: number): void
}>()

const addNewTaskGroup = () => {
    emit('add-task-group');
}

const deleteTaskGroup = (group_id: number) => {
    emit('delete-task-group', group_id);
}

const updateTaskGroupTitle = (value:string, group_id: number) => {
    emit('update-task-group-title', value, group_id);
}

const deleteTask = async (task_id: number) => {
    if (!tasks || !tasks.value) {
        return;
    }

    const updatedTasks = tasks.value.filter(task => task.task_id !== task_id);

    tasks.value = updatedTasks;

    await removeTask({task_id:task_id});
}

const addNewTask = async (task: Task | DataTableItem) => {

    if (!task.name || task.name.length < 3) {
        return;
    }

    var now = new Date().getTime().toString();

    const newTask: CreateTask = {
        group_id: task.group_id,
        name: task.name,
        date: now,
        expiration_date: now + " - " + now,
        status: 1,
        priority: 1,
    };

    var completeNewTask = await addTask(newTask);

    const lastIndex = tasks.value.length - 1;

    // Insert the new task before the last task
    tasks.value.splice(lastIndex, 0, {
        ...completeNewTask,
        updates: [],
        supervisors: []
    });
};

var tasks = ref(props.group.tasks);
var group_title = ref(props.group.title);

</script>

<template>
    <div class="task-group">
        <div class="task-group-holder">
            <VaCardTitle class="task-group-title">
                <VaInput class="input-no-border" v-model="group_title" @blur="updateTaskGroupTitle($event.target.value, props.group.group_id)" />
            </VaCardTitle>
            <VaCard class="tasks">
                <Tasks v-if="!loading" :tasks="tasks" @delete-task="deleteTask" @add-task-group="addNewTaskGroup" @delete-task-group="deleteTaskGroup" :loading="loading"
                    @add-task="addNewTask"></Tasks>
            </VaCard>
        </div>
    </div>
</template>

<style lang="scss">
.task-group {
    display: flex;
    width: 100%;

    .task-group-holder {
        width: 100%;
    }
}
</style>