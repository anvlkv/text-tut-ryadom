<script lang="ts">
  export let text: string;
  export let color: string;
  export let onRemove: () => void;
</script>

<div class="inline-block relative">
  <span style="background-color: {color};" class="rounded-sm">
    {text}
  </span>
  <button style="background-color: {color};" class="rounded-t-sm leading-none absolute h-3 -top-1 px-2 right-0" title="Удалить выделение" on:click={onRemove}>
    x
  </button>
</div>
