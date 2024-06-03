<script lang="ts">
  import { getContext } from "svelte";

  const loadDir: () => Promise<void> = getContext('loadDir');
</script>

<section>
  <p class="max-w-prose mx-auto my-6">
    Чтобы начать работу, выбери папку с проектом
  </p>
  <button
    class="rounded-full mx-auto w-auto px-6 py-2 text-lg bg-cyan-700 text-gray-50"
    on:click={loadDir}
  >
    Выбрать
  </button>
</section>