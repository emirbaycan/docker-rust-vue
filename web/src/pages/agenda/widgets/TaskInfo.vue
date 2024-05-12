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
            <div class="left">
                <VaInput :model-value="title" class="input-no-border"></VaInput>
                <VaTextarea :model-value="description" class="input-no-border"></VaTextarea>
            </div>
            <div class="right"> 
                <div class="header">
                    Pano Bilgisi
                </div>
                <div class="head">
                    Oluşturan
                </div>
                <div class="desc">
                    <VaAvatar></VaAvatar>
                    <div class="date">
                        14 Mayıs, 1998
                    </div>
                </div>
                <div class="head">
                    Sahipleri
                </div>
                <div class="desc">
                    <div class="owner">
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

<style lang="scss">

.agenda-info{
    display:flex;
    .left{
        display:flex;
        flex-direction: column;
        background:white;
        padding:1rem;
        .va-input{
       
        }
        .va-textarea{
            min-height: 300px;
            width:100%;
            .va-input-wrapper__fieldset{
                height: 100%;
            }
        }
        .va-input-wrapper__field{
            border:0;
        }
    }

    .right{
        display:flex;
        flex-direction: column;
        background:#33354b;
        padding:1rem;
        min-width: 300px;
        .header{
            font-size: 1rem;
            color:white;
        }
    }
}

</style>