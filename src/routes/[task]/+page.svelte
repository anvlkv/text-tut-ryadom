<script lang="ts">
    import { goto } from "$app/navigation";
    import type { Task } from "$lib/types/Task";
    import { onMount } from "svelte";

    export let task: Task;

    onMount(() => {
        goto(`/${task.input.id}/highlight`);
    });
</script>
