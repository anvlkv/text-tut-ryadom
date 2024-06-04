<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import type { AppData } from "$lib/types/AppData";
  import HighlightsPrioritized from "$lib/highlightsPrioritized.svelte";
  import type { ChangeEventHandler } from "svelte/elements";
  import String from "$lib/string.svelte";

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
</script>

<div class="contents">
  {#if task}
    <h2 class="w-full text-center mb-8 text-gray-500 font-semibold text-lg">
      <String>
        <svelte:fragment slot="familiar">
          Составь краткое содержание этого текста:
        </svelte:fragment>
        <svelte:fragment>
          Введите краткое содержание текста:
        </svelte:fragment>
      </String>
    </h2>
    <div class="max-w-prose mx-auto flex flex-col my-auto overflow-y-auto overflow-x-visible">
      <HighlightsPrioritized {task} />
      <textarea
        class="my-4 px-4 py-2 min-h-72 rounded border-2 outline-none border-gray-400/50 dc:border-white lc:border-black focus:yellow:border-yellow-500 focus:warm:border-orange-500 focus:green:border-emerald-500 focus:cold:border-sky-500 focus:earth:border-lime-600 focus:dc:border-violet-400 focus:lc:border-violet-800"
        rows="8"
        placeholder="Краткое описание..."
        value={task.output?.text || ""}
        on:change={handleChange}
        on:keyup={handleKeyUp}
      ></textarea>
    </div>
  {/if}
</div>
