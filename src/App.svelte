<script lang="ts">
  import Extractor from "./lib/Extractor.svelte";
</script>

<main class="container">
  <h1>Hi Papa!</h1>

  <div>
    Mit diesem Programm kannst du die Dateinamen von ausgewählten Dateien in
    "Excel" bekommen.
  </div>

  <div class="row">
    <ol>
      <li>
        Suche und markiere die Dateien in deinem "normalen" Dateimanager und
        drücke dann STRG+C.
      </li>
      <li>
        Klicke auf den Knopf mit der Aufschrift "Dateinamen extrahieren" (siehe
        unten)
      </li>
      <li>Öffne dein "Excel" und drücke dort STRG+V.</li>
    </ol>
  </div>

  <div class="row">
    ----------------------------------------------------------------
    ----------------------------------------------------------------
  </div>

  <Extractor />
</main>
