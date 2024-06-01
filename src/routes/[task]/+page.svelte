<script lang="ts">
  import { redirect } from "@sveltejs/kit";
  import { onMount } from "svelte";
  import type { Task } from "../../types/Task";

  export let task: Task;

  onMount(() => {
    throw redirect(307, `/${task.input.id}/1`)
  })
</script>