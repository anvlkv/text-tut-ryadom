<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import { highlightColor, textColor } from "./color";
  import type { AppData } from "./types/AppData";
  import { ColorSchema } from "./types/Preferences";

  export let text: string;
  export let color: number;
  export let onRemove: undefined | (() => void) = undefined;
  export let onClick: undefined | (() => void) = undefined;

  if (onClick && onRemove) {
    throw new Error(
      "Only one at a time supported onClick or onRemove for highlight",
    );
  }

  const ctx: Writable<AppData> = getContext("appData");

  $: schema = $ctx.activeSchema || ColorSchema.Dull;

  $: colorValue = highlightColor(schema, color);
  $: textColorValue = textColor(schema, color);
  $: style = `background-color: ${colorValue} !important; color: ${textColorValue} !important;`;
</script>

<div class="contents">
  {#if onClick}
    <button class="inline relative text-left" on:click={onClick}>
      <span {style} class="rounded-sm py-px">{text}</span>
    </button>
  {:else}
    <div class="inline relative">
      <span {style} class="rounded-sm py-px">{text}</span>{#if onRemove}<button
          {style}
          class="text-base font-thin rounded-t-sm leading-none absolute h-3 px-2 -ml-6 -mt-1"
          title="Удалить выделение"
          on:click={onRemove}
        >
          ×
        </button>{/if}
    </div>
  {/if}
</div>
