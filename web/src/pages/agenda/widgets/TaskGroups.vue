<script setup lang="ts">
import { PropType } from 'vue';
import { CollectedTaskGroup } from '../../../api/agenda/types';
import TaskGroup from './TaskGroup.vue';

defineProps({
    groups: {
        type: Object as PropType<Array<CollectedTaskGroup>>,
    },})

</script>

<template>
    <div class="task-groups" v-for="group in groups">
        <TaskGroup :group="group"></TaskGroup>
    </div>
</template>
