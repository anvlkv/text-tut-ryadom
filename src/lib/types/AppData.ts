import type { ColorSchema, Preferences } from "./Preferences";
import type { Task } from "./Task";

export interface LabelStudioSource {
  label_studio_url: string;
  token: string;
  project: string;
  view: number;
  page_size: number;
}

export interface AppData {
  src_dir?: string;
  label_studio_src?: LabelStudioSource;
  total_entries?: number;
  current_entry?: string;
  entries?: Task[];
  preferences?: Preferences;
  activeSchema?: ColorSchema;
  loading?: boolean;
}
