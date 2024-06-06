<script lang="ts">
  import Button from "$lib/button.svelte";
  import String from "$lib/string.svelte";
  import type { AppData } from "$lib/types/AppData";
  import { getContext } from "svelte";
  import type { ChangeEventHandler } from "svelte/elements";
  import type { Writable } from "svelte/store";

  const ctx: Writable<AppData> = getContext("appData");
  const loadDir: () => Promise<void> = getContext("loadDir");
  const loadLabelStudio: () => Promise<any | undefined> =
    getContext("loadLabelStudio");

  async function reselectLocalDir() {
    ctx.update((d) => {
      d.src = undefined;
      d.current_entry = undefined;
      d.entries_window = undefined;
      d.entries = undefined;

      return d;
    });

    await loadDir();
  }

  let url: string | undefined = undefined;
  let token: string | undefined = undefined;
  let connectLabelStudio = false;
  let availableProjects: any;
  let project: string | undefined = undefined;
  function resetLabelStudioForm() {
    connectLabelStudio = false;
    url = undefined;
    token = undefined;
  }
  async function onConnectLabelStudio() {
    // TODO: use https://v2.tauri.app/reference/javascript/stronghold/#client
    ctx.update((d) => {
      d.src = {
        label_studio_url: url!,
        token: token!,
        project,
      };
      d.current_entry = undefined;
      d.entries_window = undefined;
      d.entries = undefined;

      return d;
    });

    availableProjects = await loadLabelStudio();
  }

  const onChangeUrl: ChangeEventHandler<HTMLInputElement> = (e) => {
    const val = e.currentTarget.value;
    url = val;
  };
  const onChangeToken: ChangeEventHandler<HTMLInputElement> = (e) => {
    const val = e.currentTarget.value;
    token = val;
  };
  const onChangeProject: ChangeEventHandler<HTMLSelectElement> = (e) => {
    const val = e.currentTarget.value;
    project = val;
  };
</script>

<section class="flex flex-col items-center justify-center">
  {#if $ctx.loading}
    <h1 class="text-lg mt-16">Загрузка...</h1>
  {:else if connectLabelStudio}
    <h2 class="text-lg mb-4">
      <String>
        <svelte:fragment slot="familiar">
          Добавь URL и Access Token из настроек аккаунта Label Studio
        </svelte:fragment>
        <svelte:fragment>Подключение Label Studio</svelte:fragment>
      </String>
    </h2>
    <form on:reset={resetLabelStudioForm} on:submit={onConnectLabelStudio}>
      <div class="flex gap-4 mb-4">
        <label class="w-80">
          <p class="mb-2">URL</p>
          <input
            on:change={onChangeUrl}
            on:keyup={onChangeUrl}
            class="w-full p-2 rounded border-2 outline-none
        border-gray-400/50 dc:border-white lc:border-black focus:yellow:border-yellow-500 focus:warm:border-orange-500 focus:green:border-emerald-500 focus:cold:border-sky-500 focus:earth:border-lime-600 focus:dc:border-violet-400 focus:lc:border-violet-800 bg-gray-50 text-gray-900 dull:bg-gray-100 dull:text-gray-800 dc:bg-black dc:text-white lc:bg-white lc:text-black dark:bg-gray-900 dark:text-gray-50 earth:bg-earth-50 earth:text-emerald-900 yellow:bg-yellow-50 yellow:text-yellow-900 green:bg-green-50 green:text-green-900 warm:bg-orange-50 warm:text-rose-900 cold:bg-sky-50 cold:text-blue-900"
            type="url"
            placeholder="https://..."
            name="label-studio-url"
          />
        </label>
        <label class="w-80">
          <p class="mb-2">Access Token</p>
          <input
            on:change={onChangeToken}
            on:keyup={onChangeToken}
            class="w-full p-2 rounded border-2 outline-none
        border-gray-400/50 dc:border-white lc:border-black focus:yellow:border-yellow-500 focus:warm:border-orange-500 focus:green:border-emerald-500 focus:cold:border-sky-500 focus:earth:border-lime-600 focus:dc:border-violet-400 focus:lc:border-violet-800 bg-gray-50 text-gray-900 dull:bg-gray-100 dull:text-gray-800 dc:bg-black dc:text-white lc:bg-white lc:text-black dark:bg-gray-900 dark:text-gray-50 earth:bg-earth-50 earth:text-emerald-900 yellow:bg-yellow-50 yellow:text-yellow-900 green:bg-green-50 green:text-green-900 warm:bg-orange-50 warm:text-rose-900 cold:bg-sky-50 cold:text-blue-900"
            type="password"
            placeholder="Access Token"
            name="label-studio-token"
          />
        </label>
      </div>
      {#if availableProjects}
        <h3 class="text-lg my-4">
          <String>
            <svelte:fragment slot="familiar">
              Выбери проект, с которым хочешь работать
            </svelte:fragment>
            <svelte:fragment>Выберите проект</svelte:fragment>
          </String>
        </h3>
        <select
          value={project}
          on:change={onChangeProject}
          name="label-studio-project"
          placeholder="Выбрать проект из списка"
          class="w-full my-4 p-2 border-gray-400/50 dc:border-white lc:border-black focus:yellow:border-yellow-500 focus:warm:border-orange-500 focus:green:border-emerald-500 focus:cold:border-sky-500 focus:earth:border-lime-600 focus:dc:border-violet-400 focus:lc:border-violet-800 bg-gray-50 text-gray-900 dull:bg-gray-100 dull:text-gray-800 dc:bg-black dc:text-white lc:bg-white lc:text-black dark:bg-gray-900 dark:text-gray-50 earth:bg-earth-50 earth:text-emerald-900 yellow:bg-yellow-50 yellow:text-yellow-900 green:bg-green-50 green:text-green-900 warm:bg-orange-50 warm:text-rose-900 cold:bg-sky-50 cold:text-blue-900"
        >
          {#each availableProjects.results as project}
            <option selected={project.id === project} value={project.id}
              >{project.title}</option
            >
          {/each}
        </select>
      {/if}
      <div class="flex gap-4 w-full justify-end">
        <Button type="reset" on:click={resetLabelStudioForm}>Отмена</Button>
        <Button
          type="submit"
          cta={true}
          disabled={!url || !token}
          on:click={onConnectLabelStudio}
        >
          Подключить
        </Button>
      </div>
    </form>
  {:else}
    <p class="max-w-prose mx-auto my-6">
      <String>
        <svelte:fragment slot="familiar">
          Чтобы начать работу, выбери папку с проектом
        </svelte:fragment>
        <svelte:fragment>Выберите директорию с проектом</svelte:fragment>
      </String>
    </p>
    <Button cta={true} on:click={reselectLocalDir}>Выбрать</Button>
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
          Подключись к Label Studio
        </svelte:fragment>
        <svelte:fragment>Подключитесь к Label Studio</svelte:fragment>
      </String>
    </p>
    <Button
      on:click={() => {
        connectLabelStudio = true;
      }}
    >
      Подключить
    </Button>
  {/if}
</section>
