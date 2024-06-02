<script lang="ts">
  import { getContext } from "svelte";
  import type { AppData } from "$lib/types/AppData";
  import type { Writable } from "svelte/store";
  import TaskFooter from "$lib/taskFooter.svelte";

  export let data;

  const ctx: Writable<AppData> = getContext("appData");
  const storeAppData: () => Promise<void> = getContext("storeAppData");

  $: {
    $ctx.current_entry = data.task;
    storeAppData().then(() => {
      console.debug("store on entry change")
    });
  }
</script>

<section class="h-full w-full overflow-hidden flex flex-col pt-4">
  <slot />
  <TaskFooter />
</section>
