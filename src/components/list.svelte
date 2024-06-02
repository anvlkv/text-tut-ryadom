<script lang="ts">
  import { getContext } from "svelte";
  import type { AppData } from "../types/AppData";
  import Task from "./task.svelte";
  import type { Writable } from "svelte/store";

  const ctx: Writable<AppData> = getContext("appData");
  const taskHeight = 4;

  $: windowEntries = ($ctx.entries || []).slice(
    $ctx.entries_window?.[0],
    $ctx.entries_window?.[1]
  );
  
  $: heightBefore = ($ctx.entries_window?.[0] || 0) * taskHeight;
  $: heightAfter = Math.max(
    (($ctx.total_entries || 0) - ($ctx.entries_window?.[1] || 0)) * taskHeight,
    0
  );
</script>

<aside>
  <style>
    .task-entry {
      height: var(--entry-height);
    }
  </style>
  <ul style="--entry-height: {`${taskHeight}rem`};" class="overflow-x-hidden overflow-y-auto border-r border-solid border-gray-500">
    <li style={`height: ${heightBefore}rem`} />
    {#each windowEntries as entry (entry.input.id)}
      <li class="task-entry">
        <Task task={entry} isCurrent={$ctx.current_entry === entry.input.id} />
      </li>
    {/each}
    <li style={`height: ${heightAfter}rem`} />
  </ul>
</aside>
