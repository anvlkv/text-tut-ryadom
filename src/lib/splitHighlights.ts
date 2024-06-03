import type { Task } from "./types/Task";

export function splitHighlights(task: Task) {
  return task.input.text.split("").reduce(
    (acc, char, at) => {
      const highlight = task.highlights.find(
        (h) => h.start <= at && h.end > at
      );
      const last = acc[acc.length - 1];

      if (last && last.group === highlight?.group_id) {
        last.chars += char;
      } else if (highlight) {
        acc.push({
          chars: char,
          group: highlight.group_id,
          color: highlight.color,
          offset: at,
        });
      } else {
        acc.push({
          chars: char,
          offset: at,
        });
      }
      return acc;
    },
    [] as { chars: string; group?: string; color?: number; offset: number }[]
  )
}