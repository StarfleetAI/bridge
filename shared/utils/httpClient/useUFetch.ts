// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

// eslint-disable-next-line boundaries/element-types
import { useAuth } from '~/features/auth'
// eslint-disable-next-line boundaries/element-types
import { useProfile } from '~/entities/profile'

import { ApiResponseError } from './ApiResponseError'
import { type BaseMeta } from './BaseMeta'
import { type BaseRequestParams } from './BaseRequestParams'
import { HTTPHeaders } from './HTTPHeaders'

const parseResultData = (parsedResponse: string, statusCode: number) => {
  try {
    return JSON.parse(parsedResponse)
  } catch {
    return JSON.parse(JSON.stringify({ data: parsedResponse, statusCode })) // для кейсов когда на 200 возвращается строка
  }
}

export function useUFetch(baseURL: string) {
  const getContentType = (body: unknown, isFormData?: boolean) => {
    const { applicationJson, urlEncoded } = HTTPHeaders.contentType
    if (isFormData) {
      return null
    }
    if (body !== null && body !== 'null' && typeof body === 'string') {
      try {
        JSON.parse(body)
        return applicationJson
      } catch {
        return urlEncoded
      }
    }
    return applicationJson
  }
  // eslint-disable-next-line sonarjs/cognitive-complexity
  const fetcher = async <T>(params: BaseRequestParams): Promise<{ data: T; meta: BaseMeta }> => {
    const { body, endpoint, formData, method } = params
    const { applicationJson } = HTTPHeaders.contentType
    const headers = new Headers({
      Accept: applicationJson
    })
    const contentType = getContentType(params.body, params.formData)
    if (contentType) {
      headers.set('Content-Type', contentType)
    }

    const fullURL = `${baseURL}${endpoint.toString()}`
    const result: { data: Nullable<T>; meta: BaseMeta } = {
      data: null,
      meta: {
        page: 1,
        perPage: 1,
        total: 1,
        totalPages: 1
      }
    }
    const fetchFn = async (retry = true) => {
      const { accessToken } = storeToRefs(useProfile())
      if (accessToken.value) {
        headers.set('Authorization', `Bearer ${accessToken.value}`)
      }
      const response = await fetch(fullURL, {
        body: formData ? (body as FormData) : JSON.stringify(body),
        headers,
        method
      })
      const parsedResponse = await response.text()
      const { headers: resHeaders, status: statusCode } = response

      result.meta = {
        page: resHeaders.get('X-Page') ? Number(resHeaders.get('X-Page')) : 1,
        perPage: resHeaders.get('X-Per-Page') ? Number(resHeaders.get('X-Per-Page')) : 1,
        total: resHeaders.get('X-Total') ? Number(resHeaders.get('X-total')) : 1,
        totalPages: resHeaders.get('X-Total-Pages') ? Number(resHeaders.get('X-Total-Pages')) : 1
      }
      result.data = parseResultData(parsedResponse, statusCode)

      if (response.status === 401 && retry) {
        const { renewToken } = useAuth()
        const { renewIsInProgress } = storeToRefs(useAuth())
        if (renewIsInProgress.value) {
          watchOnce(renewIsInProgress, async () => {
            await fetchFn(false)
          })
        } else {
          await renewToken()
          await fetchFn(false)
        }
        return
      }
      if (!response.ok) {
        const { status, statusText: text } = response
        throw new ApiResponseError({ body: result, status, text })
      }
    }

    await fetchFn()

    return result as { data: T; meta: BaseMeta }
  }

  return {
    fetcher
  }
}
