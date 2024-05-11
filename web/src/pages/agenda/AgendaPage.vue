<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { AllTasks, CollectedTaskGroup, CreateTask, Task, TaskAgenda, TaskUpdate } from '../../api/agenda/types';
import { useItems } from './composables/useTasks';
import TaskGroups from './widgets/TaskGroups.vue';
import TaskCalendar from './widgets/TaskCalendar.vue';
import { useRoute } from 'vue-router';

const route = useRoute();

const agenda_id = parseInt(route.query.id as string);

const getAgenda = () => {
  var agendas = localStorage.getItem('agendas');
  if (agendas) {
    var all_agendas: Array<TaskAgenda> = JSON.parse(agendas);
    all_agendas.filter(agenda => agenda.agenda_id == agenda_id);
    return all_agendas[0];
  }
};

var filters = ref({
  agenda_id: agenda_id
});

const { items, isLoading } = useItems(
  {
    filters: filters
  }
)

const parseGroups = (items: AllTasks | undefined) => {

  if (!items) {
    return;
  }
  var groups = items.groups;
  var tasks = items.tasks;
  var updates = items.updates;
  var supervisors = items.supervisors;
  var visors = items.visors;

  var new_groups: Array<CollectedTaskGroup> = []

  groups.forEach(group => {
    var new_group: CollectedTaskGroup = {
      group_id: group.group_id,
      agenda_id: group.agenda_id,
      title: group.title,
      tasks: []
    }
    tasks.forEach(task => {
      if (group.group_id != 0 && task.group_id != group.group_id) {
        return;
      }

      task.updates = updates.filter(update => update.task_id == task.task_id);
      task.supervisors = supervisors.filter(supervisor => supervisor.task_id == task.task_id);;

      new_group.tasks.push(task);
      new_group.group_id = task.group_id;

    });
    var task_adder: Task = {
      task_id: 0,
      group_id: new_group.group_id,
      name: "",
      date: "",
      expiration_date: "",
      status: 0,
      priority: 0,
      updates: [
        {
          task_id: 0,
          update_id: 0,
          user_id: 0,
          email: "",
          fullname: "",
          avatar: "",
          text: "",
          created_at: "",
          updated_at: "",
        }
      ],
      supervisors: [{
        supervisor_id: 0,
        task_id: 0,
        email: "",
        fullname: "",
        avatar: "",
        created_at: "",
        updated_at: "",
      }],
      created_at: "",
      updated_at: "",
    }
    new_group.tasks.push(task_adder);
    if (new_group.group_id == 0) {
      return;
    }
    new_groups.push(new_group);
  });

  return new_groups;
}

var selectedTab = ref(0);

type CalendarTask = {
  id: number,
  title: string,
  start: number,
  end: number,
}

var taskStatusClasses = {
  0: {
    textColor: 'white',
    borderColor: 'transparent',
    backgroundColor: '#767C88',
  },
  1: {
    textColor: 'white',
    borderColor: 'transparent',
    backgroundColor: '#767C88',
  },
  2: {
    textColor: 'black',
    borderColor: 'transparent',
    backgroundColor: '#FFD43A',
  },
  3: {
    textColor: 'white',
    borderColor: 'transparent',
    backgroundColor: '#E42222',
  },
  4: {
    textColor: 'white',
    borderColor: 'transparent',
    backgroundColor: '#154EC1',
  }
}

const calendarData = (tasks: Array<Task> | undefined) => {
  var data: Array<CalendarTask> = [];

  if (!tasks) {
    return;
  }

  tasks.forEach(task => {

    var date = task.date.split(' - ');
    var status = taskStatusClasses[parseInt(task.status)];
    var new_task: GantTask = {
      id: task.task_id,
      title: task.name,
      start: parseInt(date[0]),
      end: parseInt(date[1]),
      backgroundColor: status.backgroundColor,
      textColor: status.textColor,
      borderColor: status.borderColor,
    }

    data.push(new_task)

  });
  return data;
}


</script>


<template>
  <VaCard class="agenda" color="transparent">
    <div class="agenda-header">
      <div class="agenda-title">
        <VaInput class="agenda-title-input" :model-value="getAgenda()?.title"></VaInput>
      </div>
      <VaButton class="agenda-updates">
        <div class="agenda-updates-title">
          Updates
        </div>
        <div v-if="items && items.updates.length" class="agenda-updater">
          <VaAvatar :src="items.updates[0].avatar"></VaAvatar>
        </div>
      </VaButton>
      <VaButton class="agenda-invite">
        <VaIcon name="person_add"></VaIcon>
        <div class="agenda-invite-title">Invite</div>
      </VaButton>
      <VaButton>
        <VaMenu>
          <template #anchor>
            <div class="task-group-option-menu">
              <VaIcon name="more_horiz" color="secondary"></VaIcon>
            </div>
          </template>
          <VaMenuItem>
            <VaButton>
              <VaIcon name="remove" color="secondary"></VaIcon>
              <span class="group-menu-item">Delete Agenda</span>
            </VaButton>
          </VaMenuItem>
        </VaMenu>
      </VaButton>
    </div>
    <div class="agenda-description">
      <div class="agenda-desc"></div>
      <VaButton>
        See More
      </VaButton>
    </div>
    <VaTabs v-model="selectedTab">
      <template #tabs>
        <VaTab v-for="tab in ['Table', 'Calendar']" :key="tab">
          {{ tab }}
        </VaTab>
      </template>
    </VaTabs>
    <div class="tab-items">
      <TaskGroups :groups="parseGroups(items)" :loading="isLoading" v-if="selectedTab == 0"></TaskGroups>
      <TaskCalendar :data="calendarData(items?.tasks)" v-if="selectedTab == 1"></TaskCalendar>
    </div>
  </VaCard>
</template>


<style lang="scss">
.agenda {
  .agenda-top {
    margin-bottom: 1rem;
  }
}
</style>