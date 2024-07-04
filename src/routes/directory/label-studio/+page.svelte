<script lang="ts">
    import String from "$lib/string.svelte";
    import Button from "$lib/button.svelte";
    import type { AppData } from "$lib/types/AppData";
    import { getContext } from "svelte";
    import type { ChangeEventHandler } from "svelte/elements";
    import type { Writable } from "svelte/store";
    import { labelStudioGet, labelStudioPost } from "$lib/labelStudio";
    import Input from "$lib/input.svelte";

    const ctx: Writable<AppData> = getContext("appData");
    const prerequisites: () => Promise<any | undefined> =
        getContext("prerequisites");

    let url: string | undefined = undefined;
    let token: string | undefined = undefined;
    let availableProjects: any;
    let project: string | undefined = undefined;
    let queueStart: number | undefined = undefined;
    let queueEnd: number | undefined = undefined;
    let creatingView = false;

    async function onLoadProjects({
        detail: { originalEvent },
    }: CustomEvent<{ originalEvent: any }>) {
        originalEvent.preventDefault();

        availableProjects = await labelStudioGet("projects", {
            label_studio_url: url!,
            token: token!,
        });

        if (availableProjects.results.length) {
            project = availableProjects.results[0].id;
        }
    }
    function resetLabelStudioForm() {
        url = undefined;
        token = undefined;
    }
    async function onConnectLabelStudio() {
        creatingView = true;
        const { id: view } = await labelStudioPost<{ id: number }>(
            "dm/views/",
            JSON.stringify({
                project,
                data: {
                    filters: {
                        conjunction: "and",
                        items: [
                            {
                                filter: "filter:tasks:id",
                                operator: "in",
                                type: "Number",
                                value: { min: queueStart!, max: queueEnd! },
                            },
                        ],
                    },
                    ordering: ["id"],
                },
            }),
            {
                label_studio_url: url!,
                token: token!,
            },
        );

        // TODO: use https://v2.tauri.app/reference/javascript/stronghold/#client
        ctx.update((d) => {
            d.src_dir = undefined;
            d.label_studio_src = {
                label_studio_url: url!,
                token: token!,
                project: project!,
                view,
                page_size: queueEnd! - queueStart!,
            };
            d.current_entry = undefined;
            d.entries = undefined;
            d.loading = true;

            return d;
        });

        creatingView = false;

        await prerequisites();
    }

    const onChangeUrl = ({ detail: val }: CustomEvent<string>) => {
        url = val;
    };
    const onChangeToken = ({ detail: val }: CustomEvent<string>) => {
        token = val;
    };
    const onChangeStart = ({ detail: val }: CustomEvent<string>) => {
        queueStart = parseInt(val);
    };
    const onChangeEnd = ({ detail: val }: CustomEvent<string>) => {
        queueEnd = parseInt(val);
    };
    const onChangeProject: ChangeEventHandler<HTMLSelectElement> = (e) => {
        const val = e.currentTarget.value;
        project = val;
    };
</script>

<section
    class="flex flex-col items-center justify-center h-full w-full overflow-auto"
>
    <h2 class="text-lg mb-4">
        <String>
            <svelte:fragment slot="familiar">
                Для импорта добавь URL и Access Token из настроек аккаунта Label
                Studio
            </svelte:fragment>
            <svelte:fragment>Импорт задач из Label Studio</svelte:fragment>
        </String>
    </h2>

    {#if creatingView}
        <h2 class="m-8 w-full text-center">Подключение...</h2>
    {:else}
        <form
            class="flex flex-col gap-4"
            on:reset={resetLabelStudioForm}
            on:submit={onConnectLabelStudio}
        >
            <fieldset class="flex flex-col gap-4 mb-4">
                <Input
                    title="URL"
                    type="url"
                    placeholder="https://..."
                    name="label-studio-url"
                    on:change={onChangeUrl}
                    on:keyup={onChangeUrl}
                />
                <Input
                    title="Access Token"
                    type="password"
                    placeholder="Access Token"
                    name="label-studio-token"
                    on:change={onChangeToken}
                    on:keyup={onChangeToken}
                />
                <Button on:click={onLoadProjects} cta={true}>Подключить</Button>
            </fieldset>
            {#if availableProjects}
                <fieldset>
                    <h3 class="text-lg my-4">
                        <String>
                            <svelte:fragment slot="familiar">
                                Выбери проект, с которым хочешь работать
                            </svelte:fragment>
                            <svelte:fragment>Выберите проект</svelte:fragment>
                        </String>
                    </h3>
                    <select
                        value={project || ""}
                        on:change={onChangeProject}
                        name="label-studio-project"
                        placeholder="Выбрать проект из списка"
                        class="w-full my-4 p-2 border-gray-400/50 dc:border-white lc:border-black focus:yellow:border-yellow-500 focus:warm:border-orange-500 focus:green:border-emerald-500 focus:cold:border-sky-500 focus:earth:border-lime-600 focus:dc:border-violet-400 focus:lc:border-violet-800 bg-gray-50 text-gray-900 dull:bg-gray-100 dull:text-gray-800 dc:bg-black dc:text-white lc:bg-white lc:text-black dark:bg-gray-900 dark:text-gray-50 earth:bg-earth-50 earth:text-emerald-900 yellow:bg-yellow-50 yellow:text-yellow-900 green:bg-green-50 green:text-green-900 warm:bg-orange-50 warm:text-rose-900 cold:bg-sky-50 cold:text-blue-900"
                    >
                        {#each availableProjects.results as project}
                            <option
                                selected={project.id === project}
                                value={project.id}>{project.title}</option
                            >
                        {/each}
                    </select>
                    <div class="flex gap-4">
                        <Input
                            title="Начало очереди задач"
                            type="number"
                            placeholder="Первая задача"
                            name="label-studio-start"
                            on:change={onChangeStart}
                            on:keyup={onChangeStart}
                            min="1"
                            max={queueEnd ? `${queueEnd}` : undefined}
                        />
                        <Input
                            title="Конец очереди задач"
                            type="number"
                            placeholder="Последняя задача"
                            name="label-studio-end"
                            min={queueStart ? `${queueStart}` : undefined}
                            on:change={onChangeEnd}
                            on:keyup={onChangeEnd}
                        />
                    </div>
                </fieldset>
            {/if}
            <div class="flex gap-4 w-full justify-end">
                <Button type="reset" on:click={resetLabelStudioForm}
                    >Отмена</Button
                >
                <Button
                    type="submit"
                    cta={true}
                    disabled={!project || !queueEnd || !queueStart}
                    on:click={onConnectLabelStudio}
                >
                    Импортировать
                </Button>
            </div>
        </form>
    {/if}
</section>
