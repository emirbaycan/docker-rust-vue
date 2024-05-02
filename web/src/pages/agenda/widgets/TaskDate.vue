<script setup lang="ts">
import { PropType } from 'vue';
import { Task } from '../../../api/agenda/types';

defineProps({
    value: {
        type: Object as PropType<Task>,
        required: true,
    }
})

</script>

<template>
    <VaDateInput v-model="value.date" />
</template>
