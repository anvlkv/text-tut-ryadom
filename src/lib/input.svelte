<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let name: string;
    export let title: string;
    export let type: string = "text";
    export let placeholder: string = "";
    export let min: string | undefined = undefined;
    export let max: string | undefined = undefined;

    const dispatch = createEventDispatcher();
</script>

<label class="w-80">
    <p class="mb-2">{title}</p>
    <input
        on:change={(e) => dispatch("change", e.currentTarget.value)}
        on:keyup={(e) => dispatch("keyup", e.currentTarget.value)}
        class="w-full p-2 rounded border-2 outline-none
border-gray-400/50 dc:border-white lc:border-black focus:yellow:border-yellow-500 focus:warm:border-orange-500 focus:green:border-emerald-500 focus:cold:border-sky-500 focus:earth:border-lime-600 focus:dc:border-violet-400 focus:lc:border-violet-800 bg-gray-50 text-gray-900 dull:bg-gray-100 dull:text-gray-800 dc:bg-black dc:text-white lc:bg-white lc:text-black dark:bg-gray-900 dark:text-gray-50 earth:bg-earth-50 earth:text-emerald-900 yellow:bg-yellow-50 yellow:text-yellow-900 green:bg-green-50 green:text-green-900 warm:bg-orange-50 warm:text-rose-900 cold:bg-sky-50 cold:text-blue-900"
        {type}
        {placeholder}
        {name}
        {min}
        {max}
    />
</label>
