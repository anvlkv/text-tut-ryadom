<script lang="ts">
    import { page } from "$app/stores";
    import { getContext, onMount } from "svelte";

    const prerequisites: () => Promise<void> = getContext("prerequisites")!;

    onMount(() => {
        if ($page.route.id === "/") {
            prerequisites().then(() => {
                console.log("satisfy prerequisites");
            });
        }
    });
</script>
