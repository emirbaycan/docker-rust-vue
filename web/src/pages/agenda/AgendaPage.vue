<script setup lang="ts">
import { ref } from 'vue'
import Tasks from './widgets/Tasks.vue';
import { Task } from '../../api/agenda/types';
import { useItems } from './composables/useTasks'
import { useModal, useToast } from 'vuestic-ui'

const doShowEditImageModal = ref(false)

const { tasks, isLoading, filters, sorting, ...itemsApi } = useItems()

const itemToEdit = ref<Task | null>(null)

const showEditImageModal = (task: Task) => {
  itemToEdit.value = task
  doShowEditImageModal.value = true
}

const showAddImageModal = () => {
  itemToEdit.value = null
  doShowEditImageModal.value = true
}

const { init: notify } = useToast()

</script>

<template>
  <VaCard>
    <VaCard>
      <VaCardContent>
        <div class="flex flex-col md:flex-row gap-2 mb-2 justify-between">
          <VaMenu>
            <template #anchor>
              <VaButton>Actions</VaButton>
            </template>
            <VaMenuItem>
              <VaButton>Add Task Group</VaButton>
            </VaMenuItem>
          </VaMenu>
        </div>
      </VaCardContent>
    </VaCard>
    <VaCard>
      <Tasks v-model:sort-by="sorting.sortBy" v-model:sorting-order="sorting.sortingOrder" :tasks="tasks"
        :loading="isLoading" @edit="showEditImageModal"></Tasks>
    </VaCard>
  </VaCard>

</template>
