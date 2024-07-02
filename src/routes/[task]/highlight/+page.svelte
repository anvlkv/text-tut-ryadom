<script lang="ts">
    import { highlightColor, textColor } from "$lib/color";
    import HighlightsArea from "$lib/highlightsArea.svelte";
    import HighlightsList from "$lib/highlightsList.svelte";
    import String from "$lib/string.svelte";
    import type { AppData } from "$lib/types/AppData";
    import { ColorSchema } from "$lib/types/Preferences";
    import { getContext } from "svelte";
    import type { Writable } from "svelte/store";

    const ctx: Writable<AppData> = getContext("appData");

    $: task = $ctx.entries?.find((e) => e.input.id === $ctx.current_entry);

    let highlightGroup: string | null = null;
    let highlightColorIndex: number;

    $: highlightColorIndex = (task?.highlights.length || 0) + 1;
    $: schema = $ctx.activeSchema || ColorSchema.Dull;

    function addHighlight({
        detail,
    }: CustomEvent<{ start: number; end: number }[]>) {
        ctx.update((d) => {
            const task = d.entries?.find(
                (e) => e.input.id === $ctx.current_entry,
            )!;
            const group_id =
                highlightGroup || task.highlights.length.toString();
            detail
                .filter(({ start, end }) => end > start)
                .forEach(({ start, end }) => {
                    task?.highlights.push({
                        text_id: task.input.id,
                        start,
                        end,
                        group_id,
                        color: highlightColorIndex,
                    });
                });
            return d;
        });
        highlightGroup = null;
    }

    function removeHighlight({
        detail: { group, start },
    }: CustomEvent<{ group: string; start: number }>) {
        ctx.update((d) => {
            const highlight_index = task?.highlights.findIndex(
                (h) => h.group_id === group && h.start === start,
            );
            const current_task = d.entries?.find(
                (e) => e.input.id === $ctx.current_entry,
            );
            current_task?.highlights.splice(highlight_index!, 1);
            return d;
        });
    }

    function onSelectGroup({
        detail: { group },
    }: CustomEvent<{ group: string }>) {
        const selected = task?.highlights.find((h) => h.group_id === group)!;
        highlightGroup = selected.group_id;
        highlightColorIndex = selected.color;
    }
</script>

<div class="contents">
    {#if task}
        <h2
            class="w-2/3 text-center mb-8
    text-gray-500
    dull:text-gray-800
    dc:text-white
    lc:text-black
    dark:text-gray-300
    earth:text-emerald-600
    yellow:text-yellow-600
    green:text-green-600
    warm:text-rose-600
    cold:text-blue-600 font-semibold text-lg"
        >
            <String>
                <svelte:fragment slot="familiar">
                    Прочитай текст и выдели его смысловые части:
                </svelte:fragment>
                <svelte:fragment>
                    Прочитайте текст и выделите смысловые фрагменты:
                </svelte:fragment>
            </String>
        </h2>
        <div class="flex relative my-auto">
            <HighlightsArea
                __class="basis-2/3"
                {task}
                on:highlightAdded={addHighlight}
                on:highlightRemoved={removeHighlight}
                --highlight-color={highlightColor(schema, highlightColorIndex)}
                --highlight-text-color={textColor(schema, highlightColorIndex)}
            />
            <HighlightsList
                __class="basis-72"
                {task}
                activeGroup={highlightGroup}
                on:selectGroup={onSelectGroup}
            />
        </div>
    {/if}
</div>
