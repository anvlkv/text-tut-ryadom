import type { Config } from "tailwindcss";
import plugin from "tailwindcss/plugin";

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  darkMode: ["variant", '&:is([data-theme="dark"] *)'],
  theme: {
    fontFamily: {
      body: ['"Fira Sans"', "sans-serif"],
    },
    fontSize: {
      xs: ["calc(0.75 * var(--font-size))", "calc(1 * var(--font-size))"],
      sm: ["calc(0.875 * var(--font-size))", "calc(1.25 * var(--font-size))"],
      base: ["calc(1 * var(--font-size))", "calc(1.5 * var(--font-size))"],
      lg: ["calc(1.125 * var(--font-size))", "calc(1.75 * var(--font-size))"],
      xl: ["calc(1.25 * var(--font-size))", "calc(1.75 * var(--font-size))"],
      "2xl": ["calc(1.5 * var(--font-size))", "calc(2 * var(--font-size))"],
      "3xl": ["calc(2 * var(--font-size))", "calc(2.25 * var(--font-size))"],
      "4xl": ["calc(2.25 * var(--font-size))", "calc(2.5 * var(--font-size))"],
      "5xl": "calc(3 * var(--font-size))",
      "6xl": "calc(3.75 * var(--font-size))",
      "7xl": "calc(4.5 * var(--font-size))",
      "8xl": "calc(6 * var(--font-size))",
      "9xl": "calc(8 * var(--font-size))",
    },
    extend: {
      lineHeight: {
        "leading-3": "calc(0.75 * var(--font-size))",
        "leading-4": "calc(1 * var(--font-size))",
        "leading-5": "calc(1.25 * var(--font-size))",
        "leading-6": "calc(1.5 * var(--font-size))",
        "leading-7": "calc(1.75 * var(--font-size))",
        "leading-8": "calc(2 * var(--font-size))",
        "leading-9": "calc(2.25 * var(--font-size))",
        "leading-10": "calc(2.75 * var(--font-size))",
      },
      colors: {
        earth: {
          950: "#1A1817",
          900: "#302D29",
          800: "#47433C",
          700: "#5E594F",
          600: "#756E62",
          500: "#8C8476",
          400: "#A39989",
          300: "#BAAF9C",
          200: "#D1C4AF",
          100: "#E8DAC3",
          50: "#FFF4E3",
        },
      },
    },
  },

  plugins: [
    require("@tailwindcss/typography"),
    plugin(function ({ addVariant }) {
      // color schema variants
      addVariant("dull", '&:is([data-theme="dull"] *)');
      addVariant("dc", '&:is([data-theme="dark-contrast"] *)');
      addVariant("lc", '&:is([data-theme="light-contrast"] *)');
      addVariant("green", '&:is([data-theme="green"] *)');
      addVariant("yellow", '&:is([data-theme="yellow"] *)');
      addVariant("cold", '&:is([data-theme="cold"] *)');
      addVariant("warm", '&:is([data-theme="warm"] *)');
      addVariant("earth", '&:is([data-theme="earth"] *)');

      // font-size variants
      // prefers small font
      addVariant("x-hyp", '&:is([data-font-size="x-hyp"] *)');
      addVariant("hyp", '&:is([data-font-size$="hyp"] *)');
      // prefers large font
      addVariant("myo", '&:is([data-font-size$="myo"] *)');
      addVariant("x-myo", '&:is([data-font-size="x-myo"] *)');
    }),
  ],
} as Config;
