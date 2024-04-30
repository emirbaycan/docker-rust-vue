<template>
  <VaForm ref="form" @submit.prevent="submit">
    <h1 class="font-semibold text-4xl mb-4">Sign up</h1>
    <p class="text-base mb-4 leading-5">
      Have an account?
      <RouterLink :to="{ name: 'login' }" class="font-semibold text-primary">Login</RouterLink>
    </p>
    <VaInput v-model="formData.email"
      :rules="[(v) => !!v || 'Email field is required', (v) => /.+@.+\..+/.test(v) || 'Email should be valid']"
      class="mb-4" label="Email" type="email" />
    <VaValue v-slot="isPasswordVisible" :default-value="false">
      <VaInput ref="password1" v-model="formData.password" :rules="passwordRules"
        :type="isPasswordVisible.value ? 'text' : 'password'" class="mb-4" label="Password"
        messages="Password should be 8+ characters: letters, numbers, and special characters."
        @clickAppendInner.stop="isPasswordVisible.value = !isPasswordVisible.value">
        <template #appendInner>
          <VaIcon :name="isPasswordVisible.value ? 'mso-visibility_off' : 'mso-visibility'" class="cursor-pointer"
            color="secondary" />
        </template>
      </VaInput>
      <VaInput ref="password2" v-model="formData.repeatPassword" :rules="[
        (v) => !!v || 'Repeat Password field is required',
        (v) => v === formData.password || 'Passwords don\'t match',
      ]" :type="isPasswordVisible.value ? 'text' : 'password'" class="mb-4" label="Repeat Password"
        @clickAppendInner.stop="isPasswordVisible.value = !isPasswordVisible.value">
        <template #appendInner>
          <VaIcon :name="isPasswordVisible.value ? 'mso-visibility_off' : 'mso-visibility'" class="cursor-pointer"
            color="secondary" />
        </template>
      </VaInput>
    </VaValue>

    <div class="flex justify-center mt-4">
      <VaButton class="w-full" @click="submit"> Create account</VaButton>
    </div>
  </VaForm>
</template>

<script lang="ts" setup>
import { reactive } from 'vue'
import { useRouter } from 'vue-router'
import { useForm, useToast } from 'vuestic-ui'

const { validate } = useForm('form')
const { push } = useRouter()
const { init } = useToast()

const formData = reactive({
  email: '',
  password: '',
  repeatPassword: '',
})

const submit = async () => {
  if (validate()) {
    const response = await fetch(import.meta.env.VITE_API_URL + 'auth/register', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      credentials: 'include',
      body: JSON.stringify(formData),
    })

    const result = await response.json()

    if (result.status == 'success') {
      let user = result.data;
      localStorage.last_login = new Date().getTime();
      localStorage.id = user.id;
      localStorage.username = user.username;
      localStorage.email = user.email;
      localStorage.fullname = user.fullname;
      localStorage.role = user.role;
      localStorage.avatar = user.avatar;
      localStorage.active = user.active;
      
      init({ message: "You've successfully signed up", color: 'success' })
      push({ name: 'dashboard' })
    } else {
      init({ message: result.message, color: 'warning' })
    }

  }
}

const passwordRules: ((v: string) => boolean | string)[] = [
  (v) => !!v || 'Password field is required',
  (v) => (v && v.length >= 8) || 'Password must be at least 8 characters long',
  (v) => (v && /[A-Za-z]/.test(v)) || 'Password must contain at least one letter',
  (v) => (v && /\d/.test(v)) || 'Password must contain at least one number',
  (v) => (v && /[!@#$%^&*(),.?":{}|<>]/.test(v)) || 'Password must contain at least one special character',
]
</script>
