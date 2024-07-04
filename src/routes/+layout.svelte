<script lang="ts">
    import { goto } from "$app/navigation";
    import Header from "$lib/header.svelte";
    import type { AppData, LabelStudioSource } from "$lib/types/AppData";
    import { ColorSchema } from "$lib/types/Preferences";
    import type { Task } from "$lib/types/Task";
    import { BASE_FONT_SIZE, MAX_FONT_SIZE, MIN_FONT_SIZE } from "$lib/values";
    import { invoke } from "@tauri-apps/api/core";
    import { open, save } from "@tauri-apps/plugin-dialog";
    import {
        BaseDirectory,
        mkdir,
        create,
        exists,
        readTextFile,
        writeTextFile,
    } from "@tauri-apps/plugin-fs";
    import { onMount, setContext } from "svelte";
    import { writable } from "svelte/store";
    import "../app.css";
    import { labelStudioGet } from "$lib/labelStudio";

    const APP_CONFIG_FILE = "config/app_data.json";

    const data = setContext("appData", writable({} as AppData));
    setContext("storeAppData", storeAppConfig);
    setContext("loadDir", loadDir);
    setContext("loadLabelStudio", loadLabelStudio);
    setContext("prerequisites", prerequisites);

    let theme = ColorSchema.Dull;
    let fontSizeCategory: undefined | string;

    $: $data.activeSchema = theme;

    async function storeAppConfig() {
        const { src_dir, current_entry, preferences, label_studio_src } = $data;

        try {
            if (
                !(await exists(APP_CONFIG_FILE, {
                    baseDir: BaseDirectory.AppConfig,
                }))
            ) {
                await create(APP_CONFIG_FILE, {
                    baseDir: BaseDirectory.AppConfig,
                });
            }

            await writeTextFile(
                APP_CONFIG_FILE,
                JSON.stringify({
                    src_dir,
                    current_entry,
                    preferences,
                    label_studio_src,
                }),
                { baseDir: BaseDirectory.AppConfig },
            );

            console.info("app data stored");
        } catch (e) {
            console.error("failed to store app data", e);
        }
    }

    async function loadLabelStudio() {
        data.update((d) => {
            d.loading = true;
            return d;
        });
        try {
            const config = $data.label_studio_src! as LabelStudioSource;

            const tasks_data = await labelStudioGet<{
                tasks: {
                    id: number;
                    data: {
                        text: string | string[];
                    };
                    annotations_results: string;
                    completed_at: any;
                }[];
            }>(
                `tasks?page_size=${config.page_size}&view=${config.view}&project=${config.project}`,
                config,
            );
            console.log(tasks_data);
            const tasks: Task[] = tasks_data.tasks.map((d) => ({
                input: {
                    id: d.id.toString(),
                    text: Array.isArray(d.data.text)
                        ? d.data.text.join("\n")
                        : d.data.text,
                },
                output: d.annotations_results
                    ? (() => {
                          const {
                              value: { text },
                          } = JSON.parse(d.annotations_results);
                          return {
                              text: text[0] as string,
                              text_id: d.id.toString(),
                              completed_ts: d.completed_at,
                          };
                      })()
                    : null,
                highlights: [],
                origin: config.label_studio_url,
                postponed: null,
            }));

            $data.entries = tasks;
            $data.total_entries = tasks.length;

            const entryIndex = $data.current_entry
                ? $data.entries.findIndex(
                      (e) => e.input.id === $data.current_entry,
                  )
                : $data.entries.findIndex((e) => !e.output);
            $data.current_entry = $data.entries[entryIndex]!.input.id;

            await openProjectPage();
        } catch (e) {
            console.error(e);
            $data.label_studio_src = undefined;
            await goto("/directory");
        }
        $data.loading = false;
    }

    async function loadDir() {
        console.info("loading dir");

        let dir;

        if (!$data.src_dir) {
            dir = (await open({
                title: "Выбери папку с проектом",
                directory: true,
            }).then((d) => {
                $data.loading = true;
                return d;
            })!) as string;
        } else {
            dir = $data.src_dir;
        }

        $data.src_dir = dir;

        try {
            const tasks: Task[] = await invoke("read_dir_tasks", { dir });
            $data.entries = tasks;
            $data.total_entries = tasks.length;

            const entryIndex = $data.current_entry
                ? $data.entries.findIndex(
                      (e) => e.input.id === $data.current_entry,
                  )
                : $data.entries.findIndex((e) => !e.output);
            $data.current_entry = $data.entries[entryIndex]?.input.id;

            await openProjectPage();
        } catch {
            $data.src_dir = undefined;
            console.warn("loading dir failed. try again");
        }
        $data.loading = false;
    }

    async function openProjectPage() {
        const entry_id = $data.current_entry! || $data.entries![0].input.id;
        const entry_step = $data.entries!.find((e) => e.input.id === entry_id)
            ?.output
            ? "summarize"
            : "highlight";

        await goto(`/${entry_id}/${entry_step}`, { replaceState: true });
    }

    async function prerequisites() {
        console.log($data);
        if (!$data.preferences) {
            await goto("/setup/tone");
        } else if (!$data.src_dir && !$data.label_studio_src) {
            await goto("/directory");
        } else if (!$data.entries) {
            if ($data.src_dir) {
                await loadDir();
            } else if ($data.label_studio_src) {
                await loadLabelStudio();
                // FIXME: the app is ready to post annotations to LS, but the `result` format must match the format expected by the original editor.
                await importSaveDir();
            } else {
                await goto("/directory");
            }
        } else {
            await openProjectPage();
        }
    }

    async function importSaveDir() {
        const savePath = await save({
            canCreateDirectories: true,
            title: "Где сохранить проект?",
        });

        if (savePath && $data.entries) {
            await mkdir(savePath);
            await invoke("save_dir_tasks", {
                dir: savePath,
                tasks: $data.entries,
            });
            $data.src_dir = savePath;
            $data.label_studio_src = undefined;
            await loadDir();
        }
    }

    async function loadAppConfig() {
        console.info("loading");
        try {
            const { src_dir, current_entry, preferences, label_studio_src } =
                JSON.parse(
                    await readTextFile(APP_CONFIG_FILE, {
                        baseDir: BaseDirectory.AppConfig,
                    }),
                );
            $data.current_entry = current_entry;
            $data.src_dir = src_dir;
            $data.label_studio_src = label_studio_src;
            $data.preferences = preferences;
            console.info("loaded from config");
        } catch {
            $data.src_dir = undefined;
            $data.current_entry = undefined;
            $data.total_entries = undefined;
            console.info("no config");
        }
    }

    function sysDark(dark: boolean, contrast: boolean) {
        if (contrast) {
            if (dark) {
                theme = ColorSchema.HighContrastDark;
            } else {
                theme = ColorSchema.HighContrastLight;
            }
        } else {
            if (dark) {
                theme = ColorSchema.BrightDark;
            } else {
                theme = ColorSchema.BrightLight;
            }
        }
    }

    onMount(() => {
        const mq = window.matchMedia("(prefers-color-scheme: dark)");
        let listener: undefined | ((ev: MediaQueryListEvent) => void);
        data.subscribe((d) => {
            if (d.preferences) {
                if (listener) {
                    mq.removeEventListener("change", listener);
                }
                if (
                    [
                        ColorSchema.BrightAuto,
                        ColorSchema.HighContrastAuto,
                    ].includes(d.preferences.colorSchema)
                ) {
                    const contrast =
                        ColorSchema.HighContrastAuto ===
                        d.preferences.colorSchema;
                    listener = (ev: MediaQueryListEvent) => {
                        sysDark(ev.matches, contrast);
                    };
                    mq.addEventListener("change", listener);
                } else {
                    theme = d.preferences.colorSchema;
                }

                const mDiff = MAX_FONT_SIZE - BASE_FONT_SIZE;
                const hDiff = BASE_FONT_SIZE - MIN_FONT_SIZE;

                if (d.preferences.fontSize < BASE_FONT_SIZE) {
                    if (BASE_FONT_SIZE - d.preferences.fontSize <= hDiff / 2) {
                        fontSizeCategory = "x-hyp";
                    } else {
                        fontSizeCategory = "hyp";
                    }
                } else if (d.preferences.fontSize > BASE_FONT_SIZE) {
                    if (d.preferences.fontSize - BASE_FONT_SIZE >= mDiff / 2) {
                        fontSizeCategory = "x-myo";
                    } else {
                        fontSizeCategory = "myo";
                    }
                }
            }
        });

        loadAppConfig()
            .then(() => {
                console.info("app data loaded");
                return prerequisites();
            })
            .then(() => {
                console.info("app prerequisites complete");
                return invoke("close_splashscreen");
            })
            .then(() => {
                console.info("app ready");
            });

        return () => {
            if (listener) {
                mq.removeEventListener("change", listener);
            }
        };
    });

    $: fontSize = $data.preferences?.fontSize;
</script>

<div
    class="contents text-base leading-normal"
    style="--font-size: {fontSize}px"
    data-theme={theme}
    data-font-size={fontSizeCategory}
>
    <main
        class="font-body relative select-none h-screen w-screen overflow-hidden flex flex-col items-stretch justify-stretch bg-gray-50 text-gray-950 dull:bg-gray-200 dull:text-gray-800 dc:bg-black dc:text-white lc:bg-white lc:text-black dark:bg-gray-950 dark:text-gray-50 earth:bg-earth-100 earth:text-emerald-950 yellow:bg-yellow-100 yellow:text-yellow-950 green:bg-green-100 green:text-green-950 warm:bg-orange-100 warm:text-rose-950 cold:bg-sky-100 cold:text-blue-950"
    >
        <Header />
        <slot />
    </main>
</div>
