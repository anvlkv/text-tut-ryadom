<script lang="ts">
  import HighlightsPrioritized from "$lib/highlightsPrioritized.svelte";
  import String from "$lib/string.svelte";
  import type { AppData } from "$lib/types/AppData";
  import { getContext } from "svelte";
  import type { ChangeEventHandler } from "svelte/elements";
  import type { Writable } from "svelte/store";

  const ctx: Writable<AppData> = getContext("appData");

  $: task = $ctx.entries?.find((e) => e.input.id === $ctx.current_entry);
  $: taskIndex = $ctx.entries?.findIndex(
    (e) => e.input.id === $ctx.current_entry
  );

  function updateOutput(value: string) {
    ctx.update((d) => {
      d.entries![taskIndex!].output = {
        ...(d.entries![taskIndex!].output || {
          text_id: task!.input.id,
          completed_ts: null,
        }),
        text: value,
      };

      return d;
    });
  }

  let timer: any;

  const debounce = (value: string) => {
    clearTimeout(timer);
    timer = setTimeout(() => {
      updateOutput(value);
    }, 750);
  };

  const handleChange: ChangeEventHandler<HTMLTextAreaElement> = (e) => {
    const value = e.currentTarget.value;
    updateOutput(value);
  };
  const handleKeyUp: ChangeEventHandler<HTMLTextAreaElement> = (e) => {
    const value = e.currentTarget.value;
    debounce(value);
  };

  function addHighlightedText({detail: {text}}: CustomEvent<{text: string}>) {
    let value = task?.output?.text || ""
    if (value) {
      value += "\n";
    }
    value += text;
    updateOutput(value);
  }
</script>

<div class="contents">
  {#if task}
    <h2 class="w-full text-center mb-8 text-gray-500 
    dull:text-gray-800
    dc:text-white 
    lc:text-black
    dark:text-gray-300
    earth:text-emerald-600
    yellow:text-yellow-600
    green:text-green-600
    warm:text-rose-600
    cold:text-blue-600  font-semibold text-lg">
      <String>
        <svelte:fragment slot="familiar">
          Составь краткое содержание этого текста:
        </svelte:fragment>
        <svelte:fragment>
          Введите краткое содержание текста:
        </svelte:fragment>
      </String>
    </h2>
    <div class="max-w-prose mx-auto flex flex-col my-auto">
      <HighlightsPrioritized {task} on:addHighlightedText={addHighlightedText}/>
      <textarea
        class="my-4 px-4 py-2 min-h-72 rounded border-2 outline-none 
        border-gray-400/50 dc:border-white lc:border-black focus:yellow:border-yellow-500 focus:warm:border-orange-500 focus:green:border-emerald-500 focus:cold:border-sky-500 focus:earth:border-lime-600 focus:dc:border-violet-400 focus:lc:border-violet-800 bg-gray-50 text-gray-900 dull:bg-gray-100 dull:text-gray-800 dc:bg-black dc:text-white lc:bg-white lc:text-black dark:bg-gray-900 dark:text-gray-50 earth:bg-earth-50 earth:text-emerald-900 yellow:bg-yellow-50 yellow:text-yellow-900 green:bg-green-50 green:text-green-900 warm:bg-orange-50 warm:text-rose-900 cold:bg-sky-50 cold:text-blue-900"
        rows="8"
        placeholder="Краткое описание..."
        value={task.output?.text || ""}
        on:change={handleChange}
        on:keyup={handleKeyUp}
      ></textarea>
    </div>
  {/if}
</div>
