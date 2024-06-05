<script lang="ts">
  import { getContext } from "svelte";
  import type { Writable } from "svelte/store";
  import type { AppData } from "./types/AppData";
  import { ToneOfVoice } from "./types/Preferences";

  const ctx: Writable<AppData> = getContext("appData");
  let tone: ToneOfVoice = ToneOfVoice.Polite;
  
  $: tone = $ctx.preferences?.tone || ToneOfVoice.Polite;
</script>

{#if tone === ToneOfVoice.Familiar}
  <slot name="familiar" />
{:else if tone === ToneOfVoice.Polite}
  <slot />
{/if}
