<script setup lang="ts">
import { AllTasks, CollectedTaskGroup, Task } from '../../api/agenda/types';
import { useItems } from './composables/useTasks';
import TaskGroups from './widgets/TaskGroups.vue';

const { items, isLoading, filters, ...itemsApi } = useItems()
 
const groups = (items: AllTasks|undefined) => {

  if(!items){
    return;
  }
  var groups = items.groups;
  var tasks = items.tasks;
  var updates = items.updates;
  var supervisors = items.supervisors;
  var visors = items.visors;
 
  var new_groups: Array<CollectedTaskGroup> = []

  tasks.forEach(task => {
    var group: CollectedTaskGroup = {
      group_id: 0,
      agenda_id: 0,
      title: "",
      tasks: []
    };

    task.updates = updates;
    task.supervisors = supervisors;
    task.visors = visors;

    if (group.title == "") {
      for (var i = 0; i < groups.length; i++) {
        var g = groups[i];
        if (g.group_id == task.group_id) {
          group.title = group.title;
          group.agenda_id = group.agenda_id;
          break;
        }
      }
    }

    group.tasks.push(task);
    group.group_id = task.group_id;

    new_groups.push(group);
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
