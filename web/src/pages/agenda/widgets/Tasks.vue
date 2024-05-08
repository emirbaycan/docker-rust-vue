<script setup lang="ts">
import { ref, computed, PropType } from 'vue';
import { DataTableItem, defineVaDataTableColumns } from 'vuestic-ui'
import { Task, TaskStatus, TaskUpdate, UpdateTask } from '../../../api/agenda/types';
import { useVModel } from '@vueuse/core';
import TaskDate from './TaskDate.vue';
import TaskDateRange from './TaskDateRange.vue';
import TaskPriorities from './TaskPriorities.vue';
import TaskStatuses from './TaskStatuses.vue';
import { updateTask } from '../../../api/agenda/request';
import { Value } from 'sass';

const columns = defineVaDataTableColumns([
    { key: 'selection' },
    { label: 'Görev', key: 'name', sortable: true },
    { label: 'Sorumlu', key: 'supervisor', sortable: true },
    { label: 'Durum', key: 'status', sortable: true },
    { label: 'Öncelik', key: 'priority', sortable: true },
    { label: 'Zaman Aralığı', key: 'date', sortable: true },
    { label: 'Son Tarih', key: 'expiration_date', sortable: true },
])

const props = defineProps({
    tasks: {
        type: Object as PropType<Array<Task>>,
        required: true,
    },
    loading: {
        type: Boolean,
        required: true,
    }
})

const emit = defineEmits<{
    (event: 'edit', item: Task): void
    (event: 'delete', item: Task): void
}>()

const defaultCheckboxes = ref<boolean[]>(Array(props.tasks.length).fill(false));
const selectedTasks = ref<number[]>([]);

const toggleTaskSelection = (taskId: number) => {
    const index = selectedTasks.value.indexOf(taskId);
    if (index === -1) {
        selectedTasks.value.push(taskId);
    } else {
        selectedTasks.value.splice(index, 1);
    }
};

const toggleAllTasks = (event: Event) => {
    const target = event.target as HTMLInputElement;
    const isChecked = target.checked;
    if (isChecked) {
        selectedTasks.value = props.tasks.map(task => task.task_id);
    } else {
        selectedTasks.value = [];
    }
};

const selectAll = computed({
    get: () => defaultCheckboxes.value.every(checkbox => checkbox),
    set: (value: boolean) => {
        defaultCheckboxes.value = defaultCheckboxes.value.map(() => value);
    }
});


const updateTaskTitle = (task: Task | DataTableItem) => {

    const newTask: UpdateTask = {
        task_id: task.task_id,
        group_id: task.group_id,
        name: task.name,
        date: task.date,
        expiration_date: task.expiration_date,
        status: task.status,
        priority: task.priority,
    };

    updateTask(newTask);
};

</script>

