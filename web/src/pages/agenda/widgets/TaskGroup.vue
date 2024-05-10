<script setup lang="ts">
import { ref } from 'vue'
import Tasks from './Tasks.vue';
import { useItems } from '../composables/useTasks'
import { useModal, useToast } from 'vuestic-ui'

import { PropType } from 'vue';
import { CollectedTaskGroup, Task, TaskGroup } from '../../../api/agenda/types';

const doShowEditImageModal = ref(false)

const { isLoading } = useItems()

const itemToEdit = ref<Task | null>(null)

defineProps({
    group: {
        type: Object as PropType<CollectedTaskGroup>,
        required: true,
    },
})

</script>

<style lang="scss">
.task-group {
    display: flex;

    .task-group-options {
        .va-dropdown {}
    }
}
</style>

<template>
    <div class="task-group">
        <div class="task-group-holder">
            <VaCardTitle class="task-group-title">
                {{ group.title }}
            </VaCardTitle>
            <VaCard class="tasks">
                <Tasks :tasks="group.tasks" :loading="isLoading"></Tasks>
            </VaCard>
        </div>
    </div>
</template>
