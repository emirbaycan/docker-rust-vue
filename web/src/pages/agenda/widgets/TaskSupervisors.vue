<script lang="ts">
import { defineComponent, PropType } from 'vue';
import { TaskStatus, TaskSuperVisor, UpdateTask, UpdateTaskSuperVisor } from '../../../api/agenda/types';
import { DataTableItem } from 'vuestic-ui/web-components';
import { addTaskSupervisor, removeTaskSupervisor, updateTask } from '../../../api/agenda/request';
import { CreateTaskSuperVisor } from '../../../api/agenda/types';

export default defineComponent({
    props: {
        supervisors: {
            type: Object as PropType<Array<TaskSuperVisor>>,
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
            selectedSupervisor: null as number | null,
            email: ''
        };
    },
    methods: {
        addSupervisor() {
            var item: CreateTaskSuperVisor = {
                email: this.email ? this.email : "",
                task_id: this.task.task_id,
            }
            addTaskSupervisor(item);
        },
        deleteSupervisor(item: TaskSuperVisor) {
            removeTaskSupervisor(item);
        }
    }
})
</script>

<template>
    <div class="task-supervisor-holder">
        <div v-if="supervisors.length" class="task-supervisor-dropdown-holder">
            <VaDropdown v-if="supervisors[0].supervisor_id!=0" :close-on-content-click="false" :keyboard-navigation="false" :readonly="true">
                <template #anchor>
                    <div>
                        <VaIcon name="supervisor_account" class="va-text-secondary"></VaIcon>
                    </div>
                </template>
                <VaDropdownContent> 
                    <div class="task-supervisors" v-for="(supervisor, index) in supervisors" :key="index">
                        <VaButton class="task-supervisor" @click="deleteSupervisor(supervisor)">
                            <VaAvatar size="small" :src="supervisor.avatar" />
                            <span class="task-supervisor-name">
                                {{ supervisor.fullname }}
                            </span>
                            <VaIcon name="close" />
                        </VaButton>
                    </div>
                    <div class="task-supervisor-add">
                        <VaInput v-model="email" placeholder="E-posta ile ekleyin"></VaInput>
                        <VaButton @click="addSupervisor()">
                            <VaIcon name="add" />
                        </VaButton>
                    </div>
                </VaDropdownContent>
            </VaDropdown>
        </div>
        <VaButton v-else class="task-supervisor-btn">
            <VaDropdown :close-on-content-click="false" :keyboard-navigation="false" :readonly="true">
                <template #anchor>
                    <div>
                        <VaIcon name="person_add" class="va-text-secondary"></VaIcon>
                    </div>
                </template>
                <VaDropdownContent>
                    <div class="task-supervisor-add">
                        <VaInput v-model="email" placeholder="E-posta ile ekleyin"></VaInput>
                        <VaButton @click="addSupervisor()">
                            <VaIcon name="add" />
                        </VaButton>
                    </div>
                </VaDropdownContent>
            </VaDropdown>
        </VaButton>
    </div>
</template>

<style>

.task-supervisor-btn::before{
    display:none;
}
.task-supervisor-holder{
    display:flex;
    align-items:center;
    justify-content: center;
}
.task-supervisor {
    margin: auto;
    display: flex;
    align-items: center;
    padding: 0 .5rem;
    position: relative;
    border-radius: 3rem;
    background: var(--va-background-element) !important;
}

.task-supervisor:hover .va-icon {
    opacity: 1 !important;
}

.task-supervisor .va-avatar {
    margin-right: .5rem;
}

.task-supervisor .va-icon.va-icon {
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

.task-supervisors {
    display: flex;
}

.task-supervisors .va-button:before{
    background:var(--va-background-secondary)
}

.task-supervisor:hover {
    background: var(--va-background-secondary);
}

.task-supervisor-add {
    display: flex;
    margin-top: 1rem;
}

.task-supervisor-add .va-button {
    border: 0;
    border-radius: 0;
    border-top-right-radius: 4px;
    border-bottom-right-radius: 4px;
}

.task-supervisor-add .va-input-wrapper__field {
    border-radius: 0;
    border: 0;
    border-bottom: solid 1px var(--va-input-wrapper-border-color);
}

.va-menu-item:hover::after {
    background: transparent;
}

.task-supervisor-name {
    color: black;
    font-weight: 400;
}

.task-supervisor-dropdown-holder {
    display: flex;
    align-items: center;
    justify-content: center;
}

</style>