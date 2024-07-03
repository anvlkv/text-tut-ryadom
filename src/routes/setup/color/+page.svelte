<script lang="ts">
    import String from "$lib/string.svelte";
    import type { AppData } from "$lib/types/AppData";
    import { ColorSchema } from "$lib/types/Preferences";
    import { getContext, onMount } from "svelte";
    import type { ChangeEventHandler } from "svelte/elements";
    import type { Writable } from "svelte/store";

    const ctx: Writable<AppData> = getContext("appData");

    const {
        preferDark,
        preferHighContrast,
    }: {
        preferDark: Writable<boolean>;
        preferHighContrast: Writable<boolean>;
    } = getContext("color-preferences");

    $: schema = ($ctx.preferences?.colorSchema ||
        ($preferHighContrast
            ? $preferDark
                ? ColorSchema.HighContrastDark
                : ColorSchema.HighContrastLight
            : $preferDark
              ? ColorSchema.BrightDark
              : ColorSchema.BrightLight)) as ColorSchema;

    const schemaChange: ChangeEventHandler<HTMLInputElement> = (e) => {
        ctx.update((d) => {
            d.preferences = {
                ...d.preferences!,
                colorSchema:
                    ColorSchema[
                        e.currentTarget.value as keyof typeof ColorSchema
                    ],
            };
            return d;
        });
    };

    onMount(() => {
        window.scrollTo({ top: 0, behavior: "smooth" });
    });
</script>

<div class="w-full grid grid-cols-12 items-center mb-8">
    <h3 class="text-xl text-center col-span-full">Цвет и свет</h3>
    <h4 class="text-lg text-center col-span-full">
        <String>
            <svelte:fragment slot="familiar">
                Какая схема комфортнее для твоих глаз?
            </svelte:fragment>
            <svelte:fragment>Выберите цветовую схему:</svelte:fragment>
        </String>
    </h4>
    <div
        class="grid grid-cols-12 myo:grid-cols-6 x-myo:grid-cols-3 gap-4 col-span-full"
    >
        <div
            class="grid grid-cols-1 gap-4 col-span-1 items-center justify-center"
        >
            <h5
                class="col-span-full mb-2 border-b border-gray-400/50 dc:border-white lc:border-black text-center"
            >
                Тусклая
            </h5>
            <div class="grid grid-cols-1 col-span-1">
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="dull" src="/Dull.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Нейтральная
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.Dull}
                        value="Dull"
                    />
                </label>
            </div>
        </div>
        <div
            class="grid grid-cols-5 gap-4 col-span-5 items-center justify-center"
        >
            <h5
                class="col-span-full mb-2 border-b border-gray-400/50 dc:border-white lc:border-black text-center"
            >
                Мягкая
            </h5>
            <div class="grid grid-cols-5 gap-4 col-span-5">
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="SoftGreen" src="/Green.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Зеленая
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.SoftGreen}
                        value="SoftGreen"
                    />
                </label>
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="SoftYellow" src="/Yellow.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Желтая
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.SoftYellow}
                        value="SoftYellow"
                    />
                </label>
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="SoftCold" src="/Cold.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Холодная
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.SoftCold}
                        value="SoftCold"
                    />
                </label>
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="SoftWarm" src="/Warm.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Теплая
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.SoftWarm}
                        value="SoftWarm"
                    />
                </label>
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="SoftEarth" src="/Earth.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Земельная
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.SoftEarth}
                        value="SoftEarth"
                    />
                </label>
            </div>
        </div>
        <div
            class="grid grid-cols-3 gap-4 col-span-3 items-center justify-center"
        >
            <h5
                class="col-span-full mb-2 border-b border-gray-400/50 dc:border-white lc:border-black text-center"
            >
                Яркая
            </h5>
            <div class="grid grid-cols-3 gap-4 col-span-3">
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="BrightDark" src="/Dark.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Темная
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.BrightDark}
                        value="BrightDark"
                    />
                </label>
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="BrightLight" src="/Light.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Светлая
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.BrightLight}
                        value="BrightLight"
                    />
                </label>
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="BrightAuto" src="/SysBright.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Системная
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.BrightAuto}
                        value="BrightAuto"
                    />
                </label>
            </div>
        </div>
        <div
            class="grid grid-cols-3 gap-4 col-span-3 items-center justify-center"
        >
            <h5
                class="col-span-full mb-2 border-b border-gray-400/50 dc:border-white lc:border-black text-center"
            >
                Контрастная
            </h5>
            <div class="grid grid-cols-3 gap-4 col-span-3">
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="HighContrastDark" src="/DC.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Темная
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.HighContrastDark}
                        value="HighContrastDark"
                    />
                </label>
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="HighContrastLight" src="/LC.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Светлая
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.HighContrastLight}
                        value="HighContrastLight"
                    />
                </label>
                <label
                    class="overflow-hidden flex flex-col justify-between pb-2 break-words items-center rounded border border-gray-400/50 dc:border-white lc:border-black"
                >
                    <img alt="HighContrastAuto" src="/SysContrast.png" />
                    <p
                        class="overflow-ellipsis text-xs w-full overflow-hidden text-center"
                    >
                        Системная
                    </p>
                    <input
                        name="schema"
                        on:change={schemaChange}
                        type="radio"
                        checked={schema === ColorSchema.HighContrastAuto}
                        value="HighContrastAuto"
                    />
                </label>
            </div>
        </div>
    </div>
</div>
