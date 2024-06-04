<script lang="ts">
  import { goto } from "$app/navigation";
  import {
    BASE_FONT_SIZE,
    MAX_FONT_SIZE,
    MIN_FONT_SIZE,
  } from "$lib/values";
  import Highlight from "$lib/highlight.svelte";
  import String from "$lib/string.svelte";
  import type { AppData } from "$lib/types/AppData";
  import { ColorSchema, ToneOfVoice } from "$lib/types/Preferences";
  import { getContext, onMount } from "svelte";
  import type { ChangeEventHandler } from "svelte/elements";
  import type { Writable } from "svelte/store";
    import Button from "$lib/button.svelte";

  const ctx: Writable<AppData> = getContext("appData");
  const storeAppData: ()=> Promise<void> = getContext('storeAppData')

  let preferDark = false;
  let preferHighContrast = false;

  function darkListener(ev: MediaQueryListEvent) {
    preferDark = ev.matches;
  }
  function contrastListener(ev: MediaQueryListEvent) {
    preferHighContrast = ev.matches;
  }

  let schema: ColorSchema;

  $: fontSize = $ctx.preferences?.fontSize || BASE_FONT_SIZE;
  $: tone = $ctx.preferences?.tone || ToneOfVoice.Polite;
  $: schema = ($ctx.preferences?.colorSchema ||
    (preferHighContrast
      ? preferDark
        ? ColorSchema.HighContrastDark
        : ColorSchema.HighContrastLight
      : preferDark
        ? ColorSchema.BrightDark
        : ColorSchema.BrightLight)) as ColorSchema;

  function applySetup(e: Event) {
    e.preventDefault();
    storeAppData().then(() => {
      goto("/");
    })
  }

  function resetSetup(e: Event) {
    e.preventDefault();
    ctx.update((d) => {
      d.preferences = {
        fontSize: BASE_FONT_SIZE,
        tone: ToneOfVoice.Polite,
        colorSchema: preferHighContrast
          ? preferDark
            ? ColorSchema.HighContrastDark
            : ColorSchema.HighContrastLight
          : preferDark
            ? ColorSchema.BrightDark
            : ColorSchema.BrightLight,
      };
      return d;
    });
  }

  const toneChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    console.log(e.currentTarget.value);
    ctx.update((d) => {
      d.preferences = {
        ...(d.preferences || {
          fontSize,
          colorSchema: schema,
        }),
        tone: ToneOfVoice[e.currentTarget.value as keyof typeof ToneOfVoice],
      };
      return d;
    });
  };

  const schemaChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    ctx.update((d) => {
      d.preferences = {
        ...(d.preferences || {
          fontSize,
          tone,
        }),
        colorSchema:
          ColorSchema[e.currentTarget.value as keyof typeof ColorSchema],
      };
      return d;
    });
  };

  const fontChange: ChangeEventHandler<HTMLInputElement> = (e) => {
    ctx.update((d) => {
      d.preferences = {
        ...(d.preferences || {
          tone,
          colorSchema: schema,
        }),
        fontSize: e.currentTarget.valueAsNumber,
      };
      return d;
    });
  };

  $: {
    if (!$ctx.preferences) {
      ctx.update((d) => {
        d.preferences = {
          tone,
          fontSize,
          colorSchema: schema,
        };
        return d;
      });
    }
  }

  onMount(() => {
    const preferDarkMq = window.matchMedia("(prefers-color-scheme: dark)");
    const preferHightContrastMq = window.matchMedia("(prefers-contrast: more)");

    preferDarkMq.addEventListener("change", darkListener);

    preferHightContrastMq.addEventListener("change", contrastListener);

    return () => {
      preferDarkMq.removeEventListener("change", darkListener);
      preferHightContrastMq.removeEventListener("change", contrastListener);
    };
  });
</script>

