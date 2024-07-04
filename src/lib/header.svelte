<script lang="ts">
    import { page } from "$app/stores";
    import { getContext } from "svelte";
    import { portal } from "svelte-portal";
    import type { Writable } from "svelte/store";
    import String from "./string.svelte";
    import type { AppData } from "./types/AppData";

    const ctx: Writable<AppData> = getContext("appData");

    $: step = $page.route.id?.endsWith("[task]/highlight")
        ? "1/2"
        : $page.route.id?.endsWith("[task]/summarize")
          ? "2/2"
          : undefined;

    let menuVisible = false;
    let pos = [0, 0];
    let menuButton: HTMLButtonElement;

    $: dir = $ctx.src_dir || $ctx.label_studio_src?.label_studio_url;

    function showMenu(e: MouseEvent) {
        e.stopPropagation();
        menuVisible = true;
        pos = [
            menuButton.offsetTop + menuButton.offsetHeight,
            menuButton.offsetParent!.clientWidth - menuButton.offsetLeft,
        ];
    }

    function clickHandler() {
        if (menuVisible) {
            menuVisible = false;
        }
    }
</script>

<header
    class="flex items-center w-full bg-gray-50 dark:bg-gray-900 dc:bg-stone-950 lc:bg-stone-50 yellow:bg-yellow-50 green:bg-green-50 warm:bg-red-50 cold:bg-blue-50 earth:bg-earth-50 border-gray-400/50 dc:border-white lc:border-black shadow-sm"
>
    {#if $page.params["task"]}
        <div
            class="basis-1/5 py-2 border-r border-solid border-gray-400/50 dc:border-white lc:border-black"
        >
            <h3 class="w-full text-center text-lg">
                <String>
                    <svelte:fragment slot="familiar"
                        >Мои задачи:</svelte:fragment
                    >
                    <svelte:fragment>Задачи:</svelte:fragment>
                </String>
            </h3>
        </div>
    {/if}
    {#if $page.route.id?.endsWith("[task]/summarize")}
        <a
            href={`/${$page.params.task}/highlight`}
            class="block py-2 px-4 text-blue-500 yellow:text-yellow-500
      warm:text-orange-500
      green:text-emerald-500
      cold:text-sky-500
      earth:text-lime-600
      dc:text-violet-400
      lc:text-violet-800"
        >
            <String>
                <svelte:fragment slot="familiar">
                    &langle; Вернуться к выделению фрагментов
                </svelte:fragment>
                <svelte:fragment>&langle; Назад</svelte:fragment>
            </String>
        </a>
    {/if}
    <div class="grow py-2">
        <h3 class="w-full text-center text-lg">
            {#if step}
                Шаг {step}
            {/if}
        </h3>
    </div>
    <button
        bind:this={menuButton}
        title="Меню"
        class="py-2 px-4 text-2xl"
        on:click={showMenu}
    >
        ≡
    </button>
</header>

{#if menuVisible}
    <div
        class="fixed"
        style="top: {pos[0]}px; right: {pos[1]}px;"
        use:portal={"main"}
        hidden
    >
        <ul
            class="bg-gray-50 dark:bg-gray-900 dc:bg-stone-950 lc:bg-stone-50 yellow:bg-yellow-50 green:bg-green-50 warm:bg-red-50 cold:bg-blue-50 earth:bg-earth-50 border-gray-400/50 dc:border-white lc:border-black shadow-lg rounded overflow-hidden"
        >
            <li
                class="border-b border-gray-400/50 dc:border-white lc:border-black"
            >
                <a
                    class="p-4 w-full flex flex-col hover:text-gray-50 hover:dc:text-black hover:dark:text-gray-950 hover:bg-violet-600 hover:dark:bg-violet-300 hover:dc:bg-violet-50 hover:lc:bg-violet-900 hover:dull:bg-slate-900 hover:earth:bg-emerald-800 hover:warm:bg-amber-600 hover:cold:bg-blue-600 hover:yellow:bg-orange-600 hover:green:bg-lime-600"
                    href="/directory"
                >
                    <span> Выбор проекта </span>
                    <small class="text-xs text-thin">{dir}</small>
                </a>
            </li>
            <li>
                <a
                    class="p-4 w-full block hover:text-gray-50 hover:dc:text-black hover:dark:text-gray-950 hover:bg-violet-600 hover:dark:bg-violet-300 hover:dc:bg-violet-50 hover:lc:bg-violet-900 hover:dull:bg-slate-900 hover:earth:bg-emerald-800 hover:warm:bg-amber-600 hover:cold:bg-blue-600 hover:yellow:bg-orange-600 hover:green:bg-lime-600"
                    href="/setup/tone">Настройки</a
                >
            </li>
        </ul>
    </div>
{/if}

<svelte:document on:click={clickHandler} />
