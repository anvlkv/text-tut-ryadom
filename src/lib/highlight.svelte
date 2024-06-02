<script lang="ts">
  export let text: string;
  export let color: string;
  export let onRemove: () => void;
</script>

<div class="inline relative">
  <span 
    style="background-color: {color};" 
    class="rounded-sm"
  >
    {text}
  </span>
  <button style="background-color: {color};" class="text-base font-thin rounded-t-sm leading-none absolute h-3 px-2 -ml-7 -mt-1" title="Удалить выделение" on:click={onRemove}>
    ×
  </button>
</div>
