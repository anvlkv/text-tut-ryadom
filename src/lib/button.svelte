<script lang="ts">
    import { createEventDispatcher } from "svelte";

  export let disabled = false;
  export let cta = false;
  export let title: null | undefined | string = "";
  export let form: null | undefined | string = null;
  export let type: "button" | "submit" | "reset" | null | undefined = null;
  export let __class = "";
  const dispatch = createEventDispatcher();
</script>

<button
  {disabled}
  {title}
  {form}
  {type}
  on:click={() => dispatch("click")}
  class="{__class} rounded-full text-lg px-4 py-2 border-2 border-gray-400 yellow:border-yellow-400 warm:border-orange-400 green:border-emerald-400 cold:border-sky-400 dc:border-white lc:border-black earth:border-lime-600 {cta &&
    'text-gray-50 dc:text-black dark:text-gray-950 bg-violet-600 dark:bg-violet-300 dc:bg-violet-50 lc:bg-violet-900 dull:bg-slate-900 earth:bg-emerald-800 warm:bg-amber-600 cold:divide-blue-600 yellow:bg-orange-600 green:bg-lime-600'} {disabled &&
    'opacity-50'}"
>
  <slot />
</button>
