<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import { highlightColor, textColor } from "./color";
  import type { AppData } from "./types/AppData";
  import { ColorSchema } from "./types/Preferences";

  export let text: string;
  export let color: number;
  export let onRemove: undefined | (() => void) = undefined;

  const ctx: Writable<AppData> = getContext("appData");

  $: schema = $ctx.activeSchema || ColorSchema.Dull;

  $: colorValue = highlightColor(schema, color);
  $: textColorValue = textColor(schema, color);
</script>

<div class="inline relative">
  <span
    style="background-color: {colorValue}; color: {textColorValue};"
    class="rounded-sm py-px">{text}</span
  >{#if onRemove}<button
      style="background-color: {colorValue}; color: {textColorValue};"
      class="text-base font-thin rounded-t-sm leading-none absolute h-3 px-2 -ml-6 -mt-1"
      title="Удалить выделение"
      on:click={onRemove}
    >
      ×
    </button>{/if}
</div>
