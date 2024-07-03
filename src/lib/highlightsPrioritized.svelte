<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import Highlight from "./highlight.svelte";
    import { splitHighlights } from "./splitHighlights";
    import type { Task } from "./types/Task";

    export let task: Task;
    export let __class: string = "";

    const dispatch = createEventDispatcher();

    $: lines = splitHighlights(task);
</script>

<div class="{__class} flex flex-col leading-8">
    {#each lines as line}
        {#if line.group && line.color}
            <p>
                <Highlight
                    text={line.chars}
                    color={line.color}
                    onClick={() => {
                        dispatch("addHighlightedText", { text: line.chars });
                    }}
                />
            </p>
        {:else}
            <p class="text-xs">{line.chars}</p>
        {/if}
    {/each}
</div>
