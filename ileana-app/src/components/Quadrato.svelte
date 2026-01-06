<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  /**
   * Props per la personalizzazione del componente Quadrato
   *
   * @prop {string} squareColor - Colore del bordo del quadrato (default: "#4CAF50")
   * @prop {string} textColor - Colore del testo all'interno del quadrato (default: "#4CAF50")
   *
   * Esempi di utilizzo:
   * <Quadrato /> - Utilizza i colori predefiniti
   * <Quadrato squareColor="#FF5722" textColor="#E64A19" /> - Quadrato arancione
   * <Quadrato squareColor="#2196F3" textColor="#0D47A1" /> - Quadrato blu
   */
  const {
    squareColor = "#4CAF50",
    textColor = "#4CAF50"
  } = $props();

  // Usiamo il tipo generico 'number | string' per includere sia il risultato che gli stati 'N/A'/'Errore'.
  let lato = $state(0);
  let area = $state<number | string>("N/A");
  let perimetro = $state<number | string>("N/A");

  const MAX_SVG_LATO = 170; // Lato massimo del quadrato SVG
  const MAX_LATO_VALUE = 170; // Valore massimo accettabile per l'input lato (ad esempio, 100 unità)
  const MIN_SVG_LATO = 130; // <--- NUOVA COSTANTE: Lato minimo per evitare che il quadrato sparisca

  // Calcolo dinamico e REATTIVO di lato_quadrato_px
  const lato_quadrato_px = $derived(
    typeof lato === "number" && lato > 0
      ? Math.max(
          (Math.min(lato, MAX_LATO_VALUE) / MAX_LATO_VALUE) * MAX_SVG_LATO,
          MIN_SVG_LATO, // <-- Applica il limite minimo
        )
      : 10,
  );

  // Calcoli reattivi: raggio e margine_testo dipendono da lato_quadrato_px
  const raggio = $derived(lato_quadrato_px / 2);
  const margine_testo = $derived(raggio + 10);

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

  // Effetto reattivo per calcolare automaticamente quando il lato cambia
  $effect(() => {
    if (typeof lato === "number" && lato > 0) {
      calcolaAutomaticamente();
    } else if (lato === 0) {
      // Reset quando il valore viene azzerato
      area = "N/A";
      perimetro = "N/A";
    }
  });

  async function calcolaAutomaticamente() {
    // Convalida e conversione dell'input
    const latoVal = parseFloat(lato.toString());

    // Feedback immediato
    area = "Calcolo...";
    perimetro = "Calcolo...";

    if (isNaN(latoVal) || latoVal <= 0) {
      alert("⚠️ Inserisci un valore numerico positivo per il lato.");
      area = "N/A";
      perimetro = "N/A";
      return;
    }

    try {
      // Chiamate a Tauri
      area = await invoke("calcola_area_quadrato", { lato: latoVal });
      perimetro = await invoke("calcola_perimetro_quadrato", { lato: latoVal });
    } catch (e) {
      console.error("Errore durante l'invocazione di Tauri:", e);
      area = "Errore ❌";
      perimetro = "Errore ❌";
    }
  }

  async function calcolaQuadrato(event: Event) {
    event.preventDefault();
    calcolaAutomaticamente();
  }
</script>

<div class="calculator-card">
  <h2 class="calculator-title">Calcolo Quadrato</h2>
  <p class="calculator-subtitle">Calcola Area e Perimetro utilizzando la libreria Rust ileana-lib.</p>

  <div class="calculator-layout">
    <!-- Colonna sinistra: Input -->
    <div class="input-column">
      <form class="row" onsubmit={calcolaQuadrato}>
        <label for="lato-input" class="input-label">Lato:</label>

        <input
          id="lato-input"
          type="number"
          step="any"
          placeholder="Lato..."
          bind:value={lato}
          class="lato-input-box"
          min="0"
          oninput={e => e.currentTarget.value = Math.max(0, parseFloat(e.currentTarget.value) || 0).toString()}
        />

        <button type="submit">Calcola</button>
      </form>

      <div class="text-results">
        <p>Area: <strong>{formatResult(area)}</strong></p>
        <p>Perimetro: <strong>{formatResult(perimetro)}</strong></p>
      </div>
    </div>

    <!-- Colonna destra: Rappresentazione grafica -->

    <div class="graphic-column">
      {#if typeof area === "number" && lato > 0}
        <div class="square-container">
          <svg viewBox="0 0 200 200" class="square-svg" style={ `--square-color: ${squareColor}; --text-color: ${textColor};` }>
            <g transform="translate(100, 100)">
              <rect
                x={-raggio}
                y={-raggio}
                width={lato_quadrato_px}
                height={lato_quadrato_px}
                class="square-border"
              />

              <text x="0" y="-6" class="area-label">Area:</text>
              <text x="0" y="6" class="area-value">{formatResult(area)}</text>

              <text x="0" y={+raggio + 10} class="perimeter-text">
                Perimetro: {formatResult(perimetro)}
              </text>

              <text x="0" y={-raggio + 12} class="side-label side-b">
                Lato B: {lato.toFixed(2)}
              </text>

              <g transform="rotate(-90)">
                <text x="0" y={-raggio + 10} class="side-label side-a">
                  Lato A: {lato.toFixed(2)}
                </text>
              </g>
            </g>
          </svg>
        </div>
      {:else}
        <div class="square-placeholder">
          <p>Inserisci un valore valido per visualizzare il quadrato</p>
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
    font-size: 1.5rem; /* Dimensione ridotta rispetto all'h2 standard */
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

/* Stile per l'etichetta del testo "Lato:" */
.input-label {
    align-self: center; /* Allinea verticalmente il testo nel flex container */
    white-space: nowrap;
}

/* Stile per accorciare il campo di input */
.lato-input-box {
    width: 100px; /* Imposta una larghezza fissa più corta (es. 100px) */
    max-width: 100px;
}

  .text-results {
    margin-top: 20px;
    gap: 40px;
    font-size: 1.1em;
  }

  .info-message {
    margin-top: 15px;
    font-size: 0.9em;
    color: #666;
    font-style: italic;
    text-align: center;
  }

  .info-message p {
    margin: 5px 0;
    padding: 8px;
    background-color: rgba(76, 175, 80, 0.1);
    border-left: 3px solid #4caf50;
    border-radius: 4px;
  }

  .square-container {
    width: 200px;
    height: 200px;
  }

  .square-svg {
    width: 100%;
    height: 100%;
    font-family: Arial, sans-serif;
    margin: auto;
  }

  /* Stili per gli elementi SVG - ora gestiti via CSS */
  .square-border {
    fill: none;
    stroke: var(--square-color, #4CAF50);
    stroke-width: 2;
    transition: stroke 0.3s ease;
  }

  .area-label {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #4CAF50);
    font-weight: bold;
    font-size: 14px;
  }

  .area-value {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #4CAF50);
    font-weight: bold;
    font-size: 14px;
  }

  .perimeter-text {
    text-anchor: middle;
    dominant-baseline: middle;
    fill: var(--text-color, #4CAF50);
    font-size: 12px;
  }

  .side-label {
    text-anchor: middle;
    fill: var(--text-color, #4CAF50);
    font-size: 12px;
  }

  .side-a {
    dominant-baseline: central;
  }

  .side-b {
    text-anchor: middle;
  }

  .square-placeholder {
    width: 200px;
    height: 200px;
    display: flex;
    justify-content: center;
    align-items: center;
    border: 1px dashed #ccc;
    color: #666;
  }
</style>
