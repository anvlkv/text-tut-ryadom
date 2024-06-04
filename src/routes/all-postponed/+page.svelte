<script lang="ts">
  import { goto } from "$app/navigation";
  import type { AppData } from "$lib/types/AppData";
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import String from "$lib/string.svelte";

  const ctx: Writable<AppData> = getContext("appData");

  $: {
    const entry = $ctx.entries?.find((e) => !e.output);
    if (entry) {
      goto(`/${entry.input.id}/highlight`).then(() => {
        console.info('load postponed task');
      });
    }
  }
</script>


<h1 class="my-6 mx-auto text-3xl">
  <String>
    <svelte:fragment slot="familiar">
      Все оставшиеся задачи отложены... Продолжи через несколько минут.
    </svelte:fragment>
    <svelte:fragment>
      Отложенные задачи будут доступны через несколько минут.
    </svelte:fragment>
  </String>
</h1>
