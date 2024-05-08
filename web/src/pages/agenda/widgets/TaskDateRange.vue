<script lang="ts">
import { defineComponent, computed, PropType } from 'vue';
import { updateTask } from '../../../api/agenda/request';
import { UpdateTask } from '../../../api/agenda/types';
import { DataTableItem } from 'vuestic-ui/web-components';

const formatDate = (dateString: string): Date => {
    if (!dateString) {
        return new Date();
    }
    return new Date(parseInt(dateString));
    //return new Date(dateString.replace('T', ' ').split('.')[0]);
};

export default defineComponent({
    props: {
        range: {
            type: String,
            required: true
        },
        task: {
            type: Object as PropType<UpdateTask | DataTableItem>,
            required: true
        }
    },
    data(props) {
        var start, end;
        const ranges = props.range.split(' - ');

        if (ranges.length > 1) {
            start = formatDate(ranges[0]);
            end = formatDate(ranges[1]);
        } else {
            const now = new Date();
            start = now;
            end = now;
        }

        var rangeDate = {
            start: start,
            end: end
        }


        return { rangeDate };
    },
    watch: {
        rangeDate: {
            handler(newValue) {
                if (!(newValue.start && newValue.end)) {
                    return;
                }
                var newTask: UpdateTask = {
                    task_id: this.task.task_id,
                    group_id: this.task.group_id,
                    name: this.task.name,
                    date: newValue.start.getTime() + " - " + newValue.end.getTime(),
                    expiration_date: this.task.expiration_date,
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
    <div class="task-date-holder">
        <VaDateInput v-model="rangeDate" />
    </div>
</template>