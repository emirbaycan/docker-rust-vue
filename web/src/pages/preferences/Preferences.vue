<template>
  <div class="flex flex-col p-4 space-y-10 bg-backgroundSecondary rounded-lg">
    <div class="flex space-x-5">
      <PreferencesHeader />
    </div>
    <div class="space-y-4 md:space-y-6">
      <Settings @openNameModal="isEditNameModalOpen = true" @openUsernameModal="isEditUsernameModalOpen = true"  @openResetPasswordModal="isResetPasswordModalOpen = true" />
    </div>
  </div>
  <EditUsernameModal v-if="isEditUsernameModalOpen" @cancel="isEditUsernameModalOpen = false" />
  <EditNameModal v-if="isEditNameModalOpen" @cancel="isEditNameModalOpen = false" />
  <ResetPasswordModal v-if="isResetPasswordModalOpen" @cancel="isResetPasswordModalOpen = false" />
</template>
<script lang="ts" setup>
import { ref } from 'vue'

import PreferencesHeader from './preferences-header/PreferencesHeader.vue'
import Settings from './settings/Settings.vue'
import EditNameModal from './modals/EditNameModal.vue'
import EditUsernameModal from './modals/EditUsernameModel.vue'
import ResetPasswordModal from './modals/ResetPasswordModal.vue'

const isEditNameModalOpen = ref(false)
const isEditUsernameModalOpen = ref(false)
const isResetPasswordModalOpen = ref(false)
</script>
