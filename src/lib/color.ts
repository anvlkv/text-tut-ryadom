import { LIGHT_MILD_COLORS, DARK_MILD_COLORS } from "./defaults";
import { ColorSchema } from "./types/Preferences";

export function highlightColor(schema: ColorSchema, color: number): string {
  switch (schema) {
    case ColorSchema.BrightLight:
      return LIGHT_MILD_COLORS[color];
    case ColorSchema.BrightDark:
      return DARK_MILD_COLORS[color];
    case ColorSchema.Dull:
    case ColorSchema.HighContrastDark:
    case ColorSchema.HighContrastLight:
    case ColorSchema.SoftGreen:
    case ColorSchema.SoftYellow:
    case ColorSchema.SoftCold:
    case ColorSchema.SoftWarm:
    case ColorSchema.SoftEarth:
    default:
      return 'gray'
  }
}