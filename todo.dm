# TODO Ileana

## Piano di miglioramento

1. Rendere `ileana-lib` il vero centro del dominio.
   Introdurre costruttori sicuri `new(...) -> Result<..., ErroreGeometrico>`, campi privati dove serve e validazioni reali per misure negative o incoerenti, soprattutto per triangolo e rombo.

2. Uniformare i contratti tra libreria, CLI, WASM e Tauri.
   Fare in modo che tutti i front-end consumino gli stessi risultati e gli stessi errori, evitando che CLI, WASM e backend Tauri reinventino validazioni o convenzioni diverse.

3. Semplificare il backend Tauri.
   Sostituire i molti command quasi identici con una struttura piu compatta e tipizzata, ad esempio input/output serializzabili condivisi e `Result` espliciti al posto di soli `f64`.

4. Estrarre la logica comune dei componenti Svelte.
   Creare utility condivise per invocazione Tauri, stato `idle/loading/success/error`, formattazione numeri e validazione input, cosi i componenti delle figure restano focalizzati solo su UI e visualizzazione SVG.

5. Migliorare UX e accessibilita della UI.
   Rimuovere gli `alert()`, mostrare errori inline, scegliere un solo modello di interazione tra calcolo automatico e submit, e rendere piu coerenti etichette, placeholder e feedback all'utente.

6. Mettere in ordine sicurezza e packaging Tauri.
   Eliminare il caricamento runtime da CDN per gli screenshot, impostare una CSP non nulla, rimuovere dipendenze Rust inutilizzate e verificare che la configurazione Tauri sia minima ma corretta.

7. Completare il lato didattico e la copertura.
   Estendere `ileana-wasm` a tutte le figure, sostituire i test segnaposto con test reali, e aggiornare README e documentazione architetturale in base al nuovo flusso.

## Ordine consigliato

`ileana-lib` -> Tauri/CLI/WASM -> Svelte condiviso -> UX -> sicurezza/configurazione -> documentazione
