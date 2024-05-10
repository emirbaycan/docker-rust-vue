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
    <div class="task-group-option flex align-center justify-center">
        <VaMenu>
            <template #anchor>
                <div class="task-group-option-menu">
                    <VaIcon name="more_horiz" color="secondary"></VaIcon>
                </div>
            </template>
            <VaMenuItem>
                <VaButton>
                    <VaIcon name="user" color="secondary"></VaIcon>
                    <span class="group-menu-item">Bu grubu daralt</span>
                </VaButton>
            </VaMenuItem>
            <VaMenuItem>
                <VaButton>
                    <VaIcon name="shrink" color="secondary"></VaIcon>
                    <span class="group-menu-item">Tüm grupları daralt</span>
                </VaButton>
            </VaMenuItem>
            <VaMenuItem>
                <VaButton>
                    <VaIcon name="check" color="secondary"></VaIcon>
                    <span class="group-menu-item">Tümünü seç</span>
                </VaButton>
            </VaMenuItem>
            <VaSeparator></VaSeparator>
            <VaMenuItem>
                <VaButton>
                    <VaIcon name="plus" color="secondary"></VaIcon>
                    <span class="group-menu-item">Grup ekle</span>
                </VaButton>
            </VaMenuItem>
            <VaMenuItem>
                <VaButton>
                    <VaIcon name="copy" color="secondary"></VaIcon>
                    <span class="group-menu-item">Grubu çoğalt</span>
                </VaButton>
            </VaMenuItem>
        </VaMenu>
    </div>
</template>