<form
  id="preferences"
  class="flex flex-col items-center p-8 overflow-y-auto overflow-x-hidden"
  on:submit={applySetup}
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
        <p class="flex flex-col text-center">
          <small class="block text-sm">Сухо, вежливо, на</small>
          <strong class="block text-3xl font-bold">Вы</strong>
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
        <p class="flex flex-col text-center">
          <small class="block text-sm">Дружелюбно, развернуто, на</small>
          <strong class="block text-3xl font-bold">Ты</strong>
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
        <h5 class="col-span-full h-6 mb-2 border-b border-gray-400/50 dc:border-white lc:border-black text-center">
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
        <h5 class="col-span-full h-6 mb-2 border-b border-gray-400/50 dc:border-white lc:border-black text-center">
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
        <h5 class="col-span-full h-6 mb-2 border-b border-gray-400/50 dc:border-white lc:border-black text-center">
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
        <h5 class="col-span-full h-6 mb-2 border-b border-gray-400/50 dc:border-white lc:border-black text-center">
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
  <div class="flex flex-col items-center mb-8">
    <h3 class="text-xl text-center">Шрифт</h3>
    <h4 class="text-lg text-center">
      <String>
        <svelte:fragment slot="familiar">
          Какой шрифт тебе удобнее читать?
        </svelte:fragment>
        <svelte:fragment>Выберите размер шрифта:</svelte:fragment>
      </String>
    </h4>
    <label class="overflow-hidden w-full">
      <div class="flex w-full justify-between items-baseline">
        <span class="block" style="font-size: 12px;">Aa</span>
        <span class="block" style="font-size: 27px;">Aa</span>
        <span class="block" style="font-size: 42px;">Aa</span>
      </div>
      <style>
        input[type="range"]::-webkit-slider-thumb,
        input[type="range"]::-moz-range-thumb {
          outline-color: #7c3aed;
          outline-width: 2px;
        }
      </style>
      <input
        class="w-full appearance-none h-2 rounded-full bg-gray-700 dark:bg-gray-300 dc:bg-white lc:bg-black dull:bg-gray-500 warm:bg-red-800 cold:bg-blue-800 green:bg-green-800 yellow:bg-yellow-800 earth:bg-emerald-800"
        name="font-size"
        value={fontSize}
        on:change={fontChange}
        type="range"
        min={MIN_FONT_SIZE}
        max={MAX_FONT_SIZE}
        step="1"
      />
    </label>
    <div class="max-w-prose mx-auto my-4 leading-relaxed">
      <h5 class="mb-2">Пример текста:</h5>
      <p>
        <Highlight text="Древняя печать" color={0} />
        была основана на использовании
        <Highlight
          text="глиняных или деревянных блоков с вырезанными текстами и изображениями"
          color={1}
        />. Эти блоки использовались для печати на различных материалах, таких
        как папирус, пергамент и дерево. В
        <Highlight text="Древнем Египте и Китае" color={2} />
        печать была важной частью культуры и использовалась для создания
        <Highlight text="документов, монет и других материалов." color={3} />
      </p>
      <p>
        В <Highlight text="Средние века" color={4} /> печать была основана на использовании
        <Highlight text="металлических блоков" color={5} /> с вырезанными текстами
        и изображениями. Эти блоки использовались для печати на <Highlight
          text="бумаге и других материалах."
          color={6}
        /> В это время печать была важной частью <Highlight
          text="книжной иллюстрации"
          color={7}
        /> и использовалась для создания иллюстрированных изданий.
      </p>
      <p>
        В <Highlight text="Новое время" color={8} /> печать была основана на использовании
        <Highlight
          text="типографии, которая была изобретена в XV веке."
          color={9}
        /> Типография позволяла массово производить печатные издания, что стало важным
        шагом в развитии печати. В это время печать стала важной частью массовой
        культуры и использовалась для <Highlight
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
  <Button
    on:click={applySetup}
    cta={true}
    type="submit"
    form="preferences"
  >
    Готово
  </Button>
</footer>
