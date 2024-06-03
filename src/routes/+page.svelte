<script lang="ts">
  import { getContext, onMount } from "svelte";

  const prerequisites: () => Promise<void> = getContext('prerequisites');

  onMount(prerequisites);
</script>