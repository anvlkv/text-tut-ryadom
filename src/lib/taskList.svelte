<script lang="ts">
  import { page } from "$app/stores";
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import Task from "./task.svelte";
  import type { AppData } from "./types/AppData";

  const ctx: Writable<AppData> = getContext("appData");
  const taskHeight = 5;

  $: windowEntries = ($ctx.entries || []).slice(
    $ctx.entries_window?.[0],
    $ctx.entries_window?.[1]
  );

  $: heightBefore = ($ctx.entries_window?.[0] || 0) * taskHeight;
  $: heightAfter = Math.max(
    (($ctx.total_entries || 0) - ($ctx.entries_window?.[1] || 0)) * taskHeight,
    0
  );

  let container: HTMLUListElement;
  $: {
    const taskId = $page.params['task'];
    container?.querySelector(`#task-${taskId}`)?.scrollIntoView();
  }
</script>

<aside class="basis-1/5 shrink-0 grow-0 text-sm border-r border-solid border-gray-400/50 dc:border-white lc:border-black">
  <style>
    .task-entry {
      height: var(--entry-height);
    }
  </style>
  <ul
    style="--entry-height: {`${taskHeight}rem`};"
    class="overflow-x-hidden overflow-y-auto max-h-full"
    bind:this={container}
  >
    <li style={`height: ${heightBefore}rem`} />
    {#each windowEntries as entry (entry.input.id)}
      <li id="task-{entry.input.id}" class="task-entry">
        <Task task={entry} isCurrent={$ctx.current_entry === entry.input.id} />
      </li>
    {/each}
    <li style={`height: ${heightAfter}rem`} />
  </ul>
</aside>
