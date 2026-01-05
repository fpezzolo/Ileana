<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  /**
   * Props per la personalizzazione del componente Rettangolo
   *
   * @prop {string} rectColor - Colore del bordo del rettangolo (default: "#2196F3")
   * @prop {string} textColor - Colore del testo all'interno del rettangolo (default: "#0D47A1")
   *
   * Esempi di utilizzo:
   * <Rettangolo /> - Utilizza i colori predefiniti
   * <Rettangolo rectColor="#FF5722" textColor="#E64A19" /> - Rettangolo arancione
   * <Rettangolo rectColor="#4CAF50" textColor="#2E7D32" /> - Rettangolo verde
   */
  const {
    rectColor = "#2196F3",
    textColor = "#0D47A1"
  } = $props();

  // Usiamo il tipo generico 'number | string' per includere sia il risultato che gli stati 'N/A'/'Errore'.
  let base = $state(0);
  let altezza = $state(0);
  let area = $state<number | string>("N/A");
  let perimetro = $state<number | string>("N/A");

  const MAX_VALUE = 170; // Valore massimo accettabile per l'input
  const MAX_SVG_SIZE = 150; // Dimensione massima SVG (per il lato più lungo)
  const MIN_SVG_SIZE = 40; // Dimensione minima SVG (per il lato più corto)
  const MAX_RATIO = 5; // Rapporto massimo consentito (es. 5:1)
  const MIN_RATIO = 0.2; // Rapporto minimo consentito (es. 1:5)

  // Calcolo del rapporto con limiti per evitare estremi
  const raw_ratio = $derived(
    typeof base === "number" && typeof altezza === "number" && base > 0 && altezza > 0
      ? base / altezza
      : 1
  );

  // Applichiamo limiti al rapporto per mantenere la visualizzazione proporzionale
  const ratio = $derived(
    Math.min(Math.max(raw_ratio, MIN_RATIO), MAX_RATIO)
  );

  // Calcolo delle dimensioni SVG mantenendo il rapporto limitato
  // Usiamo una dimensione fissa per il lato più lungo e scaliamo l'altro mantenendo il rapporto
  const base_px = $derived(
    typeof base === "number" && base > 0 && typeof altezza === "number" && altezza > 0
      ? Math.min(Math.max((base / (base + altezza)) * MAX_SVG_SIZE * 1.5, MIN_SVG_SIZE), MAX_SVG_SIZE)
      : MAX_SVG_SIZE
  );

  const altezza_px = $derived(
    typeof altezza === "number" && altezza > 0 && typeof base === "number" && base > 0
      ? Math.min(Math.max((altezza / (base + altezza)) * MAX_SVG_SIZE * 1.5, MIN_SVG_SIZE), MAX_SVG_SIZE)
      : MAX_SVG_SIZE
  );

  // Calcoli reattivi per la posizione del testo
  const half_base = $derived(base_px / 2);
  const half_altezza = $derived(altezza_px / 2);
  
  // Posizione verticale per il testo del perimetro (sotto il rettangolo)
  const perimeter_y = $derived(half_altezza + 15);
  
  // Posizione verticale per l'etichetta della base (sopra il rettangolo)
  const base_label_y = $derived(-half_altezza - 10);

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

  // Effetto reattivo per calcolare automaticamente quando base o altezza cambiano
  $effect(() => {
    if (typeof base === "number" && typeof altezza === "number" && base > 0 && altezza > 0) {
      calcolaAutomaticamente();
    } else if (base === 0 || altezza === 0) {
      // Reset quando uno dei valori viene azzerato
      area = "N/A";
      perimetro = "N/A";
    }
  });

  async function calcolaAutomaticamente() {
    // Convalida e conversione dell'input
    const baseVal = parseFloat(base.toString());
    const altezzaVal = parseFloat(altezza.toString());

    // Feedback immediato
    area = "Calcolo...";
    perimetro = "Calcolo...";

    if (isNaN(baseVal) || isNaN(altezzaVal) || baseVal <= 0 || altezzaVal <= 0) {
      alert("⚠️ Inserisci valori numerici positivi per base e altezza.");
      area = "N/A";
      perimetro = "N/A";
      return;
    }

    try {
      // Chiamate a Tauri per il rettangolo
      area = await invoke("calcola_area_rettangolo", { base: baseVal, altezza: altezzaVal });
      perimetro = await invoke("calcola_perimetro_rettangolo", { base: baseVal, altezza: altezzaVal });
    } catch (e) {
      console.error("Errore durante l'invocazione di Tauri:", e);
      area = "Errore ❌";
      perimetro = "Errore ❌";
    }
  }

  async function calcolaRettangolo(event: Event) {
    event.preventDefault();
    calcolaAutomaticamente();
  }
</script>

<div class="calculator-card">
  <h2 class="calculator-title">Calcolo Rettangolo</h2>
  <p class="calculator-subtitle">Calcola Area e Perimetro utilizzando la libreria Rust ileana-lib.</p>

  <div class="calculator-layout">
    <!-- Colonna sinistra: Input -->
    <div class="input-column">
      <form class="row" onsubmit={calcolaRettangolo}>
        <div class="input-group">
          <label for="base-input" class="input-label">Base:</label>
          <input
            id="base-input"
            type="number"
            step="any"
            placeholder="Base..."
            bind:value={base}
            class="input-box"
          />
        </div>

        <div class="input-group">
          <label for="altezza-input" class="input-label">Altezza:</label>
          <input
            id="altezza-input"
            type="number"
            step="any"
            placeholder="Altezza..."
            bind:value={altezza}
            class="input-box"
          />
        </div>

        <button type="submit">Calcola</button>
      </form>

      <div class="text-results">
        <p>Area: <strong>{formatResult(area)}</strong></p>
        <p>Perimetro: <strong>{formatResult(perimetro)}</strong></p>
      </div>
    </div>

    <!-- Colonna destra: Rappresentazione grafica -->
    <div class="graphic-column">
      {#if typeof area === "number" && base > 0 && altezza > 0}
        <div class="rectangle-container">
          <svg viewBox="0 0 200 200" class="rectangle-svg" style={ `--rect-color: ${rectColor}; --text-color: ${textColor};` }>
            <g transform="translate(100, 100)">
              <rect
                x={-half_base}
                y={-half_altezza}
                width={base_px}
                height={altezza_px}
                class="rectangle-border"
              />

              <text x="0" y="-6" class="area-label">Area:</text>
              <text x="0" y="6" class="area-value">{formatResult(area)}</text>

              <text x="0" y={perimeter_y} class="perimeter-text">
                Perimetro: {formatResult(perimetro)}
              </text>

              <text x="0" y={base_label_y} class="side-label side-b">
                Base: {base.toFixed(2)}
              </text>
              
              <!-- Debug: Mostra i valori calcolati -->
              <text x="0" y={half_altezza + 30} class="side-label side-b" style="fill: red; font-size: 10px;">
                Debug: {base_px.toFixed(1)}x{altezza_px.toFixed(1)} (raw: {raw_ratio.toFixed(2)}, adj: {ratio.toFixed(2)})
              </text>

              <g transform="rotate(-90)">
                <text x="0" y={-half_altezza + 10} class="side-label side-a">
                  Altezza: {altezza.toFixed(2)}
                </text>
              </g>
            </g>
          </svg>
        </div>
      {:else}
        <div class="rectangle-placeholder">
          <p>Inserisci valori validi per visualizzare il rettangolo</p>
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
    flex-direction: column;
    gap: 10px;
  }

  .input-group {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  /* Stile per l'etichetta del testo "Base:" e "Altezza:" */
  .input-label {
    white-space: nowrap;
    min-width: 80px;
  }

  /* Stile per i campi di input */
  .input-box {
    width: 100px;
    max-width: 100px;
  }

  .text-results {
    margin-top: 20px;
    gap: 40px;
    font-size: 1.1em;
  }

  .rectangle-container {
    width: 200px;
    height: 200px;
  }

  .rectangle-svg {
    width: 100%;
    height: 100%;
    font-family: Arial, sans-serif;
    margin: auto;
  }

  /* Stili per gli elementi SVG - ora gestiti via CSS */
  .rectangle-border {
    fill: none;
    stroke: var(--rect-color, #2196F3);
    stroke-width: 2;
    transition: stroke 0.3s ease;
  }

  .area-label {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #0D47A1);
    font-weight: bold;
    font-size: 14px;
  }

  .area-value {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #0D47A1);
    font-weight: bold;
    font-size: 14px;
  }

  .perimeter-text {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #0D47A1);
    font-size: 12px;
  }

  .side-label {
    text-anchor: middle;
    fill: var(--text-color, #0D47A1);
    font-size: 12px;
  }

  .side-a {
    dominant-baseline: central;
  }

  .side-b {
    text-anchor: middle;
  }

  .rectangle-placeholder {
    width: 200px;
    height: 200px;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 1px dashed #ccc;
    color: #666;
  }
</style>