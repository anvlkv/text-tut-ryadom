<script lang="ts">
    import String from "$lib/string.svelte";
    import type { AppData } from "$lib/types/AppData";
    import { ToneOfVoice } from "$lib/types/Preferences";
    import { getContext, onMount } from "svelte";
    import type { ChangeEventHandler } from "svelte/elements";
    import type { Writable } from "svelte/store";

    const ctx: Writable<AppData> = getContext("appData");

    $: tone = $ctx.preferences?.tone || ToneOfVoice.Polite;

    const toneChange: ChangeEventHandler<HTMLInputElement> = (e) => {
        console.log(e.currentTarget.value);
        ctx.update((d) => {
            d.preferences = {
                ...d.preferences!,
                tone: ToneOfVoice[
                    e.currentTarget.value as keyof typeof ToneOfVoice
                ],
            };
            return d;
        });
    };

    onMount(() => {
        window.scrollTo({ top: 0, behavior: "smooth" });
    });
</script>

<div class="flex flex-col items-center mb-8">
    <h3 class="text-xl text-center mb-2">Тон</h3>
    <h4 class="text-lg text-center mb-2">
        <String>
            <svelte:fragment slot="familiar">
                Как к тебе обращаться?
            </svelte:fragment>
            <svelte:fragment>Выберите обращение:</svelte:fragment>
        </String>
    </h4>
    <div class="flex gap-4">
        <label
            class="overflow-hidden flex flex-col justify-between pb-2 break-words aspect-square items-center basis-1/2 shrink-0 grow-0 rounded border border-gray-400/50 dc:border-white lc:border-black"
        >
            <p class="flex flex-col grow justify-between text-center">
                <small class="block mt-2 text-sm">Сухо, вежливо, на</small>
                <strong class="block text-5xl mt-auto mb-2 font-black"
                    >Вы</strong
                >
            </p>
            <input
                name="tone"
                type="radio"
                checked={tone === ToneOfVoice.Polite}
                on:change={toneChange}
                value="Polite"
            />
        </label>
        <label
            class="overflow-hidden flex flex-col justify-between pb-2 break-words aspect-square items-center basis-1/2 shrink-0 grow-0 rounded border border-gray-400/50 dc:border-white lc:border-black"
        >
            <p class="flex flex-col grow justify-between text-center">
                <small class="block mt-2 text-sm"
                    >Дружелюбно, развернуто, на</small
                >
                <strong class="block text-5xl mt-auto mb-2 font-black"
                    >Ты</strong
                >
            </p>
            <input
                name="tone"
                type="radio"
                checked={tone === ToneOfVoice.Familiar}
                on:change={toneChange}
                value="Familiar"
            />
        </label>
    </div>
</div>
