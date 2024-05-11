<script setup lang="ts">
import { ref } from 'vue'
import Tasks from './Tasks.vue';
import { useItems } from '../composables/useTasks'
import { DataTableItem, useModal, useToast } from 'vuestic-ui'

import { PropType } from 'vue';
import { CollectedTaskGroup, CreateTask, Task, TaskGroup } from '../../../api/agenda/types';
import { addTask } from '../../../api/agenda/request';

const props = defineProps({
    loading:{
        type:Boolean as boolean,
        required:true
    },
    group: {
        type: Object as PropType<CollectedTaskGroup>,
        required: true,
    },
})

const emit = defineEmits<{
    (event: 'add-task', item: Task | DataTableItem): void
    (event: 'edit-task', item: Task): void
    (event: 'delete-task', index: number): void
}>()

const deleteTask = (index: number) => {
    tasks.value.splice(index, 1);
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
        updates:[],
        visors:[],
        supervisors:[]
    });
};

var tasks = ref(props.group.tasks);

</script>

<template>
    <div class="task-group">
        <div class="task-group-holder">
            <VaCardTitle class="task-group-title">
                {{ group.title }}
            </VaCardTitle>
            <VaCard class="tasks">
                <Tasks v-if="!loading" :tasks="tasks" @delete-task="deleteTask" :loading="loading" @add-task="addNewTask"></Tasks>
            </VaCard>
        </div>
    </div>
</template>

<style lang="scss">
.task-group {
    display: flex;

}
</style>