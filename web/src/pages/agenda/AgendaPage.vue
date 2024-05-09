<script setup lang="ts">
import { AllTasks, CollectedTaskGroup, Task, TaskUpdate } from '../../api/agenda/types';
import { useItems } from './composables/useTasks';
import TaskGroups from './widgets/TaskGroups.vue';

const { items, isLoading, filters, ...itemsApi } = useItems()

const groups = (items: AllTasks | undefined) => {

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
      if (group.group_id !=0 && task.group_id != group.group_id) {
        return;
      }

      task.updates = updates.filter(update => update.task_id == task.task_id );
      task.supervisors = supervisors.filter(supervisor => supervisor.task_id == task.task_id);;
      task.visors = visors.filter(visor => visor.task_id == task.task_id);;
 
      new_group.tasks.push(task);
      new_group.group_id = task.group_id;

    });
    if(new_group.group_id==0){
      return;
    }
    new_groups.push(new_group);
  });

  return new_groups;
}

</script>


<template>
  <VaCard class="agenda">
    <VaCard class="agenda-top">
      <VaCardContent>
        <div class="flex flex-col md:flex-row gap-2 mb-2 justify-between">
          <VaMenu>
            <template #anchor>
              <VaButton>Actions</VaButton>
            </template>
            <VaMenuItem>
              <VaButton>Add Task Group</VaButton>
            </VaMenuItem>
          </VaMenu>
        </div>
      </VaCardContent>
    </VaCard>
    <TaskGroups :groups="groups(items)"></TaskGroups>
  </VaCard>
</template>
