import type { ColorSchema, Preferences } from "./Preferences";
import type { Task } from "./Task";

export interface LocalSource {
  src_dir: string
}

export interface LabelStudioSource {
  label_studio_url: string,
  token: string,
  project?: string;
  page?: [number, number];
}

export type Source = LocalSource | LabelStudioSource;

export interface AppData {
  src?: Source,
  total_entries?: number,
  entries_window?: [number, number],
  current_entry?: string,
  entries?: Task[],
  preferences?: Preferences,
  activeSchema?: ColorSchema,
  loading?: boolean
}