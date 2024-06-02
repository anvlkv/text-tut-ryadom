<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import type { AppData } from "../../../types/AppData";
  import HighlightsList from "../../../components/highlightsList.svelte";
  import HighlightsArea from "../../../components/highlightsArea.svelte";
  
  const ctx: Writable<AppData> = getContext('appData');

  $: task = $ctx.entries!.find(e => e.input.id === $ctx.current_entry);

  function addHighlight({detail: {start, end}} : CustomEvent<{start: number, end: number}>) {
    ctx.update(d => {
      const task = d.entries?.find(e => e.input.id === $ctx.current_entry);
      task?.highlights.push({
        text_id: task.input.id,
        start,
        end,
        group_id: task.highlights.length.toString(),
        color: 'orange'
      })
      return d
    })
  }

  function removeHighlight({detail: {group, start}}: CustomEvent<{group: string, start: number}>) {
    ctx.update(d => {
      const highlight_index = task?.highlights.findIndex(h => h.group_id === group && h.start === start);
      const current_task = d.entries?.find(e => e.input.id === $ctx.current_entry);
      current_task?.highlights.splice(highlight_index!, 1)
      return d
    })
  }
</script>

<div class="flex">
  {#if task}
    <HighlightsArea {task} on:highlightAdded={addHighlight} on:highlightRemoved={removeHighlight}/>
    <HighlightsList {task}/>
  {/if}
</div>