export interface Preferences {
  fontSize: number,
  colorSchema: ColorSchema,
  tone: ToneOfVoice,
}

export enum ToneOfVoice {
  Polite,
  Familiar
}

export enum ColorSchema {
  Dull = "dull",
  BrightLight = "light",
  BrightDark = "dark",
  BrightAuto = "system",
  HighContrastDark = "dark-contrast",
  HighContrastLight = "light-contrast",
  HighContrastAuto = "sys-contrast",
  SoftGreen = "green",
  SoftYellow = "yellow",
  SoftCold = "cold",
  SoftWarm = "warm",
  SoftEarth = "earth",
}