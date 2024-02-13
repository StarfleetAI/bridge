// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

const MediaQueries = {
  FROM_DESKTOP_MEDIUM: '(min-width: 1279px)',
  IS_DESKTOP_MEDIUM: '(max-width: 1279px)',
  IS_DESKTOP_SMALL: '(min-width: 1024px)',
  IS_LAPTOP: '(min-width: 1024px) and (max-width: 1279px)',
  IS_MOBILE: '(max-width: 767px)',
  IS_TABLET: '(max-width: 1023px)',
  IS_TABLET_MEDIUM: '(max-width: 987px)',
  IS_TABLET_ONLY: '(min-width: 768px) and (max-width: 1023px)'
} as const

export const useScreenSize = defineStore('screenSize', () => {
  const isMobile = eagerComputed(
    () => {
      return useMediaQuery(MediaQueries.IS_MOBILE).value
    },
    { flush: 'pre' }
  )
  const isTabletOnly = eagerComputed(
    () => {
      return useMediaQuery(MediaQueries.IS_TABLET_ONLY).value
    },
    {
      flush: 'pre'
    }
  )
  const isTablet = eagerComputed(
    () => {
      return useMediaQuery(MediaQueries.IS_TABLET).value
    },
    { flush: 'pre' }
  )
  const isTabletMedium = eagerComputed(
    () => {
      return useMediaQuery(MediaQueries.IS_TABLET_MEDIUM).value
    },
    { flush: 'pre' }
  )
  const isLaptop = eagerComputed(
    () => {
      return useMediaQuery(MediaQueries.IS_LAPTOP).value
    },
    { flush: 'pre' }
  )

  const isDesktopSmall = eagerComputed(
    () => {
      return useMediaQuery(MediaQueries.IS_DESKTOP_SMALL).value
    },
    { flush: 'pre' }
  )
  const isDesktopMedium = eagerComputed(
    () => {
      return useMediaQuery(MediaQueries.IS_DESKTOP_MEDIUM).value
    },
    { flush: 'pre' }
  )
  const fromDesktopMedium = eagerComputed(
    () => {
      return useMediaQuery(MediaQueries.FROM_DESKTOP_MEDIUM).value
    },
    { flush: 'pre' }
  )
  const $reset = () => ({})
  return {
    $reset,
    fromDesktopMedium,
    isDesktopMedium,
    isDesktopSmall,
    isLaptop,
    isMobile,
    isTablet,
    isTabletMedium,
    isTabletOnly
  }
})
