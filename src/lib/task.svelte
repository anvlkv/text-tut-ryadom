<script lang="ts">
  import type { Task } from "./types/Task";

  export let task: Task;
  export let isCurrent: boolean;
  const ONE_DAY = 24*60*60*1000;

  const today = new Date()
  let complete_date: string | undefined;
  $: complete_date = (() => {
    if (task?.output?.completed_ts) {
      const date = new Date(task.output.completed_ts);
      if (date.valueOf() - today.valueOf() > ONE_DAY || date.getDay() != today.getDay()) {
        return date.toLocaleDateString('ru-RU', {year: 'numeric', day: 'numeric', month: 'numeric', hour: 'numeric', minute: 'numeric'})
      }
      else {
        return `Сегодня в ${date.getHours()}:${date.getMinutes()}`
      }
    }
  })();

</script>

<a
  href="/{task.input.id}/{task.output ? 'summarize' : 'highlight'}"
  class="h-full w-full p-2 flex flex-col justify-between border-b border-solid border-gray-400/50 dc:border-white lc:border-black {isCurrent && 'bg-gray-50 dark:bg-gray-900 dc:bg-stone-950 lc:bg-stone-50 yellow:bg-yellow-50 green:bg-green-50 warm:bg-red-50 cold:bg-blue-50 earth:bg-earth-50'}"
>
  <h5>#{task.input.id}</h5>

  {#if task.output}
    <p class="whitespace-nowrap overflow-hidden overflow-ellipsis">
      {task.output.text}
    </p>
  {/if}

  <div class="flex justify-between text-gray-600">
    {#if isCurrent}
      <p class="font-bold">Текущая</p>
    {:else if task.output && complete_date}
      <p>Выполнена: {complete_date}</p>
    {:else if task.postponed}
      <p>Отложенная</p>
    {:else}
      <p>Новая</p>
    {/if}
  </div>
</a>
