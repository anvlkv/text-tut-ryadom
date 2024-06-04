<script lang="ts">
  import String from "$lib/string.svelte";
</script>

<h1 class="my-6 mx-auto text-3xl">
  <String>
    <svelte:fragment slot="familiar">
      Все задачи выполнены!
    </svelte:fragment>
    <svelte:fragment>
      Нет новых задач
    </svelte:fragment>
  </String>
</h1>