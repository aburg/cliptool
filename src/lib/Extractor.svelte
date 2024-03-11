<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let filenames = "";
  let show = false;

  async function greet() {
    filenames = await invoke("filenames");
  }
</script>

<div class="row">
  <form class="row" on:submit|preventDefault={greet}>
    <button type="submit">Dateinamen extrahieren</button>
  </form>
</div>

{#if filenames}
  <h2>
    &check; {filenames.split("\n").length} Zeil(en) verarbeitet
  </h2>
  <div class="row" style="font-size: 8pt;">
    {filenames}
  </div>
{/if}
