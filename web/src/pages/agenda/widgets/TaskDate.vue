<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { DataTableItem } from 'vuestic-ui/web-components';
import { updateTask } from '../../../api/agenda/request';
import { UpdateTask } from '../../../api/agenda/types';

const formatDate = (dateString: string): Date => {
    if (!dateString) {
        return new Date();
    }
    return new Date(parseInt(dateString));
    //return new Date(dateString.replace('T', ' ').split('.')[0]);
};

export default defineComponent({
    props: {
        date: {
            type: String,
            required: true
        },
        task: {
            type: Object as PropType<UpdateTask | DataTableItem>,
            required: true
        }
    },
    data() {
        return {
            single: formatDate(this.date),
        };
    },
    watch: {
        single: {
            handler(newValue) {
                var newTask: UpdateTask = {
                    task_id: this.task.task_id,
                    group_id: this.task.group_id,
                    name: this.task.name,
                    date: this.task.date,
                    expiration_date: newValue.getTime().toString(),
                    status: this.task.status,
                    priority: this.task.priority,
                };
                updateTask(newTask);
            },
            deep: true // This is to watch for nested changes in rangeDate object
        }
    }
});
</script>

<template>
    <div class="task-expiration-date-holder">
        <VaDateInput v-model="single" v-if="task.expiration_date"/>
    </div>
</template>