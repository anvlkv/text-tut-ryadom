<script lang="ts">
    import Button from "$lib/button.svelte";
    import String from "$lib/string.svelte";
    import Highlight from "$lib/highlight.svelte";
    import type { AppData } from "$lib/types/AppData";
    import { ColorSchema, ToneOfVoice } from "$lib/types/Preferences";
    import { BASE_FONT_SIZE } from "$lib/values";
    import { page } from "$app/stores";
    import { getContext, onMount, setContext } from "svelte";
    import { writable, type Writable } from "svelte/store";
    import { goto } from "$app/navigation";

    const ctx: Writable<AppData> = getContext("appData");
    const storeAppData: () => Promise<void> = getContext("storeAppData");

    const preferDark = writable(false);
    const preferHighContrast = writable(false);

    setContext("color-preferences", {
        preferDark,
        preferHighContrast,
    });

    function darkListener(ev: MediaQueryListEvent) {
        preferDark.set(ev.matches);
    }
    function contrastListener(ev: MediaQueryListEvent) {
        preferHighContrast.set(ev.matches);
    }

    function applySetup(e: Event) {
        e.preventDefault();
        storeAppData().then(() => {
            console.log("settings done");
            window.location.href = "/";
        });
    }

    async function continueSetup(e: Event) {
        e.preventDefault();
        if ($page.route.id === "/setup/tone") {
            await goto("/setup/font");
        } else if ($page.route.id === "/setup/font") {
            await goto("/setup/color");
        } else {
            applySetup(e);
        }
    }

    async function resetSetup(e: Event) {
        e.preventDefault();
        ctx.update((d) => {
            d.preferences = {
                fontSize: BASE_FONT_SIZE,
                tone: ToneOfVoice.Polite,
                colorSchema: ColorSchema.BrightAuto,
            };
            return d;
        });
        if ($page.route.id !== "/setup/tone") {
            await goto("/setup/tone");
        } else {
            await goto("/");
        }
    }

    onMount(() => {
        const preferDarkMq = window.matchMedia("(prefers-color-scheme: dark)");
        const preferHightContrastMq = window.matchMedia(
            "(prefers-contrast: more)",
        );

        preferDarkMq.addEventListener("change", darkListener);

        preferHightContrastMq.addEventListener("change", contrastListener);

        if (!$ctx.preferences) {
            ctx.update((d) => {
                d.preferences = {
                    fontSize: BASE_FONT_SIZE,
                    tone: ToneOfVoice.Polite,
                    colorSchema: ColorSchema.BrightAuto,
                };
                return d;
            });
        }

        return () => {
            preferDarkMq.removeEventListener("change", darkListener);
            preferHightContrastMq.removeEventListener(
                "change",
                contrastListener,
            );
        };
    });
</script>

<form
    id="preferences"
    class="flex flex-col items-center p-8 overflow-y-auto overflow-x-hidden"
    on:submit={continueSetup}
    on:reset={resetSetup}
>
    <h2 class="text-2xl mb-8">
        <String>
            <svelte:fragment slot="familiar">
                Настрой это рабочее место
            </svelte:fragment>
            <svelte:fragment>Настройки рабочего места</svelte:fragment>
        </String>
    </h2>

    <slot />

    <div class="max-w-prose mx-auto my-4 leading-relaxed">
        <h5 class="mb-2">Пример текста:</h5>
        <p>
            <Highlight text="Древняя печать" color={0} />
            была основана на использовании
            <Highlight
                text="глиняных или деревянных блоков с вырезанными текстами и изображениями"
                color={1}
            />. Эти блоки использовались для печати на различных материалах,
            таких как папирус, пергамент и дерево. В
            <Highlight text="Древнем Египте и Китае" color={2} />
            печать была важной частью культуры и использовалась для создания
            <Highlight
                text="документов, монет и других материалов."
                color={3}
            />
        </p>
        <p>
            В <Highlight text="Средние века" color={4} /> печать была основана на
            использовании
            <Highlight text="металлических блоков" color={5} /> с вырезанными текстами
            и изображениями. Эти блоки использовались для печати на
            <Highlight text="бумаге и других материалах." color={6} /> В это время
            печать была важной частью <Highlight
                text="книжной иллюстрации"
                color={7}
            /> и использовалась для создания иллюстрированных изданий.
        </p>
        <p>
            В <Highlight text="Новое время" color={8} /> печать была основана на
            использовании
            <Highlight
                text="типографии, которая была изобретена в XV веке."
                color={9}
            /> Типография позволяла массово производить печатные издания, что стало
            важным шагом в развитии печати. В это время печать стала важной частью
            массовой культуры и использовалась для <Highlight
                text="создания газет, журналов"
                color={10}
            /> и других печатных изданий.
        </p>
        <p>
            <Highlight text="Современная печать" color={11} /> основана на использовании
            <Highlight
                text="электронных технологий и компьютерных программ."
                color={0}
            /> Эти технологии позволяют производить печатные издания с <Highlight
                text="высокой скоростью и качеством."
                color={1}
            /> В это время <Highlight
                text="печать стала важной частью интернета и использовалась для создания электронных изданий и онлайн-материалов."
                color={2}
            />
        </p>
    </div>
</form>

<footer
    class="w-full sticky bottom-0 flex justify-end py-3 px-4 bg-gray-50 dark:bg-gray-900 dc:bg-stone-950 lc:bg-stone-50 yellow:bg-yellow-50 green:bg-green-50 warm:bg-red-50 cold:bg-blue-50 earth:bg-earth-50 shadow-sm"
>
    <Button
        on:click={resetSetup}
        __class="mx-4"
        type="reset"
        form="preferences"
    >
        Сбросить
    </Button>

    {#if $page.route.id !== "/setup/color"}
        <Button
            on:click={continueSetup}
            cta={true}
            type="submit"
            form="preferences"
        >
            Далеее
        </Button>
    {:else}
        <Button
            on:click={applySetup}
            cta={true}
            type="submit"
            form="preferences"
        >
            Готово
        </Button>
    {/if}
</footer>
