<script setup lang="ts">
import { PropType } from 'vue';
import { TaskStatus } from '../../../api/agenda/types';


const statusColor = (status: number) => {
    const colors = ['', 'secondary', 'warning', 'danger', 'primary']
    return colors[status];
}

const statusName = (status: number) => {
    const names = ['', 'Başlamadı', 'Yapılmakta', 'Takılı', 'Tamamlandı']
    return names[status];
}


defineProps({
    status: {
        type: Object as PropType<TaskStatus>,
        required: true,
    }
})

</script>

<template>
    <VaButton :color="statusColor(status)">
        {{ statusName(status) }}
    </VaButton>
</template>
