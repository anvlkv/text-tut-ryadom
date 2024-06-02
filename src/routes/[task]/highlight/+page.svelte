<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import type { AppData } from "$lib/types/AppData";
  import HighlightsList from "$lib/highlightsList.svelte";
  import HighlightsArea from "$lib/highlightsArea.svelte";
  import { INFO_COLORS } from "$lib/colors";

  const ctx: Writable<AppData> = getContext("appData");

  $: task = $ctx.entries!.find((e) => e.input.id === $ctx.current_entry);

  let highlightGroup: string | null = null;
  let highlightColor: string;

  $: highlightColor =
    INFO_COLORS[(task?.highlights.length || 0) % INFO_COLORS.length];

  function addHighlight({
    detail,
  }: CustomEvent<{ start: number; end: number }[]>) {
    ctx.update((d) => {
      const task = d.entries?.find((e) => e.input.id === $ctx.current_entry)!;
      const group_id = highlightGroup || task.highlights.length.toString();
      detail.forEach(({ start, end }) => {
        task?.highlights.push({
          text_id: task.input.id,
          start,
          end,
          group_id,
          color: highlightColor,
        });
      });
      return d;
    });
    highlightGroup = null;
  }

  function removeHighlight({
    detail: { group, start },
  }: CustomEvent<{ group: string; start: number }>) {
    ctx.update((d) => {
      const highlight_index = task?.highlights.findIndex(
        (h) => h.group_id === group && h.start === start
      );
      const current_task = d.entries?.find(
        (e) => e.input.id === $ctx.current_entry
      );
      current_task?.highlights.splice(highlight_index!, 1);
      return d;
    });
  }

  function onSelectGroup({
    detail: { group },
  }: CustomEvent<{ group: string }>) {
    const selected = task?.highlights.find((h) => h.group_id === group)!;
    highlightGroup = selected.group_id;
    highlightColor = selected.color;
  }
</script>

<div class="contents" style="--highlight-color: {highlightColor};">
  <style>
    .highlight-area *::selection {
      background-color: var(--highlight-color);
    }
  </style>
  {#if task}
    <h2 class="w-2/3 text-center mb-8 text-gray-500 font-semibold text-lg">
      Выдели части текста, сообщающие разные идеи:
    </h2>
    <div class="flex my-auto overflow-y-auto">
      <HighlightsArea
        __class="highlight-area basis-2/3 shrink-0 grow-0"
        {task}
        on:highlightAdded={addHighlight}
        on:highlightRemoved={removeHighlight}
      />
      <HighlightsList
        __class="basis-1/3 shrink-0 grow-0"
        {task}
        activeGroup={highlightGroup}
        on:selectGroup={onSelectGroup}
      />
    </div>
  {/if}
</div>
