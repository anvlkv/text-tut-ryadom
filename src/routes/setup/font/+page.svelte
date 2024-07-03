<script lang="ts">
    import String from "$lib/string.svelte";
    import type { AppData } from "$lib/types/AppData";
    import { getContext, onMount } from "svelte";
    import type { ChangeEventHandler } from "svelte/elements";
    import type { Writable } from "svelte/store";
    import { BASE_FONT_SIZE, MAX_FONT_SIZE, MIN_FONT_SIZE } from "$lib/values";

    const ctx: Writable<AppData> = getContext("appData");

    $: fontSize = $ctx.preferences?.fontSize || BASE_FONT_SIZE;

    const fontChange: ChangeEventHandler<HTMLInputElement> = (e) => {
        ctx.update((d) => {
            d.preferences = {
                ...d.preferences!,
                fontSize: e.currentTarget.valueAsNumber,
            };
            return d;
        });
    };

    onMount(() => {
        window.scrollTo({ top: 0, behavior: "smooth" });
    });
</script>

<div class="flex flex-col items-center mb-8">
    <h3 class="text-xl text-center">Шрифт</h3>
    <h4 class="text-lg text-center">
        <String>
            <svelte:fragment slot="familiar">
                Какой шрифт тебе удобнее читать?
            </svelte:fragment>
            <svelte:fragment>Выберите размер шрифта:</svelte:fragment>
        </String>
    </h4>
    <label class="overflow-hidden w-full">
        <div class="flex w-full justify-between items-baseline">
            <span class="block" style="font-size: 12px !important;">Aa</span>
            <span class="block" style="font-size: 27px !important;">Aa</span>
            <span class="block" style="font-size: 42px !important;">Aa</span>
        </div>
        <style>
            input[type="range"]::-webkit-slider-thumb,
            input[type="range"]::-moz-range-thumb {
                outline-color: #7c3aed;
                outline-width: 2px;
            }
        </style>
        <input
            class="w-full appearance-none h-2 my-2 rounded-full bg-gray-700 dark:bg-gray-300 dc:bg-white lc:bg-black dull:bg-gray-500 warm:bg-red-800 cold:bg-blue-800 green:bg-green-800 yellow:bg-yellow-800 earth:bg-emerald-800"
            name="font-size"
            value={fontSize}
            on:change={fontChange}
            type="range"
            min={MIN_FONT_SIZE}
            max={MAX_FONT_SIZE}
            step="1"
        />
    </label>
</div>
