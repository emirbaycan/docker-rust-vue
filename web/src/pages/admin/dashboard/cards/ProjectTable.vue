<script setup lang="ts">
import { defineVaDataTableColumns } from 'vuestic-ui'
import { useItems } from '../../../projects/composables/useProjects'
import { Pagination } from '../../../../api/projects/request'
import { ref } from 'vue'

const columns = defineVaDataTableColumns([
  { label: 'Project Name', key: 'title', sortable: true },
  { label: 'Stacks', key: 'stacks', sortable: true },
  { label: 'Creation Date', key: 'created_at', sortable: true },
])

const pagination = ref<Pagination>({ page: 1, perPage: 5, total: 0 })
const { items, isLoading, sorting } = useItems({
  pagination,
})
</script>

<template>
  <VaCard>
    <VaCardTitle class="flex items-start justify-between">
      <h1 class="card-title text-secondary font-bold uppercase">Projects</h1>
      <VaButton preset="primary" size="small" to="/projects">View all projects</VaButton>
    </VaCardTitle>
    <VaCardContent>
      <div v-if="items.length > 0">
        <VaDataTable
          v-model:sort-by="sorting.sortBy"
          v-model:sorting-order="sorting.sortingOrder"
          :items="items"
          :columns="columns"
          :loading="isLoading"
        >
          <template #cell(project_name)="{ rowData }">
            <div class="ellipsis max-w-[230px] lg:max-w-[450px]">
              {{ rowData.project_name }}
            </div>
          </template>
          <template #cell(status)="{ rowData: project }">
            <ProjectStatusBadge :status="project.status" />
          </template>
        </VaDataTable>
      </div>
      <div v-else class="p-4 flex justify-center items-center text-[var(--va-secondary)]">No projects</div>
    </VaCardContent>
  </VaCard>
</template>
