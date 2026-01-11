<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  /**
   * Props per la personalizzazione del componente Triangolo
   *
   * @prop {string} triangleColor - Colore del bordo del triangolo (default: "#9C27B0")
   * @prop {string} textColor - Colore del testo all'interno del triangolo (default: "#7B1FA2")
   *
   * Esempi di utilizzo:
   * <Triangolo /> - Utilizza i colori predefiniti
   * <Triangolo triangleColor="#FF5722" textColor="#E64A19" /> - Triangolo arancione
   * <Triangolo triangleColor="#2196F3" textColor="#0D47A1" /> - Triangolo blu
   */
  const {
    triangleColor = "#9C27B0",
    textColor = "#7B1FA2"
  } = $props();

  // Usiamo il tipo generico 'number | string' per includere sia il risultato che gli stati 'N/A'/'Errore'.
  let lato1 = $state(0);
  let lato2 = $state(0);
  let lato_base = $state(0);
  let altezza = $state(0);
  let area = $state<number | string>("N/A");
  let perimetro = $state<number | string>("N/A");

  const MAX_VALUE = 100; // Valore massimo accettabile per l'input
  const MAX_SVG_SIZE = 120; // Dimensione massima SVG
  const MIN_SVG_SIZE = 40; // Dimensione minima SVG
  const MAX_RATIO = 3; // Rapporto massimo consentito
  const MIN_RATIO = 0.3; // Rapporto minimo consentito

  // Calcolo del rapporto con limiti per evitare estremi
  const raw_ratio = $derived(
    typeof lato_base === "number" && typeof altezza === "number" && lato_base > 0 && altezza > 0
      ? lato_base / altezza
      : 1
  );

  // Applichiamo limiti al rapporto per mantenere la visualizzazione proporzionale
  const ratio = $derived(
    Math.min(Math.max(raw_ratio, MIN_RATIO), MAX_RATIO)
  );

  // Calcolo delle dimensioni SVG mantenendo il rapporto limitato
  const base_px = $derived(
    typeof lato_base === "number" && lato_base > 0
      ? Math.min(Math.max((lato_base / MAX_VALUE) * MAX_SVG_SIZE, MIN_SVG_SIZE), MAX_SVG_SIZE)
      : MAX_SVG_SIZE / 2
  );

  const altezza_px = $derived(
    typeof altezza === "number" && altezza > 0
      ? Math.min(Math.max((altezza / MAX_VALUE) * MAX_SVG_SIZE, MIN_SVG_SIZE), MAX_SVG_SIZE)
      : MAX_SVG_SIZE / 2
  );

  // Calcoli reattivi per la posizione del testo
  const half_base = $derived(base_px / 2);
  const half_altezza = $derived(altezza_px / 2);
  
  // Posizione verticale per il testo del perimetro (sotto il triangolo)
  const perimeter_y = $derived(half_altezza + 15);
  
  // Posizione verticale per l'etichetta della base (sopra il triangolo)
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

  // Effetto reattivo per calcolare automaticamente quando i valori cambiano
  $effect(() => {
    if (typeof lato1 === "number" && typeof lato2 === "number" && typeof lato_base === "number" && typeof altezza === "number" && 
        lato1 > 0 && lato2 > 0 && lato_base > 0 && altezza > 0) {
      calcolaAutomaticamente();
    } else if (lato1 === 0 || lato2 === 0 || lato_base === 0 || altezza === 0) {
      // Reset quando uno dei valori viene azzerato
      area = "N/A";
      perimetro = "N/A";
    }
  });

  async function calcolaAutomaticamente() {
    // Convalida e conversione dell'input
    const lato1Val = parseFloat(lato1.toString());
    const lato2Val = parseFloat(lato2.toString());
    const latoBaseVal = parseFloat(lato_base.toString());
    const altezzaVal = parseFloat(altezza.toString());

    // Feedback immediato
    area = "Calcolo...";
    perimetro = "Calcolo...";

    if (isNaN(lato1Val) || isNaN(lato2Val) || isNaN(latoBaseVal) || isNaN(altezzaVal) || 
        lato1Val <= 0 || lato2Val <= 0 || latoBaseVal <= 0 || altezzaVal <= 0) {
      alert("⚠️ Inserisci valori numerici positivi per tutti i lati e l'altezza.");
      area = "N/A";
      perimetro = "N/A";
      return;
    }

    try {
      // Chiamate a Tauri per il triangolo
      area = await invoke("calcola_area_triangolo", { 
        lato1: lato1Val, 
        lato2: lato2Val, 
        lato_base: latoBaseVal, 
        altezza: altezzaVal 
      });
      perimetro = await invoke("calcola_perimetro_triangolo", { 
        lato1: lato1Val, 
        lato2: lato2Val, 
        lato_base: latoBaseVal, 
        altezza: altezzaVal 
      });
      
      // Se il risultato è 0, significa che il triangolo non è valido
      if (typeof area === "number" && area === 0 && typeof perimetro === "number" && perimetro === 0) {
        alert("⚠️ Il triangolo non è valido secondo la disuguaglianza triangolare (lato1 + lato2 deve essere > lato_base).");
      }
    } catch (e) {
      console.error("Errore durante l'invocazione di Tauri:", e);
      area = "Errore ❌";
      perimetro = "Errore ❌";
    }
  }

  async function calcolaTriangolo(event: Event) {
    event.preventDefault();
    calcolaAutomaticamente();
  }
</script>

<div class="calculator-card">
  <h2 class="calculator-title">Calcolo Triangolo</h2>
  <p class="calculator-subtitle">Calcola Area e Perimetro utilizzando la libreria Rust ileana-lib.</p>

  <div class="calculator-layout">
    <!-- Colonna sinistra: Input -->
    <div class="input-column">
      <form class="row" onsubmit={calcolaTriangolo}>
        <div class="input-group">
          <label for="lato1-input" class="input-label">Lato 1:</label>
          <input
            id="lato1-input"
            type="number"
            step="any"
            placeholder="Lato 1..."
            bind:value={lato1}
            class="input-box"
            min="0"
            oninput={e => e.currentTarget.value = Math.max(0, parseFloat(e.currentTarget.value) || 0).toString()}
          />
        </div>

        <div class="input-group">
          <label for="lato2-input" class="input-label">Lato 2:</label>
          <input
            id="lato2-input"
            type="number"
            step="any"
            placeholder="Lato 2..."
            bind:value={lato2}
            class="input-box"
            min="0"
            oninput={e => e.currentTarget.value = Math.max(0, parseFloat(e.currentTarget.value) || 0).toString()}
          />
        </div>

        <div class="input-group">
          <label for="base-input" class="input-label">Base:</label>
          <input
            id="base-input"
            type="number"
            step="any"
            placeholder="Base..."
            bind:value={lato_base}
            class="input-box"
            min="0"
            oninput={e => e.currentTarget.value = Math.max(0, parseFloat(e.currentTarget.value) || 0).toString()}
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
            min="0"
            oninput={e => e.currentTarget.value = Math.max(0, parseFloat(e.currentTarget.value) || 0).toString()}
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
      {#if typeof area === "number" && area > 0 && lato_base > 0 && altezza > 0}
        <div class="triangle-container">
          <svg viewBox="0 0 200 200" class="triangle-svg" style={ `--triangle-color: ${triangleColor}; --text-color: ${textColor};` }>
            <g transform="translate(100, 100)">
              <!-- Triangolo - usiamo un path per creare un triangolo isoscele per semplicità -->
              <path
                d={`M 0,${-half_altezza} L ${half_base},${half_altezza} L ${-half_base},${half_altezza} Z`}
                class="triangle-border"
              />

              <text x="0" y="-6" class="area-label">Area:</text>
              <text x="0" y="6" class="area-value">{formatResult(area)}</text>

              <text x="0" y={perimeter_y} class="perimeter-text">
                Perimetro: {formatResult(perimetro)}
              </text>

              <text x="0" y={base_label_y} class="side-label side-b">
                Base: {lato_base.toFixed(2)}
              </text>
              
              <text x="0" y={-half_altezza - 15} class="side-label side-a">
                Altezza: {altezza.toFixed(2)}
              </text>
            </g>
          </svg>
        </div>
      {:else}
        <div class="triangle-placeholder">
          <p>Inserisci valori validi per visualizzare il triangolo</p>
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

  /* Stile per l'etichetta del testo "Lato 1:", "Lato 2:", ecc. */
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

  .triangle-container {
    width: 200px;
    height: 200px;
  }

  .triangle-svg {
    width: 100%;
    height: 100%;
    font-family: Arial, sans-serif;
    margin: auto;
  }

  /* Stili per gli elementi SVG - ora gestiti via CSS */
  .triangle-border {
    fill: none;
    stroke: var(--triangle-color, #9C27B0);
    stroke-width: 2;
    transition: stroke 0.3s ease;
  }

  .area-label {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #7B1FA2);
    font-weight: bold;
    font-size: 14px;
  }

  .area-value {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #7B1FA2);
    font-weight: bold;
    font-size: 14px;
  }

  .perimeter-text {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #7B1FA2);
    font-size: 12px;
  }

  .side-label {
    text-anchor: middle;
    fill: var(--text-color, #7B1FA2);
    font-size: 12px;
  }

  .side-a {
    dominant-baseline: central;
  }

  .side-b {
    text-anchor: middle;
  }

  .triangle-placeholder {
    width: 200px;
    height: 200px;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 1px dashed #ccc;
    color: #666;
  }
</style>