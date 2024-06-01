import type { Config } from "tailwindcss";

export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],

  theme: {
    fontFamily: {
      'body': ['"Fira Sans"', 'sans-serif']
    },
    extend: {}
  },

  plugins: [require("@tailwindcss/typography")]
} as Config;