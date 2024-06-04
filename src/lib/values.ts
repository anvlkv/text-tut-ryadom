import { readableColor } from "color2k";

export const LIGHT_MILD_COLORS = [
  "#8286dd",
  "#e2c78f",
  "#afb1eb",
  "#8dc593",
  "#aed3db",
  "#fddada",
  "#88bdc6",
  "#f0b3b3",
  "#b787d3",
  "#da8aa5",
  "#e48b8b",
  "#b4d9b7",
  "#e49f8c",
];

export const DARK_MILD_COLORS = [
  "#005861",
  "#192a8c",
  "#371a89",
  "#69260d",
  "#005b3d",
  "#914900",
  "#00235a",
  "#274407",
  "#466a02",
  "#034182",
  "#821e4e",
  "#590d2b",
];

export const DULL_COLORS = [
  "#fafafa",
  "#f5f5f5",
  "#e5e5e5",
  "#d4d4d4",
  "#a3a3a3",
  "#737373",
  "#525252",
  "#404040",
  "#262626",
  "#171717",
  "#0a0a0a",
  "#71717a",
];

export const WARM_COLORS = [
  "#fffbeb",
  "#fef3c7",
  "#fde68a",
  "#fcd34d",
  "#fbbf24",
  "#f59e0b",
  "#d97706",
  "#b45309",
  "#92400e",
  "#78350f",
  "#451a03",
  "#ffffff",
];

export const EARTH_COLORS = [
  "#b91c1c",
  "#c2410c",
  "#b45309",
  "#a16207",
  "#4d7c0f",
  "#15803d",
  "#047857",
  "#0f766e",
  "#0e7490",
  "#0369a1",
  "#1d4ed8",
  "#4338ca",
  "#6d28d9",
  "#7e22ce",
  "#a21caf",
  "#be185d",
  "#be123c",
];

export const COLD_COLORS = [
  "#a5f3fc",
  "#67e8f9",
  "#22d3ee",
  "#06b6d4",
  "#0891b2",
  "#0e7490",
  "#155e75",
  "#818cf8",
  "#6366f1",
  "#4f46e5",
  "#4338ca",
  "#3730a3",
  "#312e81",
  "#1e1b4b",
  "#ffffff",
];

export const GREEN_COLORS = [
  "#6ee7b7",
  "#34d399",
  "#10b981",
  "#059669",
  "#047857",
  "#065f46",
  "#5eead4",
  "#2dd4bf",
  "#14b8a6",
  "#0d9488",
  "#0f766e",
  "#115e59",
  "#134e4a",
  "#ffffff",
];

export const YELLOW_COLORS = [
  "#d97706",
  "#b45309",
  "#92400e",
  "#78350f",
  "#451a03",
  "#ea580c",
  "#c2410c",
  "#9a3412",
  "#7c2d12",
  "#431407",
  "#ca8a04",
  "#a16207",
  "#854d0e",
  "#713f12",
  "#422006",
  "#ffffff",
];

export const LIGHT_CONTRAST_COLORS = [
  "#0f172a",
  "#111827",
  "#18181b",
  "#171717",
  "#1c1917",
  "#7f1d1d",
  "#7c2d12",
  "#78350f",
  "#713f12",
  "#365314",
  "#14532d",
  "#064e3b",
  "#134e4a",
  "#164e63",
  "#0c4a6e",
  "#1e3a8a",
  "#312e81",
  "#4c1d95",
  "#581c87",
  "#701a75",
  "#831843",
  "#881337",
];

export const DARK_CONTRAST_COLORS = [
  "#f1f5f9",
  "#f3f4f6",
  "#f4f4f5",
  "#f5f5f5",
  "#f5f5f4",
  "#fee2e2",
  "#ffedd5",
  "#fef3c7",
  "#fef9c3",
  "#ecfccb",
  "#dcfce7",
  "#d1fae5",
  "#ccfbf1",
  "#cffafe",
  "#e0f2fe",
  "#dbeafe",
  "#e0e7ff",
  "#ede9fe",
  "#f3e8ff",
  "#fae8ff",
  "#fce7f3",
  "#ffe4e6",
];

export const PAIRINGS = Object.entries({
  LIGHT_MILD_COLORS,
  DARK_MILD_COLORS,
  DULL_COLORS,
  WARM_COLORS,
  EARTH_COLORS,
  COLD_COLORS,
  GREEN_COLORS,
  YELLOW_COLORS,
  LIGHT_CONTRAST_COLORS,
  DARK_CONTRAST_COLORS,
}).reduce((acc, [setName, values]) => {
  return {
    ...acc,
    [setName]: values.map((v) => readableColor(v)),
  };
}, {} as Record<string, string[]>);

export const MIN_FONT_SIZE = 8;
export const BASE_FONT_SIZE = 16;
export const MAX_FONT_SIZE = 42;
