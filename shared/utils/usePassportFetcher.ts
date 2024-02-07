// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type BaseRequestParams, useUFetch } from '~/shared/utils'

export const usePassportFetcher = <T>(params: BaseRequestParams) => {
  const config = useRuntimeConfig()
  const { fetcher } = useUFetch(`${config.public.passportApi}/passport.idm.Idm`)

  return fetcher<T>({
    ...params,
    method: params.method || 'POST'
  })
}
