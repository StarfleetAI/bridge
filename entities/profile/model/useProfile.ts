// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type User } from 'oidc-client-ts'

import { getCurrentUserProfile } from '../api'
import { type Profile } from './Profile'

export const useProfile = defineStore('profile', () => {
  const profile = ref<Nullable<Profile>>(null)
  const accessToken = ref('')
  const idToken = ref('')

  const isLoggedIn = computed(() => !!accessToken.value)

  const setupUser = async (user: User) => {
    accessToken.value = user.access_token
    try {
      const { data } = await getCurrentUserProfile()
      profile.value = data

      if (user.id_token) {
        idToken.value = user.id_token
      } else {
        console.warn('No ID token provided!')
      }
    } catch (e) {
      console.warn(e)
    }
  }

  const resetUser = () => {
    profile.value = null
  }
  const firstName = computed(() => profile.value?.firstName || '')
  const lastName = computed(() => profile.value?.lastName || '')
  const fullName = computed(() => `${firstName.value} ${lastName.value}`)
  const $reset = () => {
    accessToken.value = ''
    profile.value = null
    idToken.value = ''
  }
  return {
    $reset,
    accessToken,
    fullName,
    idToken,
    isLoggedIn,
    profile,
    resetUser,
    setupUser
  }
})
