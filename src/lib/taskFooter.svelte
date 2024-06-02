<script lang="ts">
  import { page } from "$app/stores";
  import type { Writable } from "svelte/store";
  import type { AppData } from "$lib/types/AppData";
  import { getContext } from "svelte";
  import { invoke } from "@tauri-apps/api";
  import { goto } from "$app/navigation";
  import { type Task } from "$lib/types/Task";

  const ctx: Writable<AppData> = getContext("appData");
  const POSTPONE_DURATION = 5 * 60 * 1000;

  $: task = $ctx.entries!.find((e) => e.input.id === $ctx.current_entry);

  $: nextBtn = $page.route.id?.endsWith("[task]/highlight")
    ? "Далее"
    : $page.route.id?.endsWith("[task]/summarize")
      ? "Готово"
      : undefined;

  $: nextBtnEnabled = $page.route.id?.endsWith("[task]/highlight")
    ? (task?.highlights.length || 0) > 0
    : $page.route.id?.endsWith("[task]/summarize")
      ? (task?.output?.text.length || 0) > 0
      : false;

  const storeAppData: () => Promise<void> = getContext("storeAppData");

  async function writeTask(completed?: Date) {
    const saveTask: Task = !completed
      ? task!
      : {
          ...task!,
          output: {
            ...task!.output!,
            completed_ts: completed.toUTCString(),
          },
        };
    console.log(saveTask);
    await invoke("write_task", { task: saveTask });
  }

  async function next() {
    if ($page.route.id?.endsWith("[task]/highlight")) {
      await writeTask();
      await goto(`/${$page.params.task}/summarize`);
    } else {
      await writeTask(new Date());
      const taskIndex = $ctx.entries!.findIndex(
        (e) => e.input.id === $ctx.current_entry
      );
      const nextTask = $ctx.entries![taskIndex + 1];
      if (nextTask) {
        await goto(`/${nextTask.input.id}/highlight`);
      } else {
        await goto(`/done`);
      }
    }
    await storeAppData();
  }

  function scheduleRestore(postponedTask: Task) {
    setTimeout(() => {
      ctx.update((d) => {
        let lastCompleteIndex = d.entries!.findLastIndex((e) => e.output);
        lastCompleteIndex =
          lastCompleteIndex >= 0 ? lastCompleteIndex : d.entries!.length - 1;

        d.entries!.splice(lastCompleteIndex, 0, postponedTask);
        return d;
      });
    }, POSTPONE_DURATION);
  }

  async function postpone() {
    await writeTask();
    const taskIndex = $ctx.entries!.findIndex(
      (e) => e.input.id === $ctx.current_entry
    );
    const nextTask = $ctx.entries![taskIndex + 1];

    ctx.update((d) => {
      const postponedTask = {
        ...d.entries!.splice(taskIndex, 1)[0]!,
        postponed: new Date().toUTCString(),
      };
      scheduleRestore(postponedTask);
      return d;
    });

    if (nextTask) {
      await goto(`/${nextTask.input.id}/highlight`);
    } else {
      await goto(`/all-postponed`);
    }
  }
</script>

<footer
  class="w-full flex justify-end py-3 px-4 border-t border-solid border-gray-100 bg-gray-50 shadow-sm"
>
  <button
    on:click={postpone}
    title="Отложенная задача вернется в список через некоторое время"
    class="px-4 py-2 mx-4 rounded border border-gray-600"
  >
    Отложить
  </button>
  {#if nextBtn}
    <button
      on:click={next}
      class="{!nextBtnEnabled &&
        'opacity-30'} px-4 py-2 rounded border border-gray-600 bg-blue-500 text-white"
      disabled={!nextBtnEnabled}
    >
      {nextBtn}
    </button>
  {/if}
</footer>
