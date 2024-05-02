<script setup lang="ts">
import { PropType } from 'vue';
import { TaskPriority } from '../../../api/agenda/types';


const priorityColor = (priority: number) => {
    const colors = ['', 'secondary', 'warning', 'danger', 'primary']
    return colors[priority];
}

const priorityName = (priority: number) => {
    const names = ['', 'Düşük', 'Normal', 'Yüksek', 'Kritik']
    return names[priority];
}


defineProps({
    priority: {
        type: Object as PropType<TaskPriority>,
        required: true,
    }
})

</script>

<template>
    <VaButton :color="priorityColor(priority)">
        {{ priorityName(priority) }}
    </VaButton>
</template>
