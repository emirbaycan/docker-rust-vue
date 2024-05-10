<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { TaskStatus, TaskVisor, UpdateTask, UpdateTaskVisor } from '../../../api/agenda/types';
import { DataTableItem } from 'vuestic-ui/web-components';
import { addTaskVisor, removeTaskVisor, updateTask } from '../../../api/agenda/request';
import { CreateTaskVisor } from '../../../api/agenda/types';

export default defineComponent({
    props: {
        visors: {
            type: Object as PropType<Array<TaskVisor>>,
            required: true,
        },
        task: {
            type: Object as PropType<UpdateTask | DataTableItem>,
            required: true
        },
        email: {
            type: String,
        }
    },
    data() {
        return {
            selectedvisor: null as number | null
        };
    },
    methods: {
        addvisor() {
            var item: CreateTaskVisor = {
                email: this.email ? this.email : "",
                task_id: this.task.task_id,
            }
            addTaskVisor(item);
        },
        deletevisor(item: TaskVisor) {
            removeTaskVisor(item);
        }
    }
})
</script>

<template>
    <div class="task-visor-holder">
        <div v-if="visors.length" class="task-visor-dropdown-holder">
            <VaDropdown v-if="visors[0].visor_id != 0" :close-on-content-click="false" :keyboard-navigation="false"
                :readonly="true">
                <template #anchor>
                    <div>
                        <VaIcon name="supervisor_account" class="va-text-secondary"></VaIcon>
                    </div>
                </template>
                <VaDropdownContent>
                    <div class="task-visors" v-for="(visor, index) in visors" :key="index">
                        <VaButton class="task-visor" @click="deletevisor(visor)">
                            <VaAvatar size="small" :src="visor.avatar" />
                            <span class="task-visor-name">
                                {{ visor.fullname }}
                            </span>
                            <VaIcon name="close" />
                        </VaButton>
                    </div>
                    <div class="task-visor-add">
                        <VaInput v-model="email" placeholder="E-posta ile ekleyin"></VaInput>
                        <VaButton @click="addvisor()">
                            <VaIcon name="add" />
                        </VaButton>
                    </div>
                </VaDropdownContent>
            </VaDropdown>
        </div>
        <VaButton v-else class="task-update-btn">
            <VaMenu :close-on-content-click="false" :keyboard-navigation="false" :readonly="true">
                <template #anchor>
                    <div>
                        <VaIcon name="person_add" class="va-text-secondary"></VaIcon>
                    </div>
                </template>
                <VaMenuItem>
                    <div class="task-visor-add">
                        <VaInput v-model="email" placeholder="E-posta ile ekleyin"></VaInput>
                        <VaButton @click="addvisor()">
                            <VaIcon name="add" />
                        </VaButton>
                    </div>
                </VaMenuItem>
            </VaMenu>
        </VaButton>
    </div>
</template>

<style>
.task-visor {
    margin: auto;
    display: flex;
    align-items: center;
    padding: 0 .5rem;
    position: relative;
    border-radius: 3rem;
    background: var(--va-background-element) !important;
}

.task-visor:hover .va-icon {
    opacity: 1 !important;
}

.task-visor .va-avatar {
    margin-right: .5rem;
}

.task-visor .va-icon.va-icon {
    font-size: 13px !important;
    height: 13px !important;
    line-height: 13px !important;
    position: absolute;
    right: .25rem;
    top: .125rem;
    color: var(--va-danger);
    opacity: 0;
    transition: .5s;
}

.task-visors {
    display: flex;
}

.task-visors .va-button:before {
    background: var(--va-background-secondary)
}

.task-visor:hover {
    background: var(--va-background-secondary);
}

.task-visor-add {
    display: flex;
    margin-top: 1rem;
}

.task-visor-add .va-button {
    border: 0;
    border-radius: 0;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
}

.task-visor-add .va-input-wrapper__field {
    border-radius: 0;
    border: 0;
    border-bottom: solid 1px var(--va-input-wrapper-border-color);
}

.va-menu-item:hover::after {
    background: transparent;
}

.task-visor-name {
    color: black;
    font-weight: 400;
}
</style>