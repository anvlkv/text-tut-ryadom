<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { Task } from "./types/Task";
  import Highlight from "./highlight.svelte";
  import { invoke } from "@tauri-apps/api";
  import { splitHighlights } from "./splitHighlights";

  export let task: Task;
  export let __class: string = "";

  const dispatch = createEventDispatcher();

  let container: HTMLParagraphElement;

  $: lines = splitHighlights(task);

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
          const offsetStart = parseInt(
            (reverse
              ? selection.focusNode
              : selection.anchorNode)!.parentElement!.getAttribute(
              "data-offset"
            )!
          );
          const offsetEnd = parseInt(
            (reverse
              ? selection.anchorNode
              : selection.focusNode)!.parentElement!.getAttribute(
              "data-offset"
            )!
          );

          const len =
            (reverse ? selection.anchorOffset : selection.focusOffset) -
            (reverse ? selection.focusOffset : selection.anchorOffset);

          const text = selection.toString().slice(0, len);
          const endsWithSpace = text.match(/[ ]*$/);
          const startsWithSpace = text.match(/^[ ]*/);

          const start =
            offsetStart +
            (reverse ? selection.focusOffset : selection.anchorOffset) +
            (startsWithSpace ? startsWithSpace[0].length : 0);
          const end =
            offsetEnd +
            (reverse ? selection.anchorOffset : selection.focusOffset) -
            (endsWithSpace ? endsWithSpace[0].length : 0);

          const splits = task.highlights
            .filter((h) => start <= h.start && end >= h.end)
            .sort((a, b) => a.start - b.start);

          if (splits.length) {
            invoke<[number, number][]>("split_highlight_ranges", {
              highlight: [start, end],
              task,
            }).then((d) => {
              console.log('add multiple');
              dispatch(
                "highlightAdded",
                d.map(([start, end]) => ({ start, end }))
              );
            });
          } else {
            console.log('add one');
            dispatch("highlightAdded", [{ start, end }]);
          }

          selection.removeAllRanges();
        }
      }
    }
  }
</script>

<p
  class="{__class} leading-loose mx-auto max-w-prose p-8"
  bind:this={container}
  on:pointerdown={onPointerDown}
>
  {#each lines as line}
    {#if line.group && line.color}
      <Highlight
        text={line.chars}
        color={line.color}
        onRemove={() =>
          dispatch("highlightRemoved", {
            group: line.group,
            start: line.offset,
          })}
      />
    {:else}
      <span class="select-text" data-offset={line.offset}>{line.chars}</span>
    {/if}
  {/each}
</p>

<svelte:document on:pointerup={onPointerUp} />
