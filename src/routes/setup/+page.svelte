<script lang="ts">
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";

    onMount(() => {
        if ($page.route.id === "/setup") {
            goto(`/setup/tone`);
        }
    });
</script>
