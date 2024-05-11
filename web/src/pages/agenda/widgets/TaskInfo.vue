<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { DataTableItem } from 'vuestic-ui/web-components';
import { updateTask } from '../../../api/agenda/request';
import { UpdateTask } from '../../../api/agenda/types';


export default defineComponent({
    props: {
       
    },
});
</script>

<template>
    <div class="task-info">
        <div class="task-info-inner">
            <div class="task-info-left">
                
            </div>
            <div class="task-info-right">
                
            </div>
        </div>
        <div class="task-info-close"></div>
        <div class="task-info-bg"></div>
    </div>
</template>