<script setup lang="ts">
import { PropType, defineEmits } from 'vue';
import Sideup from '../../../components/general/Sideup.vue';

const props = defineProps({
    open: {
        type: Boolean as PropType<boolean>,
        required:true
    }
});

const emit = defineEmits(['close-sideup']);

const closeSideup = () => {
    emit('close-sideup');
}

</script>

<template>
    <Sideup :open="open" @close-sideup="closeSideup" >
        <div>
            
        </div>
    </Sideup>
</template>

<style lang="scss">

</style>