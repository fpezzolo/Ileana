<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  /**
   * Props per la personalizzazione del componente Rombo
   *
   * @prop {string} rhombusColor - Colore del bordo del rombo (default: "#FF9800")
   * @prop {string} textColor - Colore del testo all'interno del rombo (default: "#E65100")
   *
   * Esempi di utilizzo:
   * <Rombo /> - Utilizza i colori predefiniti
   * <Rombo rhombusColor="#2196F3" textColor="#0D47A1" /> - Rombo blu
   * <Rombo rhombusColor="#4CAF50" textColor="#2E7D32" /> - Rombo verde
   */
  const {
    rhombusColor = "#FF9800",
    textColor = "#E65100"
  } = $props();

  // Usiamo il tipo generico 'number | string' per includere sia il risultato che gli stati 'N/A'/'Errore'.
  let diagonale1 = $state(0);
  let diagonale2 = $state(0);
  let lato = $state(0);
  let area = $state<number | string>("N/A");
  let perimetro = $state<number | string>("N/A");

  const MAX_VALUE = 200; // Valore massimo accettabile per l'input
  const MAX_SVG_SIZE = 120; // Dimensione massima SVG
  const MIN_SVG_SIZE = 40; // Dimensione minima SVG
  const MAX_RATIO = 3; // Rapporto massimo consentito tra diagonali
  const MIN_RATIO = 0.3; // Rapporto minimo consentito tra diagonali

  // Calcolo del rapporto con limiti per evitare estremi
  const raw_ratio = $derived(
    typeof diagonale1 === "number" && typeof diagonale2 === "number" && diagonale1 > 0 && diagonale2 > 0
      ? diagonale1 / diagonale2
      : 1
  );

  // Applichiamo limiti al rapporto per mantenere la visualizzazione proporzionale
  const ratio = $derived(
    Math.min(Math.max(raw_ratio, MIN_RATIO), MAX_RATIO)
  );

  // Calcolo delle dimensioni SVG mantenendo il rapporto limitato
  const diag1_px = $derived(
    typeof diagonale1 === "number" && diagonale1 > 0
      ? Math.min(Math.max((diagonale1 / MAX_VALUE) * MAX_SVG_SIZE * 1.5, MIN_SVG_SIZE), MAX_SVG_SIZE)
      : MAX_SVG_SIZE
  );

  const diag2_px = $derived(
    typeof diagonale2 === "number" && diagonale2 > 0
      ? Math.min(Math.max((diagonale2 / MAX_VALUE) * MAX_SVG_SIZE * 1.5, MIN_SVG_SIZE), MAX_SVG_SIZE)
      : MAX_SVG_SIZE
  );

  // Calcoli reattivi per la posizione del testo
  const half_diag1 = $derived(diag1_px / 2);
  const half_diag2 = $derived(diag2_px / 2);
  
  // Calcolo dinamico delle dimensioni del font in base alle dimensioni del rombo
  const font_size = $derived(Math.max(8, Math.min(14, Math.sqrt(diag1_px * diag2_px) / 10)));
  
  // Posizione verticale per il testo del perimetro (sotto il rombo) - scalata in base alla dimensione
  const perimeter_y = $derived(half_diag2 + Math.max(20, diag2_px * 0.2));
  
  // Posizione verticale per l'etichetta della diagonale 1 - scalata in base alla dimensione
  const diag1_label_y = $derived(-half_diag2 - Math.max(25, diag2_px * 0.2));
  
  // Posizione verticale per l'etichetta della diagonale 2 - scalata in base alla dimensione
  const diag2_label_offset = $derived(Math.max(10, diag2_px * 0.1));

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
    if (typeof diagonale1 === "number" && typeof diagonale2 === "number" && typeof lato === "number" &&
        diagonale1 > 0 && diagonale2 > 0 && lato > 0) {
      calcolaAutomaticamente();
    } else if (diagonale1 === 0 || diagonale2 === 0 || lato === 0) {
      // Reset quando uno dei valori viene azzerato
      area = "N/A";
      perimetro = "N/A";
    }
  });

  async function calcolaAutomaticamente() {
    // Convalida e conversione dell'input
    const diag1Val = parseFloat(diagonale1.toString());
    const diag2Val = parseFloat(diagonale2.toString());
    const latoVal = parseFloat(lato.toString());

    // Feedback immediato
    area = "Calcolo...";
    perimetro = "Calcolo...";

    if (isNaN(diag1Val) || isNaN(diag2Val) || isNaN(latoVal) || 
        diag1Val <= 0 || diag2Val <= 0 || latoVal <= 0) {
      alert("⚠️ Inserisci valori numerici positivi per tutte le diagonali e il lato.");
      area = "N/A";
      perimetro = "N/A";
      return;
    }

    try {
      // Chiamate a Tauri per il rombo
      area = await invoke("calcola_area_rombo", { 
        diagonale1: diag1Val, 
        diagonale2: diag2Val 
      });
      perimetro = await invoke("calcola_perimetro_rombo", { 
        lato: latoVal 
      });
      
    } catch (e) {
      console.error("Errore durante l'invocazione di Tauri:", e);
      area = "Errore ❌";
      perimetro = "Errore ❌";
    }
  }

  async function calcolaRombo(event: Event) {
    event.preventDefault();
    calcolaAutomaticamente();
  }
