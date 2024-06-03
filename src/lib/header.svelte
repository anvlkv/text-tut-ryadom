<script lang="ts">
  import { page } from "$app/stores";
  $: step = $page.route.id?.endsWith("[task]/highlight")
    ? "1/2"
    : $page.route.id?.endsWith("[task]/summarize")
      ? "2/2"
      : undefined;

</script>

<header class="flex w-full border-b border-solid border-gray-100 bg-gray-50 shadow-sm">
  <div class="basis-1/5 py-2 border-r border-solid border-gray-400">
    <h3 class="w-full text-center text-lg">Мои задачи:</h3>
  </div>
  {#if $page.route.id?.endsWith("[task]/summarize")}
    <a href={`/${$page.params.task}/highlight`} class="block py-2 px-4 text-blue-500">
      &langle; Назад
    </a>
  {/if}
  <div class="grow py-2">
    <h3 class="w-full text-center text-lg">
      {#if step}
        Шаг {step}
      {/if}
    </h3>
  </div>
</header>
