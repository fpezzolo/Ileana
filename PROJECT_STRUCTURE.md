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

## Flusso di Build

1. La libreria Rust (`ileana-lib`) contiene la logica principale
2. L'applicazione Svelte (`ileana-app`) usa la libreria tramite Tauri
3. Tauri collega il frontend Svelte con il backend Rust
4. Gli script nella cartella `scripts/` automatizzano vari processi

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

## Processo di Sviluppo Tipico

1. Implementare la logica in `ileana-lib`
2. Testare tramite `ileana-cli` o `ileana-wasm`
3. Integrare nell'interfaccia utente in `ileana-app`
4. Costruire l'applicazione desktop con Tauri
5. Eseguire test e linting con gli script forniti

## Note Importanti

- Il progetto usa un approccio modulare con separazione chiara tra logica e interfaccia
- La libreria Rust è progettata per essere riutilizzabile in diversi contesti
- L'applicazione desktop combina il meglio di Rust (performance) e Svelte (UI reattiva)
- Gli script di utilità semplificano le operazioni comuni di sviluppo
