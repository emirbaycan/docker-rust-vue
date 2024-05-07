<script setup lang="ts">
import { ref } from 'vue'
import Tasks from './Tasks.vue';
import { useItems } from '../composables/useTasks'
import { useModal, useToast } from 'vuestic-ui'

import { PropType } from 'vue';
import { CollectedTaskGroup, Task, TaskGroup } from '../../../api/agenda/types';

const doShowEditImageModal = ref(false)

const { isLoading, sorting } = useItems()

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

defineProps({
    group: {
        type: Object as PropType<CollectedTaskGroup>,
        required: true,
    },})

</script>

<style lang="scss">
    .task-group{
        display: flex;
        .task-group-options{
            .va-dropdown{

            }
        }
    }
</style>

<template>
    <div class="task-group">
        <div class="task-group-options">
            <VaMenu>
                <template #anchor>
                    <div>
                        <VaIcon name="more_horiz" color="secondary"></VaIcon>
                    </div>
                </template>
                <VaMenuItem>
                    <VaButton>
                        <VaIcon name="user" color="secondary"></VaIcon>
                        <span class="group-menu-item">Bu grubu daralt</span>
                    </VaButton>
                </VaMenuItem>
                <VaMenuItem>
                    <VaButton>
                        <VaIcon name="shrink" color="secondary"></VaIcon>
                        <span class="group-menu-item">Tüm grupları daralt</span>
                    </VaButton>
                </VaMenuItem>
                <VaMenuItem>
                    <VaButton>
                        <VaIcon name="check" color="secondary"></VaIcon>
                        <span class="group-menu-item">Tümünü seç</span>
                    </VaButton>
                </VaMenuItem>
                <VaSeparator></VaSeparator>
                <VaMenuItem>
                    <VaButton>
                        <VaIcon name="plus" color="secondary"></VaIcon>
                        <span class="group-menu-item">Grup ekle</span>
                    </VaButton>
                </VaMenuItem>
                <VaMenuItem>
                    <VaButton>
                        <VaIcon name="copy" color="secondary"></VaIcon>
                        <span class="group-menu-item">Grubu çoğalt</span>
                    </VaButton>
                </VaMenuItem>
            </VaMenu>
        </div>
        <div class="task-group-holder">
            <VaCardTitle class="task-group-title">
                {{ group.title }}
            </VaCardTitle>
            <VaCard class="tasks">
                <Tasks v-model:sort-by="sorting.sortBy" v-model:sorting-order="sorting.sortingOrder" :tasks="group.tasks"
                :loading="isLoading" @edit="showEditImageModal"></Tasks>
            </VaCard>
        </div>
    </div>
</template>
