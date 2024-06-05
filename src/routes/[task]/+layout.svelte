<script lang="ts">
  import { page } from "$app/stores";
  import TaskFooter from "$lib/taskFooter.svelte";
  import TaskList from "$lib/taskList.svelte";
  import type { AppData } from "$lib/types/AppData";
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";

  const ctx: Writable<AppData> = getContext("appData");
  const storeAppData: () => Promise<void> = getContext("storeAppData");

  $: {
    if ($page.params.task && $ctx.entries) {
      $ctx.current_entry = $page.params.task;
      storeAppData().then(() => {
        console.debug("store on entry change");
      });
    }
  }
</script>

<div class="flex grow items-stretch justify-stretch w-full overflow-hidden">
  <TaskList />
  <section class="h-full w-full overflow-hidden flex flex-col pt-4 overflow-y-auto overflow-x-visible">
    <slot />
    <TaskFooter />
  </section>
</div>
