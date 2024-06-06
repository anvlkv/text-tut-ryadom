<script lang="ts">
  import { page } from "$app/stores";
  import { getContext, onMount } from "svelte";
  import InfiniteLoading from "svelte-infinite-loading";
  import VirtualList from "svelte-tiny-virtual-list";
  import type { Writable } from "svelte/store";
  import Task from "./task.svelte";
  import type { AppData } from "./types/AppData";

  const ctx: Writable<AppData> = getContext("appData");
  const taskHeight = 5;

  let asideElement: HTMLElement;
  let asideHeight = 200;
  const resize = new ResizeObserver((e) => {
    asideHeight = e[0].contentRect.height;
  });

  let allowScrollTo = false;
  $: taskIndex = allowScrollTo ? $ctx.entries?.findIndex(
    (e) => e.input.id === $page.params["task"],
  ) : undefined;

  function infiniteHandler({
    detail: { complete, error },
  }: CustomEvent<{ complete: () => void; error: () => void }>) {}

  function onItemsUpdated() {
    allowScrollTo = true;
  }

  onMount(() => {
    resize.observe(asideElement);
    return () => {
      resize.disconnect();
    };
  });
</script>

<aside
  bind:this={asideElement}
  class="basis-1/5 shrink-0 grow-0 text-sm border-r border-solid border-gray-400/50 dc:border-white lc:border-black overflow-x-hidden h-full"
>
  <ul>
    {#each ($ctx.entries || []) as entry}
      <li>
          <Task
          task={entry}
          isCurrent={$ctx.current_entry === entry.input.id}
        />
      </li>
    {/each}
  </ul>
  <!-- <VirtualList
    height={asideHeight}
    itemCount={$ctx.total_entries || 0}
    itemSize={taskHeight * ($ctx.preferences?.fontSize || 16)}
    on:itemsUpdated={onItemsUpdated}
    scrollToIndex={taskIndex}
    scrollToBehaviour="auto"
    scrollToAlignment="center"
  >
    <div slot="item" let:index let:style {style} class="task-entry">
      <Task
        task={($ctx.entries || [])[index]}
        isCurrent={$ctx.current_entry === ($ctx.entries || [])[index].input.id}
      />
    </div>
    <div slot="footer">
      <InfiniteLoading on:infinite={infiniteHandler} />
    </div>
  </VirtualList> -->
</aside>
