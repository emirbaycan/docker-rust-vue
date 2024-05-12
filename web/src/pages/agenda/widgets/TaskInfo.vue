<script setup lang="ts">
import { PropType, ref } from 'vue';
import Popup from '../../../components/general/Popup.vue';

const props = defineProps({
    title:{
        type: Object as PropType<string>,
    },
    description:{
        type: Object as PropType<string>,
    },
    open: {
        type: Boolean as PropType<boolean>,
        required:true
    }
});

const emit = defineEmits<{
    (event: 'close-popup'): void
}>()


const closePopup = () => {
    emit('close-popup');
}

</script>

<template>
    <Popup :open="open" @close-popup="closePopup">
        <div class="agenda-info">
            <div class="agenda-info-left">
                <VaInput :model-value="title"></VaInput>
                <VaTextarea :model-value="description"></VaTextarea>
            </div>
            <div class="agenda-info-right"> 
                <div class="agenda-info-header">
                    Pano Bilgisi
                </div>
                <div class="agenda-info-head">
                    Oluşturan
                </div>
                <div class="agenda-info-desc">
                    <VaAvatar></VaAvatar>
                    <div class="agenda-info-date">
                        14 Mayıs, 1998
                    </div>
                </div>
                <div class="agenda-info-head">
                    Sahipleri
                </div>
                <div class="agenda-info-desc">
                    <div class="agenda-info-owner">
                        <VaAvatar class="image"></VaAvatar>
                        <div class="name">
                            Emir Baycan
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </Popup>
</template>