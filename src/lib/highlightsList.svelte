<script lang="ts">
    import { createEventDispatcher, getContext, onMount } from "svelte";
    import type { Writable } from "svelte/store";
    import { highlightColor } from "./color";
    import String from "./string.svelte";
    import type { AppData } from "./types/AppData";
    import { ColorSchema } from "./types/Preferences";
    import type { ScrollSync } from "./types/ScrollSync";
    import type { Task } from "./types/Task";

    export let task: Task;
    export let activeGroup: null | string;
    export let __class: string = "";

    const dispatch = createEventDispatcher();

    const ctx: Writable<AppData> = getContext("appData");
    const scrollSync: Writable<ScrollSync> = getContext("scrollSync");

    let listElement: HTMLUListElement;
    let listHeight = 0;
    const resize = new ResizeObserver((e) => {
        listHeight = e[0].contentRect.height;
    });

    $: schema = $ctx.activeSchema || ColorSchema.Dull;

    $: groups = task.highlights.reduce(
        (acc, highlight, i) => {
            const exists = acc.find((a) => a.id === highlight.group_id);
            if (exists) {
                exists.count += 1;
            } else {
                acc.push({
                    label: task.input.text.slice(
                        highlight.start,
                        highlight.end,
                    ),
                    count: 1,
                    id: highlight.group_id,
                    color: highlight.color,
                });
            }

            return acc;
        },
        [] as { label: string; count: number; color: number; id: string }[],
    );

    $: listOffset = $scrollSync.allowFixed
        ? $scrollSync.height > listHeight
            ? $scrollSync.scrollTop
            : $scrollSync.scrollTop -
              $scrollSync.height * (listHeight / $scrollSync.scrollTop)
        : 0;

    onMount(() => {
        resize.observe(listElement);

        return () => {
            resize.disconnect();
        };
    });
</script>

<div class="{__class} max-h-full">
    <ul
        bind:this={listElement}
        class="mt-auto w-72 flex flex-col justify-center items-stretch px-6 border-l border-solid border-gray-400/50 dc:border-white lc:border-black"
    >
        {#if groups.length > 0}
            <li class="mt-auto"></li>
        {/if}
        {#each groups as group (group.id)}
            <li
                class="w-full flex items-center cursor-pointer {activeGroup ===
                    group.id && 'font-bold'}"
            >
                <div
                    style="background-color:{highlightColor(
                        schema,
                        group.color,
                    )};"
                    class="w-4 h-4 shrink-0 grow-0 mr-2 rounded-full"
                ></div>
                <span
                    class="overflow-ellipsis overflow-hidden text-nowrap whitespace-nowrap block"
                    >{group.label}</span
                >
                <button
                    title="Добавить новое выделение к группе"
                    class="ml-2 p-2 rounded-full aspect-square flex items-center justify-center"
                    style="background-color:{highlightColor(
                        schema,
                        group.color,
                    )};"
                    on:click={() =>
                        dispatch("selectGroup", { group: group.id })}
                >
                    <span class="block text-sm leading-none">
                        {#if group.count > 1}
                            +{group.count - 1}
                        {:else}
                            +
                        {/if}
                    </span>
                </button>
            </li>
        {:else}
            <li class="text-center text-gray-500">
                <String>
                    <svelte:fragment slot="familiar">
                        Начни выделять текст, список фрагментов появится здесь
                    </svelte:fragment>
                    <svelte:fragment>Еще ничего не выделено</svelte:fragment>
                </String>
            </li>
        {/each}
        {#if groups.length > 0}
            <li class="text-center mt-auto mb-0 -ml-6 text-gray-500 text-sm">
                Выделено фрагментов: {groups.length}
            </li>
        {/if}
    </ul>
</div>
