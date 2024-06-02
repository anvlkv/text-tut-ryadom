<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { Task } from "../types/Task";
  import Highlight from "./highlight.svelte";

  export let task: Task;

  const dispatch = createEventDispatcher();

  let container: HTMLParagraphElement;

  $: lines = task.input.text.split("").reduce(
    (acc, char, at) => {
      const highlight = task.highlights.find(
        (h) => h.start <= at && h.end > at
      );
      const last = acc[acc.length - 1];
      console.log('adding char', char, last, task.highlights);
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
    [] as { chars: string; group?: string; color?: string; offset: number }[]
  );

  let startSelection = false;
  function onPointerDown(e: any) {
    startSelection = true;
  }

  function onPointerUp(e: any) {
    if (startSelection) {
      startSelection = false;
      const selection = document.getSelection();
      if (selection && !selection.isCollapsed) {
        if (
          container.contains(selection.anchorNode) &&
          container.contains(selection.focusNode)
        ) {
          const reverse = selection.anchorOffset > selection.focusOffset;
          const offset = parseInt(
            (reverse
              ? selection.focusNode
              : selection.anchorNode)!.parentElement!.getAttribute(
              "data-offset"
            )!
          );

          const start =
            offset + (reverse ? selection.focusOffset : selection.anchorOffset);
          const end =
            offset + (reverse ? selection.anchorOffset : selection.focusOffset);
          dispatch("highlightAdded", { start, end });
          selection.removeAllRanges();
        }
      }
    }
  }
</script>

<p class="leading-loose max-w-prose p-8" bind:this={container} on:pointerdown={onPointerDown}>
  {#each lines as line}
    {#if line.group && line.color}
      <Highlight
        text={line.chars}
        color={line.color}
        onRemove={() => dispatch("highlightRemoved", {group: line.group, start: line.offset})}
      />
    {:else}
      <span class="select-text" data-offset={line.offset}>{line.chars}</span>
    {/if}
  {/each}
</p>

<svelte:document on:pointerup={onPointerUp} />
