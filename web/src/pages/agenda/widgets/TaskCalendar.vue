<script lang="ts">
import FullCalendar from '@fullcalendar/vue3'
import dayGridPlugin from '@fullcalendar/daygrid'
import interactionPlugin from '@fullcalendar/interaction'


export default {
  props:{
    data:{
      type: Array as () => Array<any>,
    }
  },
  components: {
    FullCalendar 
  },
  data() {
    return {
      calendarOptions: {
        plugins: [ dayGridPlugin, interactionPlugin ],
        initialView: 'dayGridMonth',
        events: this.data
      }
    }
  }
}
</script>
<template>
  <FullCalendar :options="calendarOptions" />
</template>