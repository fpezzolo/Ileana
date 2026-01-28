# Struttura del Progetto Ileana

Questo documento descrive l'organizzazione del progetto Ileana, un'applicazione che combina Rust, Svelte e Tauri per creare un'applicazione desktop con funzionalità geometriche.

## Panoramica del Progetto

Ileana è un workspace Rust che include:
- Una libreria Rust per operazioni geometriche (`ileana-lib`)
- Un'interfaccia a riga di comando (`ileana-cli`)
- Supporto WebAssembly (`ileana-wasm`)
- Un'applicazione desktop Svelte/Tauri (`ileana-app`)

## Struttura delle Cartelle

```
Ileana/
├── ileana-lib/          # Libreria Rust principale
├── ileana-cli/          # Interfaccia a riga di comando
├── ileana-wasm/         # Supporto WebAssembly
├── ileana-app/          # Applicazione Svelte/Tauri
├── scripts/             # Script di utilità
├── Varie/               # File vari
└── [file di configurazione root]
```

## Componenti Dettagliati

### 1. ileana-lib (Libreria Rust)

**Posizione**: `ileana-lib/`

**Scopo**: Contiene la logica principale per le operazioni geometriche.

**File principali**:
- `src/lib.rs` - Punto di ingresso della libreria
- `src/geometria.rs` - Funzioni geometriche generiche
- `src/quadrato.rs` - Implementazione della forma quadrato
- `src/rettangolo.rs` - Implementazione della forma rettangolo
- `src/triangolo.rs` - Implementazione della forma triangolo
- `src/cerchio.rs` - Implementazione della forma cerchio
- `src/rombo.rs` - Implementazione della forma rombo
- `src/logo.rs` - Funzionalità per il logo

**Dipendenze**: Definite in `ileana-lib/Cargo.toml`

### 2. ileana-cli (Interfaccia a Riga di Comando)

**Posizione**: `ileana-cli/`

**Scopo**: Fornisce un'interfaccia a riga di comando per utilizzare la libreria.

**File principali**:
- `src/main.rs` - Punto di ingresso del CLI
- `Cargo.toml` - Dipendenze specifiche per il CLI

### 3. ileana-wasm (WebAssembly)

**Posizione**: `ileana-wasm/`

**Scopo**: Permette l'esecuzione della libreria nel browser tramite WebAssembly.

**File principali**:
- `src/lib.rs` - Punto di ingresso per WASM
- `index.html` - Pagina HTML di esempio
- `Cargo.toml` - Configurazione specifica per WASM

### 4. ileana-app (Applicazione Svelte/Tauri)

**Posizione**: `ileana-app/`

**Scopo**: Applicazione desktop con interfaccia utente Svelte e backend Tauri.

**Struttura interna**:

#### Configurazione principale
- `package.json` - Dipendenze Node.js
- `vite.config.js` - Configurazione Vite
- `svelte.config.js` - Configurazione Svelte
- `tailwind.config.js` - Configurazione Tailwind CSS

#### Codice Svelte (src/)
- `app.css` - Stili globali
- `app.html` - Template HTML principale
- `routes/` - Routing dell'applicazione
  - `+layout.svelte` - Layout principale
  - `+layout.server.ts` - Logica server per il layout
  - `+page.svelte` - Pagina principale
- `components/` - Componenti riutilizzabili
  - `Card.svelte` - Componente card
  - `Quadrato.svelte` - Componente quadrato

#### Configurazione Tauri (src-tauri/)
- `tauri.conf.json` - Configurazione Tauri
- `Cargo.toml` - Dipendenze Rust per Tauri
- `src/main.rs` - Punto di ingresso Tauri
- `src/lib.rs` - Logica aggiuntiva Rust
- `icons/` - Icone dell'applicazione

#### Asset statici (static/)
- `favicon.png` - Icona del browser
- `svelte.svg`, `tauri.svg`, `vite.svg` - Loghi

#### Output di build (build/)
- Contiene i file generati dalla build

### 5. Script di Utilità (scripts/)

**Scopo**: Automatizzare vari processi di sviluppo.

**Script disponibili**:
- `build-dev.sh` - Build in modalità sviluppo
- `build-release.sh` - Build in modalità release
- `doc.sh` - Generazione documentazione
- `lint.sh` - Linting del codice
- `lint-test.sh` - Linting e test
- `aggiorna-rust.sh` - Aggiornamento Rust
- `avvia-prova-webassembly.sh` - Avvio prova WASM

### 6. File Vari (Varie/)

**Scopo**: Contiene file vari non direttamente legati al codice.

**Contenuto**:
- `pubblicita.odt` - Materiale pubblicitario
- `qrbig.PNG`, `qrsmall.PNG` - Codici QR

## Tecnologie Chiave

1. **Rust** - Linguaggio principale per la libreria e Tauri
2. **Svelte** - Framework frontend per l'interfaccia utente
3. **Tauri** - Framework per creare applicazioni desktop
4. **Tailwind CSS** - Framework CSS per lo stile
5. **Vite** - Build tool per il frontend
6. **WebAssembly** - Per l'esecuzione nel browser
7. **Node.js** - Per la gestione delle dipendenze frontend

## Dipendenze Principali

### ileana-lib (Libreria Rust)
- **Dipendenze**: Nessuna dipendenza esterna (pura Rust)
- **Versione Rust**: 2024 Edition
- **Licenza**: GPL-3.0

### ileana-cli (Interfaccia a Riga di Comando)
- **Dipendenze principali**:
  - `ileana-lib` (locale): `{ path = "../ileana-lib" }`
- **Scopo**: Fornisce accesso CLI alle funzionalità geometriche

### ileana-wasm (WebAssembly)
- **Dipendenze principali**:
  - `wasm-bindgen = "0.2"` - Per l'interoperabilità con JavaScript
  - `ileana-lib` (locale): `{ path = "../ileana-lib" }`
- **Configurazione**: `crate-type = ["cdylib"]` per la generazione WASM

### ileana-app (Applicazione Svelte/Tauri)

#### Dipendenze Rust (Tauri)
- `tauri = { version = "2" }` - Framework desktop
- `tauri-plugin-opener = "2"` - Plugin per apertura file
- `serde = { version = "1", features = ["derive"] }` - Serializzazione
- `serde_json = "1"` - Supporto JSON
- `ileana-lib` (locale): `{ path = "../../ileana-lib" }`
- `tracing = "0.1"` - Logging avanzato
- `tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }`
- `tracing-appender = "0.2"` - Logging su file
- `image = "0.24"` - Manipolazione immagini
- `base64 = "0.21"` - Codifica/decodifica base64

#### Dipendenze Frontend (Node.js)
- `@sveltejs/kit = "^2.9.0"` - Framework Svelte
- `svelte = "^5.0.0"` - Linguaggio Svelte
- `@tauri-apps/api = "^2.9.1"` - API Tauri per frontend
- `tailwindcss = "^4.1.18"` - Framework CSS
- `vite = "^7.2.6"` - Build tool
- `typescript = "^5.9.3"` - Tipizzazione statica

## Processo di Build Dettagliato

### Build di Sviluppo

Per compilare l'intero workspace in modalità sviluppo:

```bash
# Compilazione completa del workspace
cargo build

# Compilazione WebAssembly per sviluppo
cd ileana-wasm
wasm-pack build --target web
cd ..
```

Oppure utilizzare lo script fornito:
```bash
./scripts/build-dev.sh
```

### Build di Produzione

Per compilare in modalità ottimizzata per il rilascio:

```bash
# Compilazione completa del workspace in release
cargo build --release

# Compilazione WebAssembly ottimizzata
cd ileana-wasm
wasm-pack build --release --target web
cd ..
```

Oppure utilizzare lo script fornito:
```bash
./scripts/build-release.sh
```

### Esecuzione dell'Applicazione Desktop

Per avviare l'applicazione Tauri in modalità sviluppo:

```bash
cd ileana-app
npm run tauri dev
```

Per build di produzione:
```bash
cd ileana-app
npm run tauri build
```

### Test WebAssembly

Per testare il modulo WebAssembly:

```bash
./scripts/avvia-prova-webassembly.sh
```

