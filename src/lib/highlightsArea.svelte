<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { createEventDispatcher, getContext, onMount } from "svelte";
    import Highlight from "./highlight.svelte";
    import { splitHighlights } from "./splitHighlights";
    import type { Task } from "./types/Task";
    import type { Writable } from "svelte/store";
    import type { ScrollSync } from "./types/ScrollSync";
    import { type Token } from "$lib/types/Token";
    import Words from "./words.svelte";

    export let task: Task;
    export let __class: string = "";

    const dispatch = createEventDispatcher();

    let container: HTMLParagraphElement;
    let scrollSync: Writable<ScrollSync> = getContext("scrollSync");

    $: lines = splitHighlights(task);

    const resize = new ResizeObserver((e) => {
        const height = e[0].contentRect.height;
        scrollSync.update((d) => {
            d.allowFixed = height > d.height;
            return d;
        });
    });

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
                    const reverse =
                        selection.anchorOffset > selection.focusOffset;
                    const offsetStart = parseInt(
                        (reverse
                            ? selection.focusNode
                            : selection.anchorNode)!.parentElement!.getAttribute(
                            "data-offset",
                        )!,
                    );
                    const offsetEnd = parseInt(
                        (reverse
                            ? selection.anchorNode
                            : selection.focusNode)!.parentElement!.getAttribute(
                            "data-offset",
                        )!,
                    );

                    const len =
                        (reverse
                            ? selection.anchorOffset
                            : selection.focusOffset) -
                        (reverse
                            ? selection.focusOffset
                            : selection.anchorOffset);

                    const text = selection.toString().slice(0, len);
                    const endsWithSpace = text.match(/[ ]*$/);
                    const startsWithSpace = text.match(/^[ ]*/);

                    const start =
                        offsetStart +
                        (reverse
                            ? selection.focusOffset
                            : selection.anchorOffset) +
                        (startsWithSpace ? startsWithSpace[0].length : 0);
                    const end =
                        offsetEnd +
                        (reverse
                            ? selection.anchorOffset
                            : selection.focusOffset) -
                        (endsWithSpace ? endsWithSpace[0].length : 0);

                    const splits = task.highlights
                        .filter((h) => start <= h.start && end >= h.end)
                        .sort((a, b) => a.start - b.start);

                    if (splits.length) {
                        invoke<[number, number][]>("split_highlight_ranges", {
                            highlight: [start, end],
                            task,
                        }).then((d) => {
                            console.log("add multiple");
                            dispatch(
                                "highlightAdded",
                                d.map(([start, end]) => ({ start, end })),
                            );
                        });
                    } else {
                        console.log(`add one ${start}:${end}`);
                        dispatch("highlightAdded", [{ start, end }]);
                    }

                    selection.removeAllRanges();
                }
            }
        }
    }

    function onWordClicked({ detail }: CustomEvent<Token>) {
        startSelection = false;
        const start = detail.start;
        const end = detail.end;
        dispatch("highlightAdded", [{ start, end }]);
    }

    onMount(() => {
        resize.observe(container);

        return () => {
            resize.disconnect();
        };
    });
</script>

<p
    class="{__class} highlight-area select-text leading-loose mx-auto max-w-prose h-max p-8"
    bind:this={container}
    on:pointerdown={onPointerDown}
>
    {#each lines as line}{#if line.group && line.color}<Highlight
                text={line.chars}
                color={line.color}
                onRemove={() =>
                    dispatch("highlightRemoved", {
                        group: line.group,
                        start: line.offset,
                    })}
            />{:else}<Words
                chars={line.chars}
                offset={line.offset}
                on:wordClicked={(e) => onWordClicked(e)}
            />{/if}{/each}
</p>

<svelte:document on:pointerup={onPointerUp} />
