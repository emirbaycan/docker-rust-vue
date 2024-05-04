<script setup lang="ts">
import { PropType } from 'vue';
import { defineVaDataTableColumns } from 'vuestic-ui'
import { Task } from '../../../api/agenda/types';
import { Sorting } from '../../../api/agenda/request'
import { useVModel } from '@vueuse/core';

const columns = defineVaDataTableColumns([
  { label: 'Görev', key: 'name', sortable: true },
  { label: 'Sorumlu', key: 'supervisor', sortable: true },
  { label: 'Durum', key: 'status', sortable: true },
  { label: 'Öncelik', key: 'priority', sortable: true },
  { label: 'Zaman Aralığı', key: 'date', sortable: true },
  { label: 'Son Tarih', key: 'expiration_date', sortable: true },
  { label: ' ', key: 'actions' },
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

</script>

<template>
    <VaDataTable :columns="columns"
        :items="tasks" :loading="$props.loading">
    </VaDataTable>
</template>
