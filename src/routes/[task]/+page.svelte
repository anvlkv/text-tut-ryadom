<script lang="ts">
  import type { Task } from "$lib/types/Task";
  import { redirect } from "@sveltejs/kit";
  import { onMount } from "svelte";

  export let task: Task;

  onMount(() => {
    throw redirect(307, `/${task.input.id}/highlight`)
  })
</script>