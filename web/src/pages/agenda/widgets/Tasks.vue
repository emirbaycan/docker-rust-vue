<script setup lang="ts">
import { PropType } from 'vue';
import { defineVaDataTableColumns } from 'vuestic-ui'
import { Task, TaskStatus, TaskUpdate } from '../../../api/agenda/types';
import { Sorting } from '../../../api/agenda/request'
import { useVModel } from '@vueuse/core';

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
    },
})


const emit = defineEmits<{
  (event: 'edit', item: Task): void
  (event: 'delete', item: Task): void
}>()

const taskStatusClasses: Record<any, string> = {
  1: 'secondary',
  2: 'background-element',
  3: 'danger',
  4: 'primary', 
}

const taskStatusTexts: Record<any, string> = {
  1: 'Başlamadı',
  2: 'Yapılmakta',
  3: 'Takılı',
  4: 'Tamamlandı', 
}

const taskPriorityClasses: Record<any, string> = {
  1: 'secondary',
  2: 'background-element',
  3: 'warning',
  4: 'danger', 
}

const taskPriorityTexts: Record<any, string> = {
  1: 'Düşük',
  2: 'Normal',
  3: 'Yüksek',
  4: 'Kritik', 
}

</script>

<template>
    <VaDataTable :columns="columns"
        :items="tasks" :loading="$props.loading">

    <template #header(selection)="{ label }">
        <VaCheckbox></VaCheckbox>
    </template>

    <template #cell(selection)="{ rowData }">
        <VaCheckbox></VaCheckbox>
    </template>

    <template #cell(name)="{ rowData }">
        <div class="task-holder">
            <div class="task">
                {{ rowData.name }}
            </div>
            <div class="task-updates">
                <VaButton v-if="rowData.updates.length" class="task-update-btn">
                    <VaIcon name="comment"></VaIcon>
                    <span class="task-update-count">{{rowData.updates.length}}</span>
                </VaButton>
                <VaButton v-else class="task-update-btn">
                    <VaIcon name="plus"></VaIcon>
                </VaButton>
            </div>
        </div>        
    </template>
    
    <template #cell(supervisor)="{ rowData }">
        <div class="task-visor-holder">
            <VaButton v-if="rowData.supervisors.length" class="task-update-btn">
                <VaIcon name="supervisor_account"></VaIcon>
            </VaButton>
            <VaButton v-else class="task-update-btn">
                <VaIcon name="person"></VaIcon>
            </VaButton>
        </div>        
    </template>

    <template #cell(status)="{ rowData }">
        <div class="task-status-holder">
            <div class="task-status" :class="taskStatusClasses[rowData.status]" >
                {{ taskStatusTexts[rowData.status] }}
            </div>
        </div>
    </template>
    
    <template #cell(priority)="{ rowData }">
        <div class="task-priority-holder">
            <div class="task-priority" :class="taskPriorityClasses[rowData.priority]" >
                {{ taskPriorityTexts[rowData.priority] }}
            </div>
        </div>
    </template>

    <template #cell(date)="{ rowData }">
        <div class="task-date-holder">
            <VaDateInput v-model="rowData.date" />
        </div>
    </template>

    <template #cell(expiration_date)="{ rowData }">
        <div class="task-expiration_date-holder">
            <VaDateInput v-model="rowData.expiration_date" />
        </div>
    </template>
    </VaDataTable>
</template>
