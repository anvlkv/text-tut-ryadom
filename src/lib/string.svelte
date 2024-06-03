<script lang="ts">
  import { getContext } from "svelte";
  import { ToneOfVoice } from "./types/Preferences";
  import type { Writable } from "svelte/store";
  import type { AppData } from "./types/AppData";

  const ctx: Writable<AppData> = getContext("appData");
  let tone: ToneOfVoice = ToneOfVoice.Polite;
  
  $: tone = $ctx.preferences?.tone || ToneOfVoice.Polite;
</script>

{#if tone === ToneOfVoice.Familiar}
  <slot name="familiar" />
{:else if tone === ToneOfVoice.Polite}
  <slot />
{/if}
