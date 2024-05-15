<script setup lang="ts">
import { PropType, defineEmits } from 'vue';
import Popup from '../../../components/general/Popup.vue';
import { Task } from '../../../api/agenda/types';

const props = defineProps({
    selected_tasks:{
        type: Object as PropType<Array<Task>>,
        required:true,
    },
    open: {
        type: Boolean as PropType<boolean>,
        required:true
    }
});

const emit = defineEmits(['close-popup']);

const closePopup = () => {
    emit('close-popup');
}

</script>

<template>
    <Teleport to="body">
        <div class="selection">
            <div class="inner">
                <div class="count"></div>
                <div class="functions">
                    <div class="function">
                        <div class="name">Kopya</div>
                        <div class="icon">
                            <VaIcon name="copy"></VaIcon>
                        </div>
                    </div>
                    <div class="function">
                        <div class="name">Sil</div>
                        <div class="icon">
                            <VaIcon name="copy"></VaIcon>
                        </div>
                    </div>
                    <div class="function">
                        <div class="name">Kopya</div>
                        <div class="icon">
                            <VaIcon name="copy"></VaIcon>
                        </div>
                    </div>
                </div>
                <div class="close">
                    <VaButton>
                        <VaIcon name="close"></VaIcon>
                    </VaButton>
                </div>
            </div>
        </div>
    </Teleport>
</template>

<style lang="scss">
 

</style>