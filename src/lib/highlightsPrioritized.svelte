<script lang="ts">
  import { splitHighlights } from "./splitHighlights";
  import type { Task } from "./types/Task";

  export let task: Task;
  export let __class: string = "";
  $: lines = splitHighlights(task);
</script>

<div class="{__class} flex flex-col">
  {#each lines as line}
    {#if line.group && line.color}
      <p style="background-color: {line.color};" class="rounded-sm">{line.chars}</p>
    {:else}
      <p class="text-sm">{line.chars}</p>
    {/if}
  {/each}
</div>