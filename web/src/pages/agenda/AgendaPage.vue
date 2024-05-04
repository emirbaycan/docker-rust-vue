<script setup lang="ts">
import { Task } from '../../api/agenda/types';
import { useItems } from './composables/useTasks';
import TaskGroups from './widgets/TaskGroups.vue';

const { tasks, isLoading, filters, ...itemsApi } = useItems()

type DisplayTaskGroup = {
    group_id: number
    agenda_id: number
    title: string
    tasks: Array<Task>
}

const groups = (tasks: Array<Task>) => {
  var task: Task;
  var groups: Array<DisplayTaskGroup> = [];

  tasks.forEach(task => {
    var group: DisplayTaskGroup = {
      group_id: 0,
      agenda_id: 0,
      title: "",
      tasks: []
    };
    group.tasks.push(task);
    group.group_id = task.group_id;
  });

  return groups;
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
    <TaskGroups :groups="groups(tasks)"></TaskGroups>
  </VaCard>
</template>