<template>
    <VaDataTable :columns="columns" :items="tasks" :loading="$props.loading">

        <template #header(selection)="{ label }">
            <div class="flex align-center justify-center">
                <VaCheckbox v-model="selectAll" @change="toggleAllTasks"></VaCheckbox>
            </div>
        </template>

        <template #cell(selection)="{ rowData }">
            <div class="flex align-center justify-center">
                <VaCheckbox v-model="defaultCheckboxes[rowData.task_id]" @change="toggleTaskSelection(rowData.task_id)">
                </VaCheckbox>
            </div>
        </template>

        <template #cell(name)="{ rowData }">
            <div class="task-holder">
                <div class="task pr-2">
                    <VaInput v-model="rowData.name" @blur="updateTaskTitle(rowData)"></VaInput>
                </div>
                <div class="task-updates">
                    <VaButton v-if="rowData.updates.length" class="task-update-btn">
                        <VaIcon name="comment" class="va-text-secondary"></VaIcon>
                        <span class="task-update-count">{{ rowData.updates.length }}</span>
                    </VaButton>
                    <VaButton v-else class="task-update-btn">
                        <VaIcon name="plus" class="va-text-secondary"></VaIcon>
                    </VaButton>
                </div>
            </div>
        </template>

        <template #cell(supervisor)="{ rowData }">
            <div class="task-visor-holder">
                <VaButton v-if="rowData.supervisors.length" class="task-update-btn">
                    <VaIcon name="supervisor_account" class="va-text-secondary"></VaIcon>
                </VaButton>
                <VaButton v-else class="task-update-btn">
                    <VaIcon name="person" class="va-text-secondary"></VaIcon>
                </VaButton>
            </div>
        </template>

        <template #cell(status)="{ rowData }">
            <TaskStatuses :status="rowData.status" :task="rowData"></TaskStatuses>
        </template>

        <template #cell(priority)="{ rowData }">
            <TaskPriorities :priority="rowData.priority" :task="rowData"></TaskPriorities>
        </template>

        <template #cell(date)="{ rowData }">
            <div class="task-date-holder">
                <TaskDateRange :range="rowData.date" :task="rowData" />
            </div>
        </template>

        <template #cell(expiration_date)="{ rowData }">
            <div class="task-expiration_date-holder">
                <TaskDate :date="rowData.expiration_date" :task="rowData" />
            </div>
        </template>
    </VaDataTable>
</template>


<style lang="scss">
.va-data-table__table-th-wrapper {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
}

.va-data-table__table-th-wrapper span {
    text-align: center
}

.va-data-table__table-th-wrapper i {
    position: absolute;
    right: 0;
}

.va-data-table__table-td {
    padding: 0 !important;
    align-items: center;
    justify-content: center;
}

.task-holder {
    display: flex;
    align-items: center;
    justify-content: center;

    .task-updates {
        .va-button {
            background: transparent;
            position: relative;

            .task-update-count {
                display: flex;
                justify-content: center;
                align-items: center;
                background: var(--va-primary);
                font-size: 10px;
                width: 1rem;
                height: 1rem;
                border-radius: 100%;
                position: absolute;
                bottom: .25rem;
                right: .25rem;

            }
        }

        .va-button:before {
            background: transparent;
        }
    }
}


.va-dropdown__content {
    border: 0;
    border-radius: 0;
    padding: 1rem;

    .va-menu-item__cell {
        padding: 0 !important;
    }

    .task-status,
    .task-priority {
        text-align: center;
        color: white;
        width: 100%;
        justify-content: center;
        align-items: center;
        border-radius: 0;
        margin-bottom: .25rem;

        .va-button__content {
            border-radius: 0;
            width: 100%;
            text-align: center;
            display: flex;
            justify-content: center;
        }
    }

    .task-status::before,
    .task-priority::before {
        border: 0;
    }
}

.va-menu-list {
    min-width: 10rem;
}

.bg-primary {
    background: var(--va-primary);
}

.bg-secondary {
    background: var(--va-secondary);
}

.bg-danger {
    background: var(--va-danger);
}

.bg-warning {
    background: var(--va-warning);
}

.bg-success {
    background: var(--va-success);
}

.bg-info {
    background: var(--va-info);
}

.task-priority-holder,
.task-status-holder {
    min-width: 10rem;
}

.task-status-holder .va-button::before,
.task-priority-holder .va-button::before {
    border-radius: 0;
}

.task-priority-holder button,
.task-status-holder button {
    width: 100%;
}

.task-visor-holder {
    display: flex;
    justify-content: center;
    align-items: center;

    .va-button {
        background: transparent !important;
    }

    .va-button::before {
        background: transparent !important;
    }
}

.task {
    .va-input-wrapper__field {
        border: 0;
        padding: 0;

        .va-input__content__input {
            padding: .25rem .5rem;
            border: solid 1px transparent;
            border-radius: 4px;
        }

        .va-input__content__input:focus {
            border: solid 1px var(--va-input-wrapper-border-color);
        }
    }
}
</style>
