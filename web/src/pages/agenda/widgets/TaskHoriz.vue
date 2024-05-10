<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { RemoveTask } from '../../../api/agenda/types';
import { DataTableItem } from 'vuestic-ui/web-components';
import { useItems } from '../composables/useTasks';

const { ...itemsApi } = useItems()

export default defineComponent({
    props: {
        task: {
            type: Object as PropType<RemoveTask | DataTableItem>,
            required: true
        }
    },
    methods:{
        deleteTask(){
            const item:RemoveTask = {
                task_id:this.task.task_id,
            }
            itemsApi.removeTask(item);
        }
    }
})
</script>

<style>

.task-option-menu{
    opacity: 0;
    transition: .25s;
}

.task-option-menu:hover{
    opacity: 1;
}

</style>

<template>
    <div class="task-option flex align-center justify-center">
        <VaMenu>
            <template #anchor>
                <div class="task-option-menu">
                    <VaIcon name="more_horiz" color="secondary"></VaIcon>
                </div>
            </template>
            <VaMenuItem>
                <VaButton>
                    <VaIcon name="user" color="secondary"></VaIcon>
                    <span class="group-menu-item">Görüntüle</span>
                </VaButton>
            </VaMenuItem>
            <VaMenuItem>
                <VaButton>
                    <VaIcon name="shrink" color="secondary"></VaIcon>
                    <span class="group-menu-item">Taşı</span>
                </VaButton>
            </VaMenuItem>
            <VaMenuItem>
                <VaButton>
                    <VaIcon name="check" color="secondary"></VaIcon>
                    <span class="group-menu-item">Kopyala</span>
                </VaButton>
            </VaMenuItem>
            <VaSeparator></VaSeparator>
            <VaMenuItem>
                <VaButton @click="deleteTask()">
                    <VaIcon name="remove" color="secondary"></VaIcon>
                    <span class="group-menu-item">Sil</span>
                </VaButton>
            </VaMenuItem>
        </VaMenu>
    </div>
</template>
