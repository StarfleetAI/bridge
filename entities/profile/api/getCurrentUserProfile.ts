// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { usePassportFetcher } from '~/shared/utils'
import { type Profile } from '../model'

export const getCurrentUserProfile = () => {
  return usePassportFetcher<Profile>({
    endpoint: '/GetCurrentUser',
    method: 'POST'
  })
}
