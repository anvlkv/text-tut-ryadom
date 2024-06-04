import {
  LIGHT_MILD_COLORS,
  DARK_MILD_COLORS,
  DULL_COLORS,
  YELLOW_COLORS,
  COLD_COLORS,
  DARK_CONTRAST_COLORS,
  EARTH_COLORS,
  GREEN_COLORS,
  LIGHT_CONTRAST_COLORS,
  WARM_COLORS,
  PAIRINGS,
} from "./values";
import { ColorSchema } from "./types/Preferences";

export function highlightColor(schema: ColorSchema, color: number): string {
  switch (schema) {
    case ColorSchema.BrightLight:
      return LIGHT_MILD_COLORS[color % LIGHT_MILD_COLORS.length];
    case ColorSchema.BrightDark:
      return DARK_MILD_COLORS[color % DARK_MILD_COLORS.length];
    case ColorSchema.Dull:
      return DULL_COLORS[color % DULL_COLORS.length];
    case ColorSchema.HighContrastDark:
      return DARK_CONTRAST_COLORS[color % DARK_CONTRAST_COLORS.length];
    case ColorSchema.HighContrastLight:
      return LIGHT_CONTRAST_COLORS[color % LIGHT_CONTRAST_COLORS.length];
    case ColorSchema.SoftGreen:
      return GREEN_COLORS[color % GREEN_COLORS.length];
    case ColorSchema.SoftYellow:
      return YELLOW_COLORS[color % YELLOW_COLORS.length];
    case ColorSchema.SoftCold:
      return COLD_COLORS[color % COLD_COLORS.length];
    case ColorSchema.SoftWarm:
      return WARM_COLORS[color % WARM_COLORS.length];
    case ColorSchema.SoftEarth:
      return EARTH_COLORS[color % EARTH_COLORS.length];
    default:
      return "gray";
  }
}

export function textColor(schema: ColorSchema, color: number): string {
  switch (schema) {
    case ColorSchema.BrightLight:
      return PAIRINGS["LIGHT_MILD_COLORS"][color % LIGHT_MILD_COLORS.length];
    case ColorSchema.BrightDark:
      return PAIRINGS["DARK_MILD_COLORS"][color % DARK_MILD_COLORS.length];
    case ColorSchema.Dull:
      return PAIRINGS["DULL_COLORS"][color % DULL_COLORS.length];
    case ColorSchema.HighContrastDark:
      return PAIRINGS["DARK_CONTRAST_COLORS"][
        color % DARK_CONTRAST_COLORS.length
      ];
    case ColorSchema.HighContrastLight:
      return PAIRINGS["LIGHT_CONTRAST_COLORS"][
        color % LIGHT_CONTRAST_COLORS.length
      ];
    case ColorSchema.SoftGreen:
      return PAIRINGS["GREEN_COLORS"][color % GREEN_COLORS.length];
    case ColorSchema.SoftYellow:
      return PAIRINGS["YELLOW_COLORS"][color % YELLOW_COLORS.length];
    case ColorSchema.SoftCold:
      return PAIRINGS["COLD_COLORS"][color % COLD_COLORS.length];
    case ColorSchema.SoftWarm:
      return PAIRINGS["WARM_COLORS"][color % WARM_COLORS.length];
    case ColorSchema.SoftEarth:
      return PAIRINGS["EARTH_COLORS"][color % EARTH_COLORS.length];
    default:
      return "black";
  }
}
