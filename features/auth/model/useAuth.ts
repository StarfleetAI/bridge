// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { UserManager, type UserManagerSettings, WebStorageStateStore } from 'oidc-client-ts'

import { useProfile } from '~/entities/profile'
import { useResetStores } from '~/shared/lib'

export const useAuth = defineStore('auth', () => {
  const userManager = ref<Nullable<UserManager>>(null)
  const initializeOIDC = () => {
    try {
      const config = useRuntimeConfig()
      const settings: UserManagerSettings = {
        authority: config.public.openidHost as string,
        automaticSilentRenew: true,
        client_id: config.public.openidClientId as string,
        loadUserInfo: true,
        post_logout_redirect_uri: window.location.origin,
        redirect_uri: `${config.public.host}/login`,
        response_type: 'code',
        scope: 'openid profile offline_access',
        silent_redirect_uri: `${config.public.host}/silent-renew`,
        userStore: new WebStorageStateStore({ prefix: 'uplatform_writer_' })
      }
      userManager.value = new UserManager(settings)
    } catch (error) {
      console.log(error)
    }
  }

  initializeOIDC()

  const signInRedirect = () => {
    if (userManager.value) {
      try {
        return userManager.value.signinRedirect()
      } catch (e) {
        console.log(e)
      }
    }
    return null
  }

  const signInCallback = async (path: string) => {
    if (userManager.value) {
      try {
        return await userManager.value.signinCallback(path)
      } catch (e) {
        console.log(e)
        signOut()
      }
    }
    return null
  }

  const renewIsInProgress = ref(false)
  const renewToken = async (): Promise<void> => {
    const profileStore = useProfile()

    const { setupUser } = profileStore
    if (userManager.value && !renewIsInProgress.value) {
      try {
        renewIsInProgress.value = true
        const user = await userManager.value.signinSilent()
        if (user) {
          await setupUser(user)
        }
      } catch (error) {
        console.log(error)
        await signOut()
      } finally {
        renewIsInProgress.value = false
      }
    }
  }

  const getUser = () => {
    if (userManager.value) {
      try {
        return userManager.value.getUser()
      } catch (e) {
        console.log(e)
      }
    }
    return null
  }

  const signOut = async () => {
    if (userManager.value) {
      useResetStores()
      await userManager.value.removeUser()
      await userManager.value.clearStaleState()
      // @ts-expect-error TODO add login route
      navigateTo('/login')
    }
  }
  const $reset = () => {
    initializeOIDC()
  }

  return {
    $reset,
    getUser,
    renewIsInProgress,
    renewToken,
    signInCallback,
    signInRedirect,
    signOut,
    userManager
  }
})
