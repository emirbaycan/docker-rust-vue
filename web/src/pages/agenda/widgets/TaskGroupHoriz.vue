<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { Task } from '../../../api/agenda/types';
import { DataTableCell } from 'vuestic-ui/web-components';

export default defineComponent({
    props: {
        tasks: {
            type: Object as PropType<Array<Task>>,
            required: true
        }
    },
    setup(props, { emit }) {

        const addNewTaskGroup = () => {
            emit('add-task-group');
        }

        const deleteTaskGroup = () =>{
            emit('delete-task-group', props.tasks[0].group_id);
        }

        return {
            addNewTaskGroup,
            deleteTaskGroup
        };
    }
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
                <VaButton preset="secondary" @click="addNewTaskGroup">
                    <VaIcon name="library_add" color="secondary"></VaIcon>
                    <span class="group-menu-item">Grup ekle</span>
                </VaButton>
            </VaMenuItem>
            <VaMenuItem class="task-option">
                <VaButton preset="secondary" @click="deleteTaskGroup">
                    <VaIcon name="delete" color="secondary"></VaIcon>
                    <span class="group-menu-item">Grubu Sil</span>
                </VaButton>
            </VaMenuItem>
        </VaMenu>
    </div>
</template>
