<script setup lang="ts">
import { defineComponent, PropType } from 'vue';
import { RemoveTask, Task } from '../../../api/agenda/types';
import { DataTableItem } from 'vuestic-ui/web-components';
import { useItems } from '../composables/useTasks';

const props = defineProps({
    task: {
        type: Object as PropType<RemoveTask | DataTableItem>,
        required: true
    },
    index: {
        type: Number,
        required: true,
    },
});


const emit = defineEmits<{
    (event: 'delete-task', task_id:number): void
}>()


const deleteTask = (task_id:number) => {
    emit('delete-task',task_id);
}

</script>

<template>
    <div class="task-option flex align-center justify-center">
        <VaMenu>
            <template #anchor>
                <div class="task-option-menu">
                    <VaIcon name="more_horiz" color="secondary"></VaIcon>
                </div>
            </template>
            <VaMenuItem class="task-option">
                <VaButton @click="deleteTask(task.task_id)" preset="secondary">
                    <VaIcon name="remove" color="secondary"></VaIcon>
                    <span class="group-menu-item">Sil</span>
                </VaButton>
            </VaMenuItem>
        </VaMenu>
    </div>
</template>

<style>
.task-option-menu {
    opacity: 0;
    transition: .25s;
}

.task-option-menu:hover {
    opacity: 1;
}
</style>
