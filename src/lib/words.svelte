<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { type Token } from "$lib/types/Token";

    export let chars: string;
    export let offset: number;

    const dispatch = createEventDispatcher();

    $: tokens = Array.from(chars.split("")).reduce((acc, ch) => {
        const last = acc[acc.length - 1];
        const isWord = (ch.match(/\p{Letter}/u) || ch.match(/\d/)) !== null;
        if (isWord && last?.word) {
            last.tok += ch;
            last.end += 1;
        } else {
            const start = last?.end || 0 + offset;
            acc.push({ tok: ch, word: isWord, start: start, end: start + 1 });
        }
        return acc;
    }, [] as Token[]);
</script>

<span class="select-text"
    >{#each tokens as token}{#if token.word}<span
                tabindex="0"
                role="button"
                on:keyup={() => dispatch("wordClicked", token)}
                on:click={() => dispatch("wordClicked", token)}
                data-offset={token.start}>{token.tok}</span
            >{:else}<span data-offset={token.start}>{token.tok}</span
            >{/if}{/each}</span
>

<style>
    .select-text::selection,
    .select-text *::selection {
        background-color: var(--highlight-color) !important;
        color: var(--highlight-text-color) !important;
    }
</style>
