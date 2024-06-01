<script lang="ts">
  import { getContext } from 'svelte';
  import type { AppData } from '../../types/AppData.js';
  import type { Writable } from 'svelte/store';

  export let data;

  const ctx: Writable<AppData> = getContext('appData');

  $: $ctx.current_entry = data.task;
</script>


<section>
  <slot/>
  <footer></footer>
</section>