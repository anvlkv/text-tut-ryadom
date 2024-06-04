<script lang="ts">
  import type { AppData } from "$lib/types/AppData";
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import String from "$lib/string.svelte";
    import Button from "$lib/button.svelte";


  const ctx: Writable<AppData> = getContext('appData');
  const loadDir: () => Promise<void> = getContext('loadDir');

  async function reselectDir() {
    ctx.update(d => {
      d.src_dir = undefined;
      d.current_entry = undefined;
      d.entries_window = undefined;
      d.entries = undefined;

      return d;
    });

    await loadDir();
  }
</script>

<section class="flex flex-col items-center justify-center">
  {#if $ctx.loading}
    <h1 class="text-lg mt-16">Загрузка...</h1>
  {:else}
    <p class="max-w-prose mx-auto my-6">
      <String>
        <svelte:fragment slot="familiar">
          Чтобы начать работу, выбери папку с проектом
        </svelte:fragment>
        <svelte:fragment>
          Выберите директорию с проектом
        </svelte:fragment>
      </String>
    </p>
    <Button
      cta={true}
      on:click={reselectDir}
    >
      Выбрать
    </Button>
  {/if}
</section>