Questo script:
1. Compila il modulo WASM
2. Avvia un server HTTP locale sulla porta 4000
3. Apre automaticamente il browser sulla pagina di test

## Integrazione tra Componenti

### Flusso di Dati

```
ileana-lib (Rust)
     │
     ├── ileana-cli (Rust CLI)
     ├── ileana-wasm (WebAssembly)
     └── ileana-app (Tauri)
            │
            ├── src-tauri (Backend Rust)
            │      └── Integra ileana-lib direttamente
            │
            └── src (Frontend Svelte)
                   └── Componenti UI che chiamano Tauri
```

### Integrazione Tauri-Svelte

1. **Backend Rust** (`src-tauri/src/lib.rs`):
   - Importa direttamente `ileana-lib` come dipendenza locale
   - Espone funzioni Rust tramite Tauri commands
   - Gestisce la logica geometrica e la serializzazione JSON

2. **Frontend Svelte** (`src/routes/+page.svelte`):
   - Importa componenti geometrici (Quadrato, Rettangolo, Cerchio, Triangolo)
   - Utilizza Tauri API per comunicare con il backend
   - Visualizza i risultati in interfaccia utente reattiva

3. **Comunicazione**:
   - Tauri fornisce un bridge tra Rust e JavaScript
   - I dati vengono serializzati/deserializzati automaticamente
   - Supporto per chiamate asincrone e gestione errori

### Esempio di Integrazione

Nel file `src-tauri/src/lib.rs`:

```rust
use ileana_lib::quadrato::Quadrato;
use ileana_lib::rettangolo::Rettangolo;
// ... altre importazioni

// Funzioni che utilizzano direttamente la libreria
#[tauri::command]
fn calcola_area_quadrato(lato: f64) -> f64 {
    Quadrato::new(lato).area()
}
```

Nel frontend Svelte:

```svelte
<script>
import { invoke } from '@tauri-apps/api/tauri';

async function calcolaArea(lato) {
    const area = await invoke('calcola_area_quadrato', { lato });
    return area;
}
</script>
```

## Dipendenze del Workspace

Il file `Cargo.toml` nella root definisce il workspace e include:
```toml
[workspace]
members = [
    "ileana-lib",
    "ileana-cli",
    "ileana-wasm",
    "ileana-app/src-tauri"
]
resolver = "2"
```

## Esempi di Utilizzo

### Utilizzo della Libreria Rust

```rust
// Esempio di utilizzo diretto della libreria
use ileana_lib::quadrato::Quadrato;
use ileana_lib::rettangolo::Rettangolo;
use ileana_lib::cerchio::Cerchio;

fn main() {
    // Creazione e calcoli con Quadrato
    let quadrato = Quadrato::new(5.0);
    println!("Area quadrato: {}", quadrato.area());
    println!("Perimetro quadrato: {}", quadrato.perimetro());

    // Creazione e calcoli con Rettangolo
    let rettangolo = Rettangolo::new(4.0, 6.0);
    println!("Area rettangolo: {}", rettangolo.area());
    println!("Perimetro rettangolo: {}", rettangolo.perimetro());

    // Creazione e calcoli con Cerchio
    let cerchio = Cerchio::new(3.0);
    println!("Area cerchio: {}", cerchio.area());
    println!("Perimetro cerchio: {}", cerchio.perimetro());
}
```

### Utilizzo tramite CLI

```bash
# Compilare ed eseguire il CLI
cargo run -p ileana-cli

# Esempio di output:
# Area del quadrato con lato 5.0: 25.0
# Perimetro del quadrato con lato 5.0: 20.0
# Area del rettangolo 4.0x6.0: 24.0
# Perimetro del rettangolo 4.0x6.0: 20.0
# Area del cerchio con raggio 3.0: 28.274333882308138
# Perimetro del cerchio con raggio 3.0: 18.84955592153876
```

### Utilizzo nell'Applicazione Desktop

L'applicazione desktop fornisce un'interfaccia grafica per:
- Visualizzare forme geometriche
- Calcolare aree e perimetri
- Esportare risultati
- Catturare screenshot delle visualizzazioni

### Utilizzo WebAssembly

```html
<!-- Esempio di utilizzo in pagina HTML -->
<script type="module">
  import init, { calcola_area_quadrato } from './ileana_wasm.js';

  async function run() {
    await init();
    
    const lato = 5.0;
    const area = calcola_area_quadrato(lato);
    console.log(`Area del quadrato con lato ${lato}: ${area}`);
    
    document.getElementById('result').textContent = 
      `Area: ${area.toFixed(2)}`;
  }

  run();
</script>

<div id="result"></div>
```

## Come Aggiungere Nuove Funzionalità

1. **Per la libreria Rust**: Aggiungere nuovi moduli in `ileana-lib/src/`
2. **Per l'interfaccia utente**: Aggiungere nuovi componenti in `ileana-app/src/components/`
3. **Per nuove route**: Aggiungere nuovi file in `ileana-app/src/routes/`
4. **Per nuove dipendenze Rust**: Modificare il `Cargo.toml` appropriato
5. **Per nuove dipendenze frontend**: Modificare `ileana-app/package.json`

## Convenzioni di Nomenclatura

- I file Rust seguono la convenzione snake_case
- I componenti Svelte usano PascalCase
- Gli script usano kebab-case
- I file di configurazione usano snake_case o kebab-case a seconda della tecnologia

## Ambiente di Sviluppo

- **VSCode**: Configurazione workspace in `.vscode/`
- **Estensioni consigliate**: Rust Analyzer, Svelte, Tailwind CSS IntelliSense
- **Task**: Definiti in `.vscode/tasks.json`
- **Launch**: Configurazioni in `.vscode/launch.json`

## Configurazione dell'Ambiente di Sviluppo

### Prerequisiti

1. **Rust**: Versione 1.88.0 o superiore
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Node.js**: Versione 18.x o superiore
   ```bash
   # Utilizzare nvm per l'installazione
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.7/install.sh | bash
   nvm install 18
   ```

3. **Strumenti aggiuntivi**:
   ```bash
   # wasm-pack per WebAssembly
   cargo install wasm-pack
   
   # basic-http-server per test WebAssembly
   cargo install basic-http-server
   ```

### Configurazione del Progetto

1. **Clonare il repository**:
   ```bash
   git clone https://github.com/fpezzolo/Ileana.git
   cd Ileana
   ```

2. **Installare dipendenze frontend**:
   ```bash
   cd ileana-app
   npm install
   cd ..
   ```

3. **Configurare VSCode** (opzionale ma consigliato):
   - Installare estensioni: Rust Analyzer, Svelte, Tailwind CSS IntelliSense
   - Aprire il workspace: `ileana.code-workspace`

### Comandi Utili

- **Verifica linting**: `./scripts/lint.sh`
- **Esecuzione test**: `./scripts/lint-test.sh`
- **Generazione documentazione**: `./scripts/doc.sh`
- **Aggiornamento Rust**: `./scripts/aggiorna-rust.sh`

## Processo di Sviluppo Tipico

1. Implementare la logica in `ileana-lib`
2. Testare tramite `ileana-cli` o `ileana-wasm`
3. Integrare nell'interfaccia utente in `ileana-app`
4. Costruire l'applicazione desktop con Tauri
5. Eseguire test e linting con gli script forniti

## Documentazione Correlata

Per approfondire specifici aspetti del progetto:

- **[README.md](README.md)**: Panoramica generale e informazioni di base
- **[DOCUMENTATION_SUMMARY.md](DOCUMENTATION_SUMMARY.md)**: Qualità e statistiche della documentazione Rust
- **[RUST_DOCUMENTATION.md](RUST_DOCUMENTATION.md)**: Guida pratica all'utilizzo della documentazione Rust

## Note Importanti

- Il progetto usa un approccio modulare con separazione chiara tra logica e interfaccia
- La libreria Rust è progettata per essere riutilizzabile in diversi contesti
- L'applicazione desktop combina il meglio di Rust (performance) e Svelte (UI reattiva)
- Gli script di utilità semplificano le operazioni comuni di sviluppo
- **Tutta la documentazione è interconnessa** per evitare ridondanze e fornire informazioni complete
