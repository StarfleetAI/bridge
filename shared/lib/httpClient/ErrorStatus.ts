// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export enum ErrorStatus {
  UNKNOWN,
  /**
   * error in connecting to repository (Server or Database)
   */
  NO_CONNECTION,
  /**
   * error in getting value (Json Error, Server Error, etc)
   */
  BAD_RESPONSE = 400,
  /**
   * bad credential
   */
  UNAUTHORIZED = 401,
  /**
   * access forbidden
   */
  FORBIDDEN = 403,
  /**
   * resource not found
   */
  NOT_FOUND = 404,
  /**
   * Time out  error
   */
  CONFLICT = 409,
  /**
   * Request Entity Too Large
   */
  REQUEST_TOO_LARGE = 413,
  /**
   *  Conflict with the current state of the target resource
   */
  TIMEOUT = 504,
  /**
   * no data available in repository
   */
  EMPTY_RESPONSE,
  /**
   * an unexpected error
   */
  NOT_DEFINED,
  /**
   * Internal server error
   */
  SERVER_ERROR = 500,
  /**
   * Used when we need to show a custom error message
   */
  CUSTOM = 599,
}
