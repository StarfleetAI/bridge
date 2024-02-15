// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type IdmUserProfile } from '~/shared/api/passport'
import { usePassportFetcher } from '~/shared/lib'
import { type GetUserProfilesParams } from '../model'

export const getUserProfiles = async (params: GetUserProfilesParams) => {
  const {
    data: { userProfiles }
  } = await usePassportFetcher<{ userProfiles: IdmUserProfile[] }>({
    body: params,
    endpoint: '/ListUserProfiles'
  })

  return userProfiles
}
