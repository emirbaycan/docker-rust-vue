<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { Task, TaskPriority, UpdateTask } from '../../../api/agenda/types';
import { DataTableItem } from 'vuestic-ui/web-components';
import { updateTask } from '../../../api/agenda/request';

export default defineComponent({
    props: {
        tasks: {
            type: Object as PropType<Array<Task>>,
            required: true
        }
    },
})
</script>

<style>

.task-group-option-menu{
    opacity: 0;
    transition: .25s;
}

.task-group-option-menu:hover{
    opacity: 1;
}

</style>

<template>
    <div class="task-group-option-holder flex align-center justify-center">
        <VaMenu class="task-group-options">
            <template #anchor>
                <div class="task-group-option-menu">
                    <VaIcon name="more_horiz" color="secondary"></VaIcon>
                </div>
            </template>
            <VaMenuItem class="task-option">
                <VaButton preset="secondary">
                    <VaIcon name="library_add" color="secondary"></VaIcon>
                    <span class="group-menu-item">Grup ekle</span>
                </VaButton>
            </VaMenuItem>
            <VaMenuItem class="task-option">
                <VaButton  preset="secondary">
                    <VaIcon name="delete" color="secondary"></VaIcon>
                    <span class="group-menu-item">Grubu Sil</span>
                </VaButton>
            </VaMenuItem>
        </VaMenu>
    </div>
</template>
