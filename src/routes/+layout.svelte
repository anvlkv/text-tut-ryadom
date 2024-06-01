<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/tauri";
  import { readTextFile, BaseDirectory } from "@tauri-apps/api/fs";
  import { open } from "@tauri-apps/api/dialog";
  import { onMount, setContext } from "svelte";
  import Header from "../components/header.svelte";
  import List from "../components/list.svelte";
  import type { AppData } from "../types/AppData";
  import type { Task } from "../types/Task";
  import { goto } from "$app/navigation";
  import { writable } from "svelte/store";

  const data = setContext("appData", writable({} as AppData));

  export const WINDOW_SIZE = 100;

  async function loadDir() {
    console.info("loading dir");

    let dir;

    if (!$data.src_dir) {
      dir = (await open({
        title: "Выбери папку с проектом",
        directory: true,
      })!) as string;
    } else {
      dir = $data.src_dir;
    }

    $data.src_dir = dir;

    const tasks: Task[] = await invoke("read_dir_tasks", { dir });

    $data.entries = tasks;
    $data.total_entries = tasks.length;

    const entryIndex = $data.current_entry
      ? $data.entries.findIndex((e) => e.input.id === $data.current_entry)
      : $data.entries.findIndex((e) => !e.output);
    $data.current_entry = $data.entries[entryIndex]?.input.id;
    const windowStart = Math.max(entryIndex - WINDOW_SIZE / 2, 0);
    $data.entries_window = [windowStart, windowStart + WINDOW_SIZE];
  }

  async function load() {
    console.info("loading");

    await invoke("close_splashscreen");

    try {
      const { src_dir, current_entry } = JSON.parse(
        await readTextFile("app_data.json", { dir: BaseDirectory.AppData })
      );
      $data.current_entry = current_entry;
      $data.src_dir = src_dir;
    } catch {
      $data.src_dir = undefined;
      $data.current_entry = undefined;
      $data.total_entries = undefined;
      $data.entries_window = undefined;
    }

    await loadDir();

    const entry_id = $data.current_entry! || $data.entries![0].input.id;
    const entry_step = $data.entries!.find((e) => e.input.id === entry_id)
      ?.output
      ? "2"
      : "1";

    await goto(`/${entry_id}/${entry_step}`, { replaceState: true });

    console.info("loaded");
  }

  onMount(load);
</script>

<main
  class="font-body select-none flex flex-col h-screen w-screen items-stretch justify-stretch"
>
  {#if !$data.total_entries}
    <p class="max-w-prose mx-auto my-6">
      Чтобы начать работу, выбери папку с проектом
    </p>
    <button
      class="rounded-full mx-auto w-auto px-6 py-2 text-lg bg-cyan-700 text-gray-50"
      on:click={loadDir}
    >
      Выбрать
    </button>
  {:else}
    <Header>
      <slot name="header" />
    </Header>
    <div class="flex items-stretch justify-stretch">
      <List />
      <slot />
    </div>
  {/if}
</main>
