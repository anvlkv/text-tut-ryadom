<script lang="ts">
  import { createEventDispatcher, getContext } from "svelte";
  import type { Task } from "./types/Task";
  import { highlightColor } from "./color";
  import type { Writable } from "svelte/store";
  import type { AppData } from "./types/AppData";
  import { ColorSchema } from "./types/Preferences";

  export let task: Task;
  export let activeGroup: null | string;
  export let __class: string = "";

  const dispatch = createEventDispatcher();

  const ctx: Writable<AppData> = getContext('appData');

  $: schema = $ctx.activeSchema || ColorSchema.Dull;

  $: groups = task.highlights.reduce(
    (acc, highlight, i) => {
      const exists = acc.find((a) => a.id === highlight.group_id);
      if (exists) {
        exists.count += 1;
      } else {
        acc.push({
          label: task.input.text.slice(highlight.start, highlight.end),
          count: 1,
          id: highlight.group_id,
          color: highlight.color,
        });
      }

      return acc;
    },
    [] as { label: string; count: number; color: number; id: string }[]
  );
</script>

<ul
  class="{__class} flex flex-col justify-center items-stretch px-6 border-l border-solid border-gray-500 overflow-x-hidden overflow-y-auto box-border"
>
  {#if groups.length > 0}
    <li class="mt-auto"></li>
  {/if}
  {#each groups as group (group.id)}
    <li
      class="w-full flex items-center cursor-pointer {activeGroup ===
        group.id && 'font-bold'}"
    >
      <button
        title="Добавить новое выделение к группе"
        class="contents"
        on:click={() => dispatch("selectGroup", { group: group.id })}
      >
        <div
          style="background-color:{highlightColor(schema, group.color)};"
          class="w-4 h-4 shrink-0 grow-0 mr-2 rounded-full"
        ></div>
        <span
          class="overflow-ellipsis overflow-hidden text-nowrap whitespace-nowrap block"
          >{group.label}</span
        >
        {#if group.count > 1}
          <span class="ml-2 block">+{group.count - 1}</span>
        {/if}
      </button>
    </li>
  {:else}
    <li class="text-center text-gray-500">Еще ничего не выделено</li>
  {/each}
  {#if groups.length > 0}
    <li class="text-center mt-auto mb-0 -ml-6 text-gray-500 text-sm">
      Выделено идей: {groups.length}
    </li>
  {/if}
</ul>
