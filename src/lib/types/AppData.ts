import type { Task } from "./Task";

export interface AppData {
  src_dir?: string,
  total_entries?: number,
  entries_window?: [number, number],
  current_entry?: string,
  entries?: Task[]
}