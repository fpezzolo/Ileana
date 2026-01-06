<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  /**
   * Props per la personalizzazione del componente Cerchio
   *
   * @prop {string} circleColor - Colore del bordo del cerchio (default: "#FF5722")
   * @prop {string} textColor - Colore del testo all'interno del cerchio (default: "#E64A19")
   *
   * Esempi di utilizzo:
   * <Cerchio /> - Utilizza i colori predefiniti
   * <Cerchio circleColor="#2196F3" textColor="#0D47A1" /> - Cerchio blu
   * <Cerchio circleColor="#4CAF50" textColor="#2E7D32" /> - Cerchio verde
   */
  const {
    circleColor = "#FF5722",
    textColor = "#E64A19"
  } = $props();

  // Usiamo il tipo generico 'number | string' per includere sia il risultato che gli stati 'N/A'/'Errore'.
  let raggio = $state(0);
  let area = $state<number | string>("N/A");
  let perimetro = $state<number | string>("N/A");

  const MAX_SVG_RAGGIO = 80; // Raggio massimo del cerchio SVG
  const MIN_SVG_RAGGIO = 30; // Raggio minimo per evitare che il cerchio sparisca
  const MAX_RAGGIO_VALUE = 100; // Valore massimo accettabile per l'input raggio
  const MIN_RAGGIO_VALUE = 1; // Valore minimo accettabile per l'input raggio

  // Calcolo dinamico e REATTIVO del raggio del cerchio in pixel
  // con mapping non lineare per migliorare la percezione visiva
  const raggio_cerchio_px = $derived(
    typeof raggio === "number" && raggio > 0
      ? Math.max(
          MIN_SVG_RAGGIO,
          Math.min(
            MAX_SVG_RAGGIO,
            // Mapping non lineare per migliorare la percezione visiva
            MIN_SVG_RAGGIO + 
            (MAX_SVG_RAGGIO - MIN_SVG_RAGGIO) * 
            Math.pow(Math.min(raggio, MAX_RAGGIO_VALUE) / MAX_RAGGIO_VALUE, 0.7)
          )
        )
      : 10,
  );

  // Calcolo del diametro per la visualizzazione
  const diametro = $derived(raggio_cerchio_px * 2);

  /**
   * Formatta il risultato numerico a due decimali o restituisce lo stato stringa.
   * @param value Il valore di Area o Perimetro.
   * @returns Il valore formattato come stringa.
   */
  function formatResult(value: number | string): string {
    if (typeof value === "number") {
      return value.toFixed(2);
    }
    return value;
  }

  // Effetto reattivo per calcolare automaticamente quando il raggio cambia
  $effect(() => {
    if (typeof raggio === "number" && raggio > 0) {
      calcolaAutomaticamente();
    } else if (raggio === 0) {
      // Reset quando il valore viene azzerato
      area = "N/A";
      perimetro = "N/A";
    }
  });

  async function calcolaAutomaticamente() {
    // Convalida e conversione dell'input
    const raggioVal = parseFloat(raggio.toString());

    // Feedback immediato
    area = "Calcolo...";
    perimetro = "Calcolo...";

    if (isNaN(raggioVal) || raggioVal <= 0) {
      alert("⚠️ Inserisci un valore numerico positivo per il raggio.");
      area = "N/A";
      perimetro = "N/A";
      return;
    }

    try {
      // Chiamate a Tauri per il cerchio
      area = await invoke("calcola_area_cerchio", { raggio: raggioVal });
      perimetro = await invoke("calcola_perimetro_cerchio", { raggio: raggioVal });
    } catch (e) {
      console.error("Errore durante l'invocazione di Tauri:", e);
      area = "Errore ❌";
      perimetro = "Errore ❌";
    }
  }

  async function calcolaCerchio(event: Event) {
    event.preventDefault();
    calcolaAutomaticamente();
  }
</script>

<div class="calculator-card">
  <h2 class="calculator-title">Calcolo Cerchio</h2>
  <p class="calculator-subtitle">Calcola Area e Circonferenza utilizzando la libreria Rust ileana-lib.</p>

  <div class="calculator-layout">
    <!-- Colonna sinistra: Input -->
    <div class="input-column">
      <form class="row" onsubmit={calcolaCerchio}>
        <label for="raggio-input" class="input-label">Raggio:</label>

        <input
          id="raggio-input"
          type="number"
          step="any"
          placeholder="Raggio..."
          bind:value={raggio}
          class="raggio-input-box"
          min="0"
          oninput={e => e.currentTarget.value = Math.max(0, parseFloat(e.currentTarget.value) || 0).toString()}
        />

        <button type="submit">Calcola</button>
      </form>

      <div class="text-results">
        <p>Area: <strong>{formatResult(area)}</strong></p>
        <p>Circonferenza: <strong>{formatResult(perimetro)}</strong></p>
        {#if typeof raggio === "number" && raggio > 0}
          <p class="size-indicator">
            Dimensione: <strong>{Math.round((raggio_cerchio_px - MIN_SVG_RAGGIO) / (MAX_SVG_RAGGIO - MIN_SVG_RAGGIO) * 100)}%</strong>
          </p>
        {/if}
      </div>
    </div>

    <!-- Colonna destra: Rappresentazione grafica -->
    <div class="graphic-column">
      {#if typeof area === "number" && raggio > 0}
        <div class="circle-container">
          <svg viewBox="0 0 200 220" class="circle-svg" style={ `--circle-color: ${circleColor}; --text-color: ${textColor};` }>
            <g transform="translate(100, 100)">
              <circle
                cx="0"
                cy="0"
                r={raggio_cerchio_px}
                class="circle-border"
              />

              <!-- Posizionamento dinamico del testo che si adatta alle dimensioni -->
              <text x="0" y="-12" class="area-label">Area:</text>
              <text x="0" y="4" class="area-value">{formatResult(area)}</text>

              <!-- Testo sotto il cerchio - posizionamento fisso per evitare overflow -->
              <text x="0" y={Math.min(raggio_cerchio_px + 12, 90)} class="perimeter-text">
                Circonferenza: {formatResult(perimetro)}
              </text>

              <text x="0" y={-raggio_cerchio_px - 8} class="radius-label">
                Raggio: {raggio.toFixed(2)}
              </text>

              <!-- Limita la posizione massima per evitare overflow -->
              <text x="0" y={Math.min(raggio_cerchio_px + 25, 105)} class="diameter-label">
                Diametro: {(raggio * 2).toFixed(2)}
              </text>
            </g>
          </svg>
        </div>
      {:else}
        <div class="circle-placeholder">
          <p>Inserisci un valore valido per visualizzare il cerchio</p>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  /* Stili per il componente auto-contenuto */
  .calculator-card {
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 20px;
    margin: 20px auto;
    max-width: 800px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
    text-align: center;
  }

  .calculator-title {
    font-size: 1.5rem;
    margin-bottom: 8px;
    color: #333;
    font-weight: 600;
  }

  .calculator-subtitle {
    font-size: 0.95rem;
    color: #666;
    margin-bottom: 15px;
    font-style: italic;
  }

  .calculator-layout {
    display: flex;
    gap: 30px;
    margin-top: 20px;
  }

  .input-column {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
  }

  .graphic-column {
    flex: 1;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .row {
    display: flex;
    justify-content: center;
    gap: 10px;
  }

  /* Stile per l'etichetta del testo "Raggio:" */
  .input-label {
    align-self: center;
    white-space: nowrap;
  }

  /* Stile per accorciare il campo di input */
  .raggio-input-box {
    width: 100px;
    max-width: 100px;
  }

  .text-results {
    margin-top: 20px;
    gap: 40px;
    font-size: 1.1em;
  }

  .size-indicator {
    font-size: 0.9em;
    color: #666;
    margin-top: 8px;
    font-style: italic;
  }

  .circle-container {
    width: 200px;
    height: 220px; /* Aumentato per contenere il testo aggiuntivo */
  }

  .circle-svg {
    width: 100%;
    height: 100%;
    font-family: Arial, sans-serif;
    margin: auto;
  }

  /* Stili per gli elementi SVG - ora gestiti via CSS */
  .circle-border {
    fill: none;
    stroke: var(--circle-color, #FF5722);
    stroke-width: 2;
    transition: stroke 0.3s ease;
  }

  .area-label {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #E64A19);
    font-weight: bold;
    font-size: 14px;
  }

  .area-value {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #E64A19);
    font-weight: bold;
    font-size: 14px;
  }

  .perimeter-text {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #E64A19);
    font-size: 12px;
  }

  .radius-label {
    text-anchor: middle;
    fill: var(--text-color, #E64A19);
    font-size: 12px;
  }

  .diameter-label {
    text-anchor: middle;
    fill: var(--text-color, #E64A19);
    font-size: 12px;
  }

  .circle-placeholder {
    width: 200px;
    height: 200px;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 1px dashed #ccc;
    color: #666;
  }
</style>