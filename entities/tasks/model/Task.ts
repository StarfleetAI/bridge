// Copyright 2024 StarfleetAI
// SPDX-License-Identifier: Apache-2.0

export type Status = "Todo" | "InProgress" | "WaitingForUser" | "Paused" | "Done" | "Failed" | "Canceled" | "New";

export interface Task {
  id: number;
  agent_id: number;
  origin_chat_id?: number;
  control_chat_id?: number;
  execution_chat_id?: number;
  title: string;
  summary: string;
  status: Status;
  ancestry?: string;
  ancestry_level: number;
  created_at: Date;
  updated_at: Date;
}
