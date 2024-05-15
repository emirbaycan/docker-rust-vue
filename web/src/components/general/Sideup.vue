<template>
    <Teleport to="body">
        <div class="sideup" :class="{ open: open }">
            <div class="sideup-inner">
                <slot />
                <div class="sideup-close">
                    <VaButton preset="plainOpacity" color="danger" @click="closeSideup">
                        <VaIcon name="close"></VaIcon>
                    </VaButton>
                </div>
            </div>
        </div>
    </Teleport>
</template>

<script lang="ts">
import { defineComponent, PropType} from 'vue';

export default defineComponent({
    name: 'Sideup',
    props: {
        open: {
            type: Boolean as PropType<boolean>,
            required: true,
        }
    },
    emits: ['close-sideup'],
    methods: {
        closeSideup() {
            this.$emit('close-sideup');
        },
    }
});

</script>

<style lang="scss">
.sideup {
    min-width: 50%;
    height: 100%;
    position: fixed;
    top: 0;
    right: -100%;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 99999;

    .sideup-inner {
        min-width: 300px;
        height: 100%;
        margin: auto;
        position: relative;
        z-index: 1;
    }

    .sideup-close {
        position: absolute;
        top: 0;
        right: 0;
    }

    .sideup-bg {
        position: absolute;
        width: 100%;
        height: 100%;
        top: 0;
        left: 0;
        background: black;
        opacity: .7;
    }
}

.sideup.open {
    right: 0;
}
</style>