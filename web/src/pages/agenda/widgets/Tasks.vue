<script setup lang="ts">
import { ref, computed, PropType } from 'vue';
import { defineVaDataTableColumns } from 'vuestic-ui'
import { Task, TaskStatus, TaskUpdate } from '../../../api/agenda/types';
import { Sorting } from '../../../api/agenda/request'
import { useVModel } from '@vueuse/core';
import TaskDate from './TaskDate.vue';
import TaskDateRange from './TaskDateRange.vue';

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
  2: 'warning',
  3: 'danger',
  4: 'success', 
}

const taskStatusTexts: Record<any, string> = {
  1: 'Başlamadı',
  2: 'Yapılmakta',
  3: 'Takılı',
  4: 'Tamamlandı', 
}

const taskPriorityClasses: Record<any, string> = {
  1: 'secondary',
  2: 'info',
  3: 'warning',
  4: 'danger', 
}

const taskPriorityTexts: Record<any, string> = {
  1: 'Düşük',
  2: 'Normal',
  3: 'Yüksek',
  4: 'Kritik', 
}

const formattedDate = (dateString: string) => {
    return computed({
        get: () => new Date(dateString.replace('T',' ').split('.')[0]),
        set: (newValue: Date) => newValue.toISOString() 
    });
};

</script>

<style lang="scss">
.va-data-table__table-th-wrapper{
    position: relative;
    display: flex;
    justify-content:center;
    align-items:center;
}

.va-data-table__table-th-wrapper span{
    text-align:center
}

.va-data-table__table-th-wrapper i {
    position: absolute;
    right:0;
}

.va-data-table__table-td{
    padding:0 !important;
    align-items: center;
    justify-content: center;
}

.task-holder{
    display: flex;
    align-items: center;
    justify-content: center;
    .task-updates{
        .va-button{
            background:transparent;
            position: relative;
            .task-update-count{
                display: flex;
                justify-content: center;
                align-items:center;
                background:var(--va-primary);
                font-size:10px;
                width: 1rem;
                height: 1rem;
                border-radius: 100%;
                position: absolute;
                bottom: .25rem;
                right: .25rem;

            }
        }
        .va-button:before{
            background:transparent;
        }
    }
}


.va-dropdown__content{
    border:0;
    border-radius: 0;
    padding:0;
    .va-menu-item__cell{
        padding:0 !important;
    }
    .task-status, .task-priority{
        padding: .75rem 1rem;
        text-align: center;
        color:white;
    }
}
.va-menu-list{
    min-width:10rem;
}

.bg-primary{
    background: var(--va-primary);
}
.bg-secondary{
    background: var(--va-secondary);
}
.bg-danger{
    background: var(--va-danger);
}
.bg-warning{
    background: var(--va-warning);
}
.bg-success{
    background: var(--va-success);
}
.bg-info{
    background: var(--va-info);
}

.task-priority-holder, .task-status-holder{
    min-width: 10rem;
}

.task-status-holder .va-button::before, .task-priority-holder .va-button::before{
    border-radius:0;
}

.task-priority-holder button, .task-status-holder button{
    width: 100%;
}

.task-visor-holder{
    display:flex;
    justify-content:center;
    align-items: center;
    .va-button{
        background:transparent !important;
    }
    
    .va-button::before{
        background:transparent !important;
    }
}



</style>

<template>
    <VaDataTable :columns="columns" :items="tasks" :loading="$props.loading">

        <template #header(selection)="{ label }">
            <VaCheckbox></VaCheckbox>
        </template>

        <template #cell(selection)="{ rowData }">
            <VaCheckbox></VaCheckbox>
        </template>

        <template #cell(name)="{ rowData }">
            <div class="task-holder">
                <div class="task pr-2">
                    {{ rowData.name }}
                </div>
                <div class="task-updates">
                    <VaButton v-if="rowData.updates.length" class="task-update-btn">
                        <VaIcon name="comment" class="va-text-secondary"></VaIcon>
                        <span class="task-update-count">{{rowData.updates.length}}</span>
                    </VaButton>
                    <VaButton v-else class="task-update-btn" >
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
            <div class="task-status-holder">
                <VaMenu class="task-status-picker">
                    <template #anchor>
                        <VaButton> {{ taskStatusTexts[rowData.status] }} </VaButton>
                    </template>
                    
                    <VaMenuItem v-for="(key,value,index) in taskStatusTexts" class="task-status-outer">
                        <div class="task-status" :class="'bg-'+taskStatusClasses[value]" >
                            {{ key }}
                        </div>
                    </VaMenuItem> 
                </VaMenu>
            </div>
        </template>
        
        <template #cell(priority)="{ rowData }">
            <div class="task-priority-holder">
                <VaMenu>
                    <template #anchor>
                        <VaButton> {{ taskPriorityTexts[rowData.status] }} </VaButton>
                    </template>
                    
                    <VaMenuItem v-for="(key,value,index) in taskPriorityTexts">
                        <div class="task-priority" :class="'bg-'+taskPriorityClasses[value]" >
                            {{ key }}
                        </div>
                    </VaMenuItem> 
                </VaMenu>
            </div>
        </template>

        <template #cell(date)="{ rowData }">
            <div class="task-date-holder">
                <TaskDateRange :range="rowData.date"/>
            </div>
        </template> 

        <template #cell(expiration_date)="{ rowData }">
            <div class="task-expiration_date-holder">
                <TaskDate :date="rowData.expiration_date"/>
            </div>
        </template>
    </VaDataTable>
</template>
