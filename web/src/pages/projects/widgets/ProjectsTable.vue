<script setup lang="ts">
import { PropType, computed } from 'vue'
import { defineVaDataTableColumns } from 'vuestic-ui'
import { Project } from '../../../api/projects/types'
import { Pagination, Sorting } from '../../../api/projects/request'
import { useVModel } from '@vueuse/core'
import { parseDbDate } from '../../../services/utils';

const columns = defineVaDataTableColumns([
  { label: 'Project title', key: 'title', sortable: true },
  { label: 'Stacks', key: 'stacks', sortable: true },
  { label: 'Creation Date', key: 'created_at', sortable: true },
  { label: ' ', key: 'actions' },
])

const props = defineProps({
  items: {
    type: Array as PropType<Project[]>,
    required: true,
  },
  loading: {
    type: Boolean,
    required: true,
  },
  sortBy: {
    type: Object as PropType<Sorting['sortBy']>,
    required: true,
  },
  sortingOrder: {
    type: Object as PropType<Sorting['sortingOrder']>,
    required: true,
  },
  pagination: {
    type: Object as PropType<Pagination>,
    required: true,
  },
})

const emit = defineEmits<{
  (event: 'edit', item: Project): void
  (event: 'delete', item: Project): void
}>()

const sortByVModel = useVModel(props, 'sortBy', emit)
const sortingOrderVModel = useVModel(props, 'sortingOrder', emit)

const totalPages = computed(() => Math.ceil(props.pagination.total / props.pagination.perPage))
</script>

<template>
  <div>
    <VaDataTable 
    v-model:sort-by="sortByVModel" 
    v-model:sorting-order="sortingOrderVModel" 
    :items="items"
      :columns="columns" 
      :loading="loading">
      <template #cell(created_at)="{ rowData }">
        {{ parseDbDate(rowData.created_at) }}
      </template>

      <template #cell(actions)="{ rowData: item }">
        <div class="flex gap-2 justify-end">
          <VaButton preset="primary" size="small" color="primary" icon="mso-edit" aria-label="Edit project"
            @click="$emit('edit', item as Project)" />
          <VaButton preset="primary" size="small" icon="mso-delete" color="danger" aria-label="Delete project"
            @click="$emit('delete', item as Project)" />
        </div>
      </template>
    </VaDataTable>
    <div class="flex flex-col-reverse md:flex-row gap-2 justify-between items-center py-2">
      <div>
        <b>{{ $props.pagination.total }} results.</b>
        Results per page
        <VaSelect v-model="$props.pagination.perPage" class="!w-20" :options="[10, 50, 100]" />
      </div>

      <div v-if="totalPages > 1" class="flex">
        <VaButton preset="secondary" icon="va-arrow-left" aria-label="Previous page"
          :disabled="$props.pagination.page === 1" @click="$props.pagination.page--" />
        <VaButton class="mr-2" preset="secondary" icon="va-arrow-right" aria-label="Next page"
          :disabled="$props.pagination.page === totalPages" @click="$props.pagination.page++" />
        <VaPagination v-model="$props.pagination.page" buttons-preset="secondary" :pages="totalPages" :visible-pages="5"
          :boundary-links="false" :direction-links="false" />
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.va-data-table {
  ::v-deep(tbody .va-data-table__table-tr) {
    border-bottom: 1px solid var(--va-background-border);
  }
}
</style>
