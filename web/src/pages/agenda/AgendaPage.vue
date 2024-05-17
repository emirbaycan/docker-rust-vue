<script setup lang="ts">
import { ref, watch } from 'vue';
import { AllTasks, CollectedTaskGroup, CreateTaskGroup, RemoveTaskGroup, Task, TaskAgenda, TaskGroup, UpdateTaskAgendaDescription, UpdateTaskAgendaTitle, UpdateTaskGroup } from '../../api/agenda/types';
import { useItems } from './composables/useTasks';
import TaskGroups from './widgets/TaskGroups.vue';
import TaskCalendar from './widgets/TaskCalendar.vue';
import { useRoute } from 'vue-router';
import { addTaskGroup, removeTaskGroup, updateTaskAgendaDescription, updateTaskAgendaTitle, updateTaskGroup } from '../../api/agenda/request';

const route = useRoute();

const agenda_id = parseInt(route.query.id as string);

function getAgenda() {
  var agendas = localStorage.getItem('agendas');
  if (agendas) {
    var all_agendas: Array<TaskAgenda> = JSON.parse(agendas);
    all_agendas.filter(agenda => agenda.agenda_id == agenda_id);
    return all_agendas[0];
  }
};

const agenda = getAgenda();

var agenda_title = ref(agenda?.title);
var agenda_description = ref(agenda?.description);

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
  textColor: string,
  borderColor: string,
  backgroundColor: string,
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
    var status = taskStatusClasses[task.status];
    var new_task: CalendarTask = {
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

const agendaDetailsPopup = ref(false);
const taskUpdatesSideUp = ref(false);

const emit = defineEmits(['update-task-group-title','add-task-group', 'close-popup', 'close-sideup', 'update-agenda-title', 'update-agenda-description']);

const closePopup = () => {
  emit('close-popup');
  agendaDetailsPopup.value = false;
}

const closeSideup = () => {
  emit('close-sideup');
  taskUpdatesSideUp.value = false;
}

function openTaskUpdatesSideUp() {
  taskUpdatesSideUp.value = true;
}

function openPopup() {
  agendaDetailsPopup.value = true;
}

async function updateAgendaTitle(value: string) {
  if (!value) {
    return;
  }
  var update_agenda: UpdateTaskAgendaTitle = {
    agenda_id: agenda_id,
    title: value,
  }
  var new_agenda: TaskAgenda = await updateTaskAgendaTitle(update_agenda)
  var agendas = localStorage.getItem('agendas');
  if (agendas) {
    var all_agendas: Array<TaskAgenda> = JSON.parse(agendas);
    var index = all_agendas.findIndex(agenda => agenda.agenda_id === agenda_id);
    if (index !== -1) {
      all_agendas[index] = new_agenda;
      localStorage.setItem('agendas', JSON.stringify(all_agendas));
      return new_agenda;
    }
  }
}

async function updateAgendaDescription(value: string) {
  if (!value) {
    return;
  }
  var update_agenda: UpdateTaskAgendaDescription = {
    agenda_id: agenda_id,
    description: value,
  }
  var new_agenda: TaskAgenda = await updateTaskAgendaDescription(update_agenda)
  var agendas = localStorage.getItem('agendas');
  if (agendas) {
    var all_agendas: Array<TaskAgenda> = JSON.parse(agendas);
    var index = all_agendas.findIndex(agenda => agenda.agenda_id === new_agenda.agenda_id);
    if (index !== -1) {
      all_agendas[index] = new_agenda;
      localStorage.setItem('agendas', JSON.stringify(all_agendas));
    }
  }
}

const addNewTaskGroup = async () => {
  if (!groups || !groups.value || !items.value) {
    return;
  }

  const newItem: CreateTaskGroup = {
    agenda_id: agenda_id,
    title: "New Task Group"
  };

  var completeNewItem = await addTaskGroup(newItem);

  items.value.groups.push(completeNewItem);

  groups.value = parseGroups(items.value);
};


const deleteTaskGroup = async (group_id: number) => {
  if (!groups || !groups.value || !items.value) {
    return;
  }

  const item: RemoveTaskGroup = {
    group_id: group_id,
  };

  var result = await removeTaskGroup(item);

  const indexToRemove = items.value.groups.findIndex(group => group.group_id === group_id);
  if (indexToRemove !== -1) {
    items.value.groups.splice(indexToRemove, 1);
  }

  groups.value = parseGroups(items.value);
};


const updateTaskGroupTitle = async (value: string, group_id: number) => {
  if (!groups || !groups.value || !items.value) {
    return;
  }

  var update_item: UpdateTaskGroup = {
    group_id: group_id,
    title: value
  }

  var new_group: TaskGroup = await updateTaskGroup(update_item)

  const group_index = items.value.groups.findIndex(group => group.group_id === group_id);
  items.value.groups[group_index].title = value;

  groups.value = parseGroups(items.value);
};

const groups = ref<CollectedTaskGroup[] | undefined>();

watch(
  () => items.value,
  (newItems) => {
    items.value = newItems;
    if (newItems) {
      groups.value = parseGroups(newItems);
    }
  }
);

</script>

<template>
  <div class="agenda" color="transparent">
    <div class="agenda-inner">
      <div class="agenda-header">
        <div class="agenda-title">
          <VaInput class="agenda-title-input input-no-border" v-model="agenda_title"
            @blur="updateAgendaTitle($event.target.value)">
          </VaInput>
        </div>
        <!-- 
        <VaButton class="agenda-updates" preset="secondary" @click="openTaskUpdatesSideUp">
          <div class="agenda-updates-title">
            Updates
          </div>
          <div v-if="items && items.updates.length" class="agenda-updater">
            <VaAvatar size="small" :src="items.updates[0].avatar"></VaAvatar>
          </div>
        </VaButton>
        -->
        <VaButton class="agenda-invite ml-auto" preset="secondary" border-color="primary">
          <VaIcon name="person_add"></VaIcon>
          <div class="agenda-invite-title">Invite</div>
        </VaButton>
        <VaButton preset="secondary">
          <VaMenu>
            <template #anchor>
              <div class="task-agenda-option-menu">
                <VaIcon name="more_horiz" color="secondary"></VaIcon>
              </div>
            </template>
            <VaMenuItem class="task-option">
              <VaButton preset="secondary">
                <VaIcon name="remove" color="secondary"></VaIcon>
                <span class="group-menu-item">Delete Agenda</span>
              </VaButton>
            </VaMenuItem>
          </VaMenu>
        </VaButton>
      </div>
      <div class="agenda-description">
        <VaInput class="agenda-desc input-no-border w-full" v-model="agenda_description"
          @blur="updateAgendaDescription($event.target.value)" :autosize="true"></VaInput>
        <!-- <VaButton preset="secondary" class="agenda-see-more ml-auto" @click="openPopup"> Details </VaButton> -->
      </div>
      <VaTabs v-model="selectedTab">
        <template #tabs>
          <VaTab v-for="tab in ['Table', 'Calendar']" :key="tab">
            {{ tab }}
          </VaTab>
        </template>
      </VaTabs>
      <div class="tab-items">
        <TaskGroups :groups="groups" :loading="isLoading" v-if="selectedTab == 0" @add-task-group="addNewTaskGroup"
          @delete-task-group="deleteTaskGroup" @update-task-group-title="updateTaskGroupTitle" />
        <TaskCalendar :data="calendarData(items?.tasks)" v-if="selectedTab == 1" />
      </div>
    </div>
    <!-- <TaskUpdates :open="taskUpdatesSideUp" @close-sideup="closeSideup"></TaskUpdates> -->
  </div>
  <!-- <TaskInfo :agenda_title="agenda_title ?? ''" :agenda_description="agenda_description ?? ''" :open="agendaDetailsPopup" @close-popup="closePopup"></TaskInfo> -->
</template>

<style lang="scss">
.agenda {
  display: flex;

  th.va-data-table__table-th:first-child {
    padding: 0 !important;
  }

  .agenda-inner {
    display: flex;
    flex-direction: column;
    max-width: 100%;
    width: 100%;

  }

  .agenda-top {
    margin-bottom: 1rem;
  }

  .input-no-border {
    .va-input-wrapper__field {
      border: solid 1px transparent;

      .va-input__content__input {
        border: solid 1px transparent;
      }

      .va-input__content__input:focus {
        border: solid 1px transparent;
      }
    }
  }

  .agenda-description {
    display: flex;
  }

}

.agenda-header {
  display: flex;
  align-items: center;
}

.agenda-updates {
  display: flex;
  margin-left: auto;
}

.agenda-updates::before {
  background: transparent;
}

.agenda-updates-title {
  color: black;
}

.agenda-updater {
  padding-left: 1rem;
}

.agenda-invite .va-icon {
  margin-right: .5rem;
}

.task-option {

  .va-button {
    width: 100%;

    i {
      margin-right: .5rem;
      color: var(--va-info);

    }

    .group-menu-item {
      color: black;
    }
  }
}
</style>