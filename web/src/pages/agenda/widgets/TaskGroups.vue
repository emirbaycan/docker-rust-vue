<script setup lang="ts">
import { PropType } from 'vue';
import { DisplayTaskGroup } from '../../../api/agenda/types';


defineProps({
    groups: {
        type: Object as PropType<Array<DisplayTaskGroup>>,
        required: true,
    },})

</script>

<template>
    <div class="task-groups" v-for="group in groups">
        <TaskGroup :group="group" :tasks="group.tasks"></TaskGroup>
    </div>
</template>
