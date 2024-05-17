<script setup lang="ts">
import { PropType } from 'vue';
import { CollectedTaskGroup, CreateTaskGroup } from '../../../api/agenda/types';
import TaskGroup from './TaskGroup.vue';

defineProps({
    loading: {
        type: Boolean as PropType<boolean>,
    },
    groups: {
        type: Object as PropType<Array<CollectedTaskGroup>>,
    },
})

const emit = defineEmits<{
    (event: 'update-task-group-title',value:string, group_id:number): void
    (event: 'add-task-group'): void
    (event: 'delete-task-group', group_id: number): void
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

</script>

<style lang="scss">
.task-groups {
    display: flex;
}
</style>

<template>
    <div class="task-groups" v-for="group in groups">
        <TaskGroup :group="group" :loading="loading" @add-task-group="addNewTaskGroup"
            @delete-task-group="deleteTaskGroup" @update-task-group-title="updateTaskGroupTitle"></TaskGroup>
    </div>
</template>
