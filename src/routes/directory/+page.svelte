<script lang="ts">
    import Button from "$lib/button.svelte";
    import String from "$lib/string.svelte";
    import type { AppData } from "$lib/types/AppData";
    import { getContext } from "svelte";
    import type { Writable } from "svelte/store";

    const ctx: Writable<AppData> = getContext("appData");
    const loadDir: () => Promise<void> = getContext("loadDir");

    async function reselectDir() {
        ctx.update((d) => {
            d.src_dir = undefined;
            d.current_entry = undefined;
            d.entries = undefined;

            return d;
        });

        await loadDir();
    }
</script>

<section class="flex flex-col items-center justify-center">
    {#if $ctx.loading}
        <h1 class="text-lg mt-16">Загрузка...</h1>
    {:else}
        <p class="max-w-prose mx-auto my-6">
            <String>
                <svelte:fragment slot="familiar">
                    Чтобы начать работу, выбери папку с проектом
                </svelte:fragment>
                <svelte:fragment>
                    Выберите директорию с проектом
                </svelte:fragment>
            </String>
        </p>
        <Button cta={true} on:click={reselectDir}>Выбрать</Button>

        <div class="flex items-center my-2 w-full">
            <hr
                class="border-t-gray-400/50 dc:border-t-white lc:border-t-black grow mx-4"
            />
            <h2 class="text-lg">или</h2>
            <hr
                class="border-t-gray-400/50 dc:border-t-white lc:border-t-black grow mx-4"
            />
        </div>
        <p class="max-w-prose mx-auto my-6">
            <String>
                <svelte:fragment slot="familiar">
                    Импортируй задачи из Label Studio
                </svelte:fragment>
                <svelte:fragment>Импорт из Label Studio</svelte:fragment>
            </String>
        </p>
        <Button href="/directory/label-studio" type="button">Начать</Button>
    {/if}
</section>
