<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { TaskStatus, UpdateTask } from '../../../api/agenda/types';
import { DataTableItem } from 'vuestic-ui/web-components';
import { updateTask } from '../../../api/agenda/request';

export default defineComponent({
    props: {
        status: {
            type: Number as PropType<TaskStatus>,
            required: true,
        },
        task: {
            type: Object as PropType<UpdateTask | DataTableItem>,
            required: true
        }
    },
    data() {
        return {
            taskStatusClasses: {
                1: 'secondary',
                2: 'warning',
                3: 'danger',
                4: 'primary'
            } as Record<number, string>,
            taskStatusTexts: {
                1: 'Başlamadı',
                2: 'Yapılmakta',
                3: 'Takılı',
                4: 'Tamamlandı'
            } as Record<number, string>,
            selectedStatus: null as number | null 
        };
    },
    methods: {
        statusColor(status: number) {
            return this.taskStatusClasses[status] || '';
        },
        statusName(status: number) {
            return this.taskStatusTexts[status] || '';
        },
        selectStatus(status: number) {
            this.selectedStatus = status;
            this.$emit('statusSelected', status); 

            const newTask: UpdateTask = {
                task_id: this.task.task_id,
                group_id: this.task.group_id,
                name: this.task.name,
                date: this.task.date,
                expiration_date: this.task.expiration_date,
                status: parseInt(status.toString()) as TaskStatus,
                priority:  this.task.priority,
            };

            updateTask(newTask);
        }
    }
})
</script>

<template>
    <div class="task-status-holder">
        <va-menu v-if="task.status">
            <template #anchor>
                <va-button :color="statusColor(selectedStatus || task.status)" >
                    {{ statusName(selectedStatus || task.status) }}
                </va-button>
            </template>
            <va-menu-item v-for="(value, key) in taskStatusTexts" :key="key">
                <va-button :color="statusColor(key)" class="task-status" @click="selectStatus(key)">
                    {{ value }}
                </va-button>
            </va-menu-item>
        </va-menu>
    </div>
</template>