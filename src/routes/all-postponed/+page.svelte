<script lang="ts">
  import { goto } from "$app/navigation";
  import type { AppData } from "$lib/types/AppData";
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";

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

<h1>Все задачи отложены...</h1>
<h2>Продолжение через несколько минут.</h2>
