// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

import { type components } from '~/shared/api/passport'

export type Profile = components['schemas']['idmUser'] & { avatar?: string }