</script>

<div class="calculator-card">
  <h2 class="calculator-title">Calcolo Rombo</h2>
  <p class="calculator-subtitle">Calcola Area e Perimetro utilizzando la libreria Rust ileana-lib.</p>

  <div class="calculator-layout">
    <!-- Colonna sinistra: Input -->
    <div class="input-column">
      <form class="row" onsubmit={calcolaRombo}>
        <div class="input-group">
          <label for="diagonale1-input" class="input-label">Diagonale 1:</label>
          <input
            id="diagonale1-input"
            type="number"
            step="any"
            placeholder="Diagonale 1..."
            bind:value={diagonale1}
            class="input-box"
            min="0"
            oninput={e => e.currentTarget.value = Math.max(0, parseFloat(e.currentTarget.value) || 0).toString()}
          />
        </div>

        <div class="input-group">
          <label for="diagonale2-input" class="input-label">Diagonale 2:</label>
          <input
            id="diagonale2-input"
            type="number"
            step="any"
            placeholder="Diagonale 2..."
            bind:value={diagonale2}
            class="input-box"
            min="0"
            oninput={e => e.currentTarget.value = Math.max(0, parseFloat(e.currentTarget.value) || 0).toString()}
          />
        </div>

        <div class="input-group">
          <label for="lato-input" class="input-label">Lato:</label>
          <input
            id="lato-input"
            type="number"
            step="any"
            placeholder="Lato..."
            bind:value={lato}
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
      {#if typeof area === "number" && area > 0 && lato > 0}
        <div class="rhombus-container">
          <svg viewBox="0 0 200 220" class="rhombus-svg" style={ `--rhombus-color: ${rhombusColor}; --text-color: ${textColor};` }>
            <g transform="translate(100, 100)">
              <!-- Rombo - usiamo un path per creare la forma -->
              <path
                d={`M 0,${-half_diag2} L ${half_diag1},0 L 0,${half_diag2} L ${-half_diag1},0 Z`}
                class="rhombus-border"
              />

              <text x="0" y="-12" class="area-label" style={`font-size: ${font_size}px`}>Area:</text>
              <text x="0" y="4" class="area-value" style={`font-size: ${font_size}px`}>{formatResult(area)}</text>

              <text x="0" y={perimeter_y} class="perimeter-text" style={`font-size: ${font_size * 0.8}px`}>
                Perimetro: {formatResult(perimetro)}
              </text>

              <text x="0" y={diag1_label_y} class="diagonal-label" style={`font-size: ${font_size * 0.8}px`}>
                Diagonale 1: {diagonale1.toFixed(2)}
              </text>
              
              <g transform="rotate(-90)">
                <text x="0" y={-half_diag2 + diag2_label_offset} class="diagonal-label" style={`font-size: ${font_size * 0.8}px`}>
                  Diagonale 2: {diagonale2.toFixed(2)}
                </text>
              </g>
            </g>
          </svg>
        </div>
      {:else}
        <div class="rhombus-placeholder">
          <p>Inserisci valori validi per visualizzare il rombo</p>
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

  /* Stile per l'etichetta del testo "Diagonale 1:", "Diagonale 2:", ecc. */
  .input-label {
    white-space: nowrap;
    min-width: 100px;
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

  .rhombus-container {
    width: 200px;
    height: 220px;
  }

  .rhombus-svg {
    width: 100%;
    height: 100%;
    font-family: Arial, sans-serif;
    margin: auto;
  }

  /* Stili per gli elementi SVG - ora gestiti via CSS */
  .rhombus-border {
    fill: none;
    stroke: var(--rhombus-color, #FF9800);
    stroke-width: 2;
    transition: stroke 0.3s ease;
  }

  .area-label {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #E65100);
    font-weight: bold;
  }

  .area-value {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #E65100);
    font-weight: bold;
  }

  .perimeter-text {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #E65100);
  }

  .diagonal-label {
    text-anchor: middle;
    fill: var(--text-color, #E65100);
  }

  .rhombus-placeholder {
    width: 200px;
    height: 200px;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 1px dashed #ccc;
    color: #666;
  }
</style>