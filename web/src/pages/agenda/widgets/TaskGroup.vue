<script setup lang="ts">
import { PropType } from 'vue';
import { Task, TaskGroup } from '../../../api/agenda/types';


defineProps({
    tasks: {
        type: Object as PropType<Array<Task>>,
        required: true,
    },
    group: {
        type: Object as PropType<TaskGroup>,
        required: true,
    },})

</script>

<template>
    <div class="flex">
        <div class="flex">
            <VaMenu>
                <template #anchor>
                    <VaIcon name="dots" color="secondary"></VaIcon>
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
        <div class="flex">
            <VaCardTitle>
                {{ group.title }}
            </VaCardTitle>
        </div>
    </div>
</template>
