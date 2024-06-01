<script lang="ts">
  import type { Task } from "../types/Task";


  export let task: Task
  export let isCurrent: boolean
</script>

<a href="/{task.input.id}/{task.output ? '2' : '1'}" class="h-full w-full p-2 flex flex-col border-b border-solid border-gray-500">
  <h5>#{task.input.id}</h5>

  {#if task.output}
    <p>{task.output.text}</p>
  {/if}

  {#if isCurrent}
    <p>Текущая</p>
  {:else if task.output}
    <p>Выполнена: {task.output.completed_ts}</p>
  {:else if task.postponed}
    <p>Отложенная</p>
  {:else}
    <p>Новая</p>
  {/if}
</a>