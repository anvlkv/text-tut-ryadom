<script lang="ts">
    import { goto } from "$app/navigation";
    import { page } from "$app/stores";
    import type { AppData } from "$lib/types/AppData";
    import { type Task } from "$lib/types/Task";
    import { invoke } from "@tauri-apps/api/core";
    import { getContext } from "svelte";
    import type { Writable } from "svelte/store";
    import Button from "./button.svelte";
    import { labelStudioPost } from "./labelStudio";

    const ctx: Writable<AppData> = getContext("appData");
    const POSTPONE_DURATION = 5 * 60 * 1000;

    $: task = $ctx.entries?.find((e) => e.input.id === $ctx.current_entry);

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
                      completed_ts: completed.toISOString(),
                  },
              };
        console.log(saveTask, $ctx.label_studio_src);

        await invoke("write_task", { task: saveTask });

        ctx.update((c) => {
            const i = c.entries!.findIndex(
                (e) => e.input.id === saveTask.input.id,
            );
            c.entries![i] = saveTask;
            return c;
        });
    }

    async function next() {
        if ($page.route.id?.endsWith("[task]/highlight")) {
            await writeTask();
            await goto(`/${$page.params.task}/summarize`);
        } else {
            await writeTask(new Date());
            const taskIndex = $ctx.entries!.findIndex(
                (e) => e.input.id === $ctx.current_entry,
            );
            const nextTask = $ctx.entries![taskIndex + 1];
            if (nextTask && nextTask.output?.completed_ts !== null) {
                await goto(`/${nextTask.input.id}/highlight`);
            } else if ($ctx.entries!.some((e) => !e.output?.completed_ts)) {
                const firstIncomplete = $ctx.entries!.find(
                    (e) => !e.output?.completed_ts,
                )!;
                await goto(`/${firstIncomplete.input.id}/highlight`);
            } else {
                await goto(`/done`);
            }
        }
        await storeAppData();
    }

    function scheduleRestore(postponedTask: Task) {
        setTimeout(() => {
            ctx.update((d) => {
                let lastCompleteIndex = d.entries!.findLastIndex(
                    (e) => e.output,
                );
                lastCompleteIndex =
                    lastCompleteIndex >= 0
                        ? lastCompleteIndex
                        : d.entries!.length - 1;

                d.entries!.splice(lastCompleteIndex, 0, postponedTask);
                return d;
            });
        }, POSTPONE_DURATION);
    }

    async function postpone() {
        await writeTask();
        const taskIndex = $ctx.entries!.findIndex(
            (e) => e.input.id === $ctx.current_entry,
        );
        const nextTask = $ctx.entries![taskIndex + 1];

        ctx.update((d) => {
            const postponedTask = {
                ...d.entries!.splice(taskIndex, 1)[0]!,
                postponed: new Date().toISOString(),
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
    class="w-full flex justify-end py-3 px-4 bg-gray-50 dark:bg-gray-900 dc:bg-stone-950 lc:bg-stone-50 yellow:bg-yellow-50 green:bg-green-50 warm:bg-red-50 cold:bg-blue-50 earth:bg-earth-50 shadow-sm"
>
    <Button
        on:click={postpone}
        title="Отложенная задача вернется в список через некоторое время"
        __class="mx-4"
    >
        Отложить
    </Button>
    {#if nextBtn}
        <Button on:click={next} disabled={!nextBtnEnabled} cta={true}>
            {nextBtn}
        </Button>
    {/if}
</footer>
