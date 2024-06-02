<script lang="ts">
  import type { Task } from "../types/Task";

  export let task: Task;

  $: groups = task.highlights.reduce((acc, highlight, i) => {
    const exists = acc.find(a => a.id === highlight.group_id);
    if (exists) {
      exists.count += 1
    }
    else {
      acc.push({
        label: task.input.text.slice(highlight.start, highlight.end),
        count: 1,
        id: highlight.group_id,
        color: highlight.color
      })
    }

    return acc
  }, [] as {label: string, count: number, color: string, id: string}[]);
</script>

<ul class="px-6 border-l border-solid border-gray-500">
  {#each groups as group (group.id)}
    <li class="flex items-center">
      <div style="background-color:{group.color};" class="w-4 h-4 mr-2 rounded-full"></div>
      <span>{group.label}</span>
      {#if group.count > 1}
        <span class="ml-2">+{group.count - 1}</span>
      {/if}
    </li>
  {/each}
</ul>