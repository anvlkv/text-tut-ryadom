<script lang="ts">
  import Highlight from "./highlight.svelte";
  import { splitHighlights } from "./splitHighlights";
  import type { Task } from "./types/Task";

  export let task: Task;
  export let __class: string = "";
  $: lines = splitHighlights(task);
</script>

<div class="{__class} flex flex-col leading-relaxed">
  {#each lines as line}
    {#if line.group && line.color}
      <p>
        <Highlight text={line.chars} color={line.color} />
      </p>
    {:else}
      <p class="text-sm">{line.chars}</p>
    {/if}
  {/each}
</div>
