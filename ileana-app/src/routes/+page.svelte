<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  // Variabili di stato reattive per input e risultati
  let lato = $state(0);
  let area = $state<number | string>("N/A");
  let perimetro = $state<number | string>("N/A");

  async function calcolaQuadrato(event: Event) {
    event.preventDefault();
    
    // Convalida e conversione dell'input
    const latoVal = parseFloat(lato.toString());

    if (isNaN(latoVal) || latoVal <= 0) {
      alert("Inserisci un valore numerico positivo per il lato.");
      area = "N/A";
      perimetro = "N/A";
      return;
    }

    try {
      // 1. Chiamata al Command Rust per l'Area
      area = await invoke("calcola_area_quadrato", { lato: latoVal });

      // 2. Chiamata al Command Rust per il Perimetro
      perimetro = await invoke("calcola_perimetro_quadrato", { lato: latoVal });

    } catch (e) {
      console.error("Errore durante l'invocazione di Tauri:", e);
      area = 'Errore';
      perimetro = 'Errore';
    }
  }
</script>

<main class="container">
  <h1>Geometria Ileana App - Quadrato</h1>
  
  <div class="row">
    <a href="https://vite.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte-kit" alt="SvelteKit Logo" />
    </a>
  </div>
  <p>Calcola Area e Perimetro utilizzando la libreria Rust ileana-lib.</p>

  <form class="row" onsubmit={calcolaQuadrato}>
    <input 
      id="lato-input" 
      type="number" 
      step="any" 
      placeholder="Inserisci la lunghezza del lato..." 
      bind:value={lato} 
    />
    <button type="submit">Calcola</button>
  </form>

  <div class="row" style="margin-top: 20px; gap: 40px;">
    <p>Area: <strong>{typeof area === 'number' ? area.toFixed(2) : area}</strong></p>
    <p>Perimetro: <strong>{typeof perimetro === 'number' ? perimetro.toFixed(2) : perimetro}</strong></p>
  </div>
</main>


<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
