<script setup lang="ts">
import { ref } from 'vue'
import { useItems } from './composables/useProjects'
import ProjectTable from './widgets/ProjectsTable.vue'
import EditProjectForm from './widgets/EditProjectForm.vue'
import { Project } from '../../api/projects/types'
import { useModal, useToast } from 'vuestic-ui'

const { items, isLoading, filters, sorting, pagination, ...itemsApi } = useItems()

const itemToEdit = ref<Project | null>(null)
const doShowProjectFormModal = ref(false)

const editProject = (item: Project) => {
  itemToEdit.value = item
  doShowProjectFormModal.value = true
}

const createNewProject = () => {
  itemToEdit.value = null
  doShowProjectFormModal.value = true
}

const { init: notify } = useToast()

const onProjectSaved = async (item: Project) => {
  if (itemToEdit.value) {
    await itemsApi.update(item)
    notify({
      message: `${item.title} has been updated`,
      color: 'success',
    })
  } else {
    itemsApi.add(item)
    notify({
      message: `${item.title} has been created`,
      color: 'success',
    })
  }
}

const { confirm } = useModal()

const onProjectDeleted = async (item: Project) => {
  const response = await confirm({
    title: 'Delete item',
    message: `Are you sure you want to delete item "${item.title}"?`,
    okText: 'Delete',
    size: 'small',
    maxWidth: '380px',
  })

  if (!response) {
    return
  }
  
  await itemsApi.remove(item)
  notify({
    message: 'Project deleted',
    color: 'success',
  })
}

const editFormRef = ref()

const beforeEditFormModalClose = async (hide: () => unknown) => {
  if (editFormRef.value.isFormHasUnsavedChanges) {
    const agreed = await confirm({
      maxWidth: '380px',
      message: 'Form has unsaved changes. Are you sure you want to close it?',
      size: 'small',
    })
    if (agreed) {
      hide()
    }
  } else {
    hide()
  }
}
</script>

<template>
  <VaCard>
    <VaCardContent>
      <div class="flex flex-col md:flex-row gap-2 mb-2 justify-between">
        <div class="flex flex-col md:flex-row gap-2 justify-start">
          <VaInput v-model="filters.search" placeholder="Search">
            <template #prependInner>
              <VaIcon name="search" color="secondary" size="small" />
            </template>
          </VaInput>
        </div>
        <VaButton icon="add" @click="createNewProject">Project</VaButton>
      </div>
      <ProjectTable
        v-model:sort-by="sorting.sortBy"
        v-model:sorting-order="sorting.sortingOrder"
        v-model:pagination="pagination"
        :items="items"
        :loading="isLoading"
        @edit="editProject"
        @delete="onProjectDeleted"
      />
    </VaCardContent>

    <VaModal
      v-slot="{ cancel, ok }"
      v-model="doShowProjectFormModal"
      size="small"
      mobile-fullscreen
      close-button
      stateful
      hide-default-actions
      :before-cancel="beforeEditFormModalClose"
    >
      <h1 v-if="itemToEdit === null" class="va-h5 mb-4">Add project</h1>
      <h1 v-else class="va-h5 mb-4">Edit project</h1>
      <EditProjectForm
        ref="editFormRef"
        :item="itemToEdit"
        :save-button-label="itemToEdit === null ? 'Add' : 'Save'"
        @close="cancel"
        @save="
          (item) => {
            onProjectSaved(item)
            ok()
          }
        "
      />
    </VaModal>
  </VaCard>
</template>
