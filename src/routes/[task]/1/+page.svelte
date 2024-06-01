<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import type { AppData } from "../../../types/AppData";
  
  const ctx: Writable<AppData> = getContext('appData');

  $: task = $ctx.entries!.find(e => e.input.id === $ctx.current_entry);

</script>

<h1>1 {task?.input.text}</h1>