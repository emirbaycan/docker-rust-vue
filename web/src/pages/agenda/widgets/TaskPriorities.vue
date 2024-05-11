<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { TaskPriority, UpdateTask } from '../../../api/agenda/types';
import { DataTableItem } from 'vuestic-ui/web-components';
import { updateTask } from '../../../api/agenda/request';

export default defineComponent({
    props: {
        priority: {
            type: Number as PropType<TaskPriority>, // Assuming TaskPriority is a number type
            required: true,
        },
        task: {
            type: Object as PropType<UpdateTask | DataTableItem>,
            required: true
        }
    },
    data() {
        return {
            taskPriorityClasses: {
                1: 'secondary',
                2: 'info',
                3: 'warning',
                4: 'danger'
            } as Record<number, string>,
            taskPriorityTexts: {
                1: 'Düşük',
                2: 'Normal',
                3: 'Yüksek',
                4: 'Kritik'
            } as Record<number, string>,
            selectedPriority: null as number | null 
        };
    },
    methods: {
        priorityColor(priority: number) {
            return this.taskPriorityClasses[priority] || '';
        },
        priorityName(priority: number) {
            return this.taskPriorityTexts[priority] || '';
        },
        selectPriority(priority: number) {
            this.selectedPriority = priority;
            this.$emit('prioritySelected', priority); 

            const newTask: UpdateTask = {
                task_id: this.task.task_id,
                group_id: this.task.group_id,
                name: this.task.name,
                date: this.task.date,
                expiration_date: this.task.expiration_date,
                status: this.task.status,
                priority: parseInt(priority.toString()) as TaskPriority, 
            };

            updateTask(newTask);
        }
    }
})
</script>

<template>
    <div class="task-priority-holder">
        <va-menu v-if="task.priority">
            <template #anchor>
                <va-button :color="priorityColor(selectedPriority || task.priority)" >
                    {{ priorityName(selectedPriority || task.priority) }}
                </va-button>
            </template>
            <va-menu-item v-for="(value, key) in taskPriorityTexts" :key="key">
                <va-button :color="priorityColor(key)" class="task-priority" @click="selectPriority(key)">
                    {{ value }}
                </va-button>
            </va-menu-item>
        </va-menu>
    </div>
</template>