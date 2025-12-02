<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // Usiamo il tipo generico 'number | string' per includere sia il risultato che gli stati 'N/A'/'Errore'.
  let lato = $state(0);
  let area = $state<number | string>("N/A");
  let perimetro = $state<number | string>("N/A");

  /**
   * Formatta il risultato numerico a due decimali o restituisce lo stato stringa.
   * @param value Il valore di Area o Perimetro.
   * @returns Il valore formattato come stringa.
   */
  function formatResult(value: number | string): string {
    if (typeof value === 'number') {
      return value.toFixed(2);
    }
    return value;
  }

  async function calcolaQuadrato(event: Event) {
    event.preventDefault();
    
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
      area = 'Errore ❌';
      perimetro = 'Errore ❌';
    }
  }
</script>

<div class="calculator-card">
  <h2>Calcolo Quadrato</h2>
  <p>Calcola Area e Perimetro utilizzando la libreria Rust ileana-lib.</p>

  <form class="row" onsubmit={calcolaQuadrato}>
    <input 
      id="lato-input" 
      type="number" 
      step="any" 
      placeholder="Lato..." 
      bind:value={lato} 
    />
    <button type="submit">Calcola</button>
  </form>

  <div class="row results-display">
    <p>Area: <strong>{formatResult(area)}</strong></p>
    <p>Perimetro: <strong>{formatResult(perimetro)}</strong></p>
  </div>
 
</div>

<style>
  /* Stili per il componente auto-contenuto */
  .calculator-card {
    border: 1px solid #e0e0e0;
    border-radius: 8px;
    padding: 20px;
    margin: 20px auto;
    max-width: 400px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
    text-align: center;
  }
  .row {
    display: flex;
    justify-content: center;
    gap: 10px;
  }
  .results-display {
    margin-top: 20px;
    gap: 40px;
    font-size: 1.1em;
  }
</style>