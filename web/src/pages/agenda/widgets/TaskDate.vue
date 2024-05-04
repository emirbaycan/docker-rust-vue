<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
    props: {
        date: {
            type: String,
            required: true
        }
    },
    data() {
        return {
            single: new Date(this.date.replace('T',' ').split('.')[0])
        };
    }
});
</script>

<template>
    <VaDateInput v-model="single" />
</template>