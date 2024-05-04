<script lang="ts">
import { defineComponent, computed } from 'vue';

const formatDate = (dateString: string): Date => {
    if (!dateString) {
        return new Date();
    }
    return new Date(dateString.replace('T', ' ').split('.')[0]);
};

export default defineComponent({
    props: {
        range: {
            type: String,
            required: true
        }
    },
    data(props) {
        var start, end;
        const ranges = props.range.split(' - ');

        if (ranges.length > 1) {
            start = formatDate(ranges[0]);
            end = formatDate(ranges[1]);
        } else {
            const now = new Date();
            start = now;
            end = now;
        }

        var rangeDate = {
            start: start,
            end: end
        }

        return { rangeDate };
    }
});
</script>

<template>
    <VaDateInput v-model="rangeDate" />
</template>