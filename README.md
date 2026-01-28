# Ileana
Programma didattico in Rust I.l.e.a.n.a. Interessante Lavoro Elabora Aree Nonchè Altro

**Versione Beta**

## 📖 Panoramica

Ileana è un progetto didattico che combina Rust, Svelte e Tauri per creare un'applicazione desktop con funzionalità geometriche. Il progetto include una libreria Rust per operazioni geometriche, un'interfaccia a riga di comando, supporto WebAssembly e un'applicazione desktop moderna.

## 🚀 Iniziare

Per informazioni dettagliate su:
- **Struttura del progetto**: Consultare [`PROJECT_STRUCTURE.md`](PROJECT_STRUCTURE.md)
- **Documentazione Rust**: Consultare [`RUST_DOCUMENTATION.md`](RUST_DOCUMENTATION.md)
- **Riassunto documentazione**: Consultare [`DOCUMENTATION_SUMMARY.md`](DOCUMENTATION_SUMMARY.md)

### Requisiti Minimi

- **Rust**: Versione 1.88.0 o superiore
- **Node.js**: Versione 18.x o superiore (per l'applicazione desktop)

### Compilazione Rapida

Per compilare ed eseguire rapidamente il CLI:

```shell
cargo run -p ileana-cli
```

## log attivita

### 26/11/2025

comando per sotoppore ad AI la cartella di un progetto
copia nella clip l'albero del progetto ripulito.
```shell
tree -L 3 -I "node_modules|target|dist|pkg|.*" | xclip -selection clipboard
```

Scheletro funzionante dell'app tauri


### 11/2025

installato il pacchetto tauri
```shell
npm create tauri-app
```

creata la app tauri con template svelt 

```shell

Template created! To get started run:
  cd ileana-app
  npm install
  npm run tauri android init

For Desktop development, run:
  npm run tauri dev

For Android development, run:
  npm run tauri android dev

```
### 09/2025

script utility multi piattaforma per linux / windows
nota bene serve abilitare i file di script per l'esecuzione in ambiente linux

ricordarsi di installare basic-http-server con il comando:
```shell
cargo install basic-http-server
```
nel caso non funzionasse lo script Prova WebAssembly


### 07/2025

creata libreria web assembly e pagina di prova.
adatta la libreria base al modello web assembly 

ulteriori script di comodo
debug binario

introdotti script di utility nella cartella scripts
introdotta la base per i test automatici 


## ⚙️ Script di Qualità

Il progetto include script per linting, test e documentazione nella cartella [`scripts/`](scripts/).

### Script Principali

- **Linting**: `./scripts/lint.sh` (o `.ps1` per Windows)
- **Test**: `./scripts/lint-test.sh`
- **Documentazione**: `./scripts/doc.sh`
- **Build**: `./scripts/build-dev.sh` e `./scripts/build-release.sh`

Per dettagli completi su tutti gli script disponibili, consultare [`PROJECT_STRUCTURE.md`](PROJECT_STRUCTURE.md#script-di-utilità).

### 05/2025
un cambio di architettura con la divisione del programma in ileana-cli ed ileana-lib 


## 📄 Licenza

Questo progetto è distribuito sotto licenza [GPL-3.0](LICENSE).

## 📚 Documentazione Completa

Per approfondire:
- **[Struttura del Progetto](PROJECT_STRUCTURE.md)**: Architettura, dipendenze, build process
- **[Documentazione Rust](RUST_DOCUMENTATION.md)**: Dettagli tecnici sulla documentazione Rust
- **[Riassunto Documentazione](DOCUMENTATION_SUMMARY.md)**: Statistiche e qualità della documentazione

## 🤝 Contribuire

Le contribuzioni sono benvenute! Per contribuire:

1. Leggere la documentazione nella cartella [`docs/`](docs/) (se esiste)
2. Seguire le convenzioni di codice esistenti
3. Aggiungere test per nuove funzionalità
4. Aggiornare la documentazione corrispondente

## 📬 Contatti

Per domande o supporto:
- **Repository**: [https://github.com/fpezzolo/Ileana](https://github.com/fpezzolo/Ileana)
- **Issue Tracker**: Utilizzare il sistema di issue di GitHub
- **Autori**: Federico Pezzolo, Francesco Pezzolo