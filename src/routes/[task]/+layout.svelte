<script lang="ts">
    import { page } from "$app/stores";
    import TaskFooter from "$lib/taskFooter.svelte";
    import TaskList from "$lib/taskList.svelte";
    import type { AppData } from "$lib/types/AppData";
    import type { ScrollSync } from "$lib/types/ScrollSync";
    import { getContext, onMount, setContext } from "svelte";
    import type { UIEventHandler } from "svelte/elements";
    import { writable, type Writable } from "svelte/store";

    const ctx: Writable<AppData> = getContext("appData");
    const scrollCtx: Writable<ScrollSync> = setContext(
        "scrollSync",
        writable({ scrollTop: 0, height: 200, allowFixed: true }),
    );
    const storeAppData: () => Promise<void> = getContext("storeAppData");

    $: {
        if ($page.params.task && $ctx.entries) {
            $ctx.current_entry = $page.params.task;
            storeAppData().then(() => {
                console.debug("store on entry change");
            });
        }
    }

    let sectionElement: HTMLElement;
    const resize = new ResizeObserver((e) => {
        const height = e[0].contentRect.height;
        scrollCtx.update((d) => {
            d.height = height;
            return d;
        });
    });

    const onScroll: UIEventHandler<HTMLElement> = (ev) => {
        scrollCtx.update((d) => {
            d.scrollTop = ev.currentTarget.scrollTop;
            return d;
        });
    };

    onMount(() => {
        resize.observe(sectionElement);
        return () => {
            resize.disconnect();
        };
    });
</script>

<div class="flex grow items-stretch justify-stretch w-full overflow-hidden">
    <TaskList />
    <section
        bind:this={sectionElement}
        on:scroll={onScroll}
        class="h-full w-full flex flex-col pt-4 overflow-y-auto overflow-x-visible"
    >
        {#if $ctx.loading}
            <h2 class="m-8 w-full text-center">Загрузка...</h2>
        {:else}
            <slot />
            <TaskFooter />
        {/if}
    </section>
</div>
