<script setup lang="ts">
import { PropType, computed, ref, watch } from 'vue'
import { useForm } from 'vuestic-ui'
import { User, UserRole } from '../../../api/users/types'
import UserAvatar from './UserAvatar.vue'
import { validators } from '../../../services/utils'

const props = defineProps({
  user: {
    type: Object as PropType<User | null>,
    default: null,
  },
  saveButtonLabel: {
    type: String,
    default: 'Save',
  },
})

const defaultNewUser: User = {
  id: -1,
  avatar: '',
  fullname: '',
  role: 1,
  username: '',
  notes: '',
  email: '',
  active: true,
}

const newUser = ref<User>({ ...defaultNewUser })

const isFormHasUnsavedChanges = computed(() => {
  return Object.keys(newUser.value).some((key) => {
    if (key === 'avatar') {
      return false
    }

    return newUser.value[key as keyof User] !== (props.user ?? defaultNewUser)?.[key as keyof User]
  })
})

defineExpose({
  isFormHasUnsavedChanges,
})

watch(
  () => props.user,
  () => {
    if (!props.user) {
      return
    }

    newUser.value = {
      ...props.user,
      avatar: props.user.avatar || '',
    }
  },
  { immediate: true },
)

const avatar = ref<File>()

watch(avatar, async (newAvatar) => {
  if (!newAvatar) {
    newUser.value.avatar = ''
    return
  }

  const formData = new FormData()

  formData.append('file', newAvatar)

  const response = await fetch(import.meta.env.VITE_API_URL + 'api/user/user_image', {
    method: 'POST',
    credentials: 'include',
    body: formData,
  })

  const result = await response.json()
  const image: any = result.data.item.name

  newUser.value.avatar = 'http://localhost:1998/images/' + image + "?" + new Date().getTime();
  localStorage.avatar = 'http://localhost:1998/images/' + image+ "?" + new Date().getTime();
})

const form = useForm('add-user-form')

const emit = defineEmits(['close', 'save'])

const onSave = () => {
  if (form.validate()) {
    emit('save', newUser.value)
  }
}

const roleSelectOptions: { text: string; value: UserRole }[] = [
  { text: 'New User', value: 1 },
  { text: 'User', value: 2 },
  { text: 'Admin', value: 3 },
  { text: 'Owner', value: 4 },
]
</script>

<template>
  <VaForm v-slot="{ isValid }" ref="add-user-form" class="flex-col justify-start items-start gap-4 inline-flex w-full">
    <VaFileUpload v-model="avatar" type="single" hide-file-list
      class="self-stretch justify-start items-center gap-4 inline-flex">
      <UserAvatar :user="newUser" size="large" />
      <VaButton preset="primary" size="small">Add image</VaButton>
      <VaButton v-if="avatar" preset="primary" color="danger" size="small" icon="delete" class="z-10"
        @click.stop="avatar = undefined" />
    </VaFileUpload>
    <div class="self-stretch flex-col justify-start items-start gap-4 flex">
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput v-model="newUser.fullname" label="Full name" class="w-full sm:w-1/2" :rules="[validators.required]"
          name="fullname" />
        <VaInput v-model="newUser.username" label="Username" class="w-full sm:w-1/2" :rules="[validators.required]"
          name="username" />
      </div>
      <div class="flex gap-4 flex-col sm:flex-row w-full">
        <VaInput v-model="newUser.email" label="Email" class="w-full sm:w-1/2"
          :rules="[validators.required, validators.email]" name="email" />
      </div>
      <div class="flex gap-4 w-full">
        <div class="w-1/2">
          <VaSelect v-model="newUser.role" label="Role" class="w-full" :options="roleSelectOptions"
            :rules="[validators.required]" name="role" value-by="value" />
        </div>

        <div class="flex items-center w-1/2 mt-4">
          <VaCheckbox v-model="newUser.active" label="Active" class="w-full" name="active" />
        </div>
      </div>

      <VaTextarea v-model="newUser.notes" label="Notes" class="w-full" name="notes" />
      <div class="flex gap-2 flex-col-reverse items-stretch justify-end w-full sm:flex-row sm:items-center">
        <VaButton preset="secondary" color="secondary" @click="$emit('close')">Cancel</VaButton>
        <VaButton :disabled="!isValid" @click="onSave">{{ saveButtonLabel }}</VaButton>
      </div>
    </div>
  </VaForm>
</template>
