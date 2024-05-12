<script lang="ts">
import { PropType } from 'vue';

export default {
    props: {
        open: {
            type: Boolean as PropType<boolean>,
        }
    },
    methods: {
        closePopup() {
            this.$emit('close-popup');
        }
    }
}

</script>

<style lang="scss">
.popup {
    width: 100%;
    height: 100%;
    position: fixed;
    top: 0;
    left: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0;
    visibility: hidden;
    z-index: 99999;

    .popup-inner {
        min-width: 300px;
        min-height: 300px;
        margin: auto;
        position: relative;
        z-index: 1;
    }

    .popup-close {
        position: absolute;
        top: 0;
        right: 0;
    }

    .popup-bg {
        position: absolute;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
        background: black;
        opacity: .7;
    }
}

.popup.open {
    opacity: 1;
    visibility: visible;
}
</style>

<template>
    <Teleport to="body">
        <div class="popup" :class="open ? 'open' : ''">
            <div class="popup-inner">
                <slot />
                <div class="popup-close">
                    <VaButton preset="plainOpacity" color="danger" @click="closePopup">
                        <VaIcon name="close"></VaIcon>
                    </VaButton>
                </div>
            </div>
            <div class="popup-bg" @click="closePopup"></div>
        </div>
    </Teleport>
</template>