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
</script>

<div
    class="contents select-none"
    style="--background-color: {colorValue}; --text-color: {textColorValue}"
>
    {#if onClick}
        <button class="inline relative text-left" on:click={onClick}>
            <span class="rounded-sm py-px">{text}</span>
        </button>
    {:else}
        <div class="inline relative">
            <span class="rounded-sm py-px"
                >{text}{#if onRemove}<button
                        class="text-xs font-thin rounded-t-sm absolute h-min px-2 right-0"
                        style="bottom: 1.5em; line-height: 0.75;"
                        title="Удалить выделение"
                        on:click={onRemove}
                    >
                        ×
                    </button>{/if}</span
            >
        </div>
    {/if}
</div>

<style>
    span,
    button {
        background-color: var(--background-color);
        color: var(--text-color);
    }
</style>
