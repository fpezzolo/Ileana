# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).


    // Avvia il server di sviluppo Vite per il frontend SvelteKit.
    // L'output è servito sulla porta 1420 (come da vite.config.js) 
    // ed è richiesto dal comando 'tauri:dev'.
    "dev": "vite dev",

    // Compila il frontend SvelteKit in una build statica (SPA)
    // utilizzando l'adapter statico, generando i file finali nella cartella 'build'.
    "build": "vite build",

    // Avvia un server locale per visualizzare l'anteprima della build compilata (dopo 'build').
    // Utile per testare la build finale prima del bundling Tauri.
    "preview": "vite preview",

    // Sincronizza i tipi di SvelteKit (per intellisense) e verifica la correttezza del codice TypeScript
    // e Svelte (componenti, markup, ecc.)
    "check": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json",
    
    // Esegue la stessa verifica dei tipi/codice, ma rimane in ascolto (watch) per i cambiamenti dei file.
    "check:watch": "svelte-kit sync && svelte-check --tsconfig ./tsconfig.json --watch",

    // Wrapper del CLI di Tauri installato localmente (consigliato). 
    // Permette di eseguire comandi generici come 'tauri info' o 'tauri completions'.
    "tauri": "tauri",

    // Compila e avvia l'applicazione Rust, che a sua volta esegue 'npm run dev' 
    // per iniettare il frontend SvelteKit. Questo è il comando principale per lo sviluppo.
    "tauri:dev": "tauri dev",

    // Compila l'applicazione Rust per la produzione, includendo il frontend statico 
    // generato da 'npm run build' (eseguito automaticamente prima del bundling finale).
    "tauri:build": "tauri build"