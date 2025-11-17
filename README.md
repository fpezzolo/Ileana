# Ileana
Programma didattico in Rust I.l.e.a.n.a. Interessante Lavoro Elabora Aree Nonchè Altro


### Note

Versione Beta

### Linguaggio

il programma è realizzato in linguaggio [Rust](https://www.rust-lang.org/)
Per l'installazione di Rust si rimanda al sito ufficiale.

Il programma è sviluppato con [visual code](https://code.visualstudio.com/docs/languages/rust) ed utilizza Rust 1.88.0


### Compilazione

Per compilare il programma si utilizza il comando

```shell
cargo run -p ileana-cli
```

## log attivita

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


## ⚙️ Script di qualità: linting e test

Per garantire coerenza e qualità del codice, questo progetto include tre script PowerShell nella cartella [`scripts/`](scripts/) che puoi eseguire facilmente per analizzare e verificare il comportamento del programma.

### 🧼 Verifica linting con Clippy

```powershell
.\scripts\lint.ps1
```

### 05/2025
un cambio di architettura con la divisione del programma in ileana-cli ed ileana-lib 


## 📄 Licenza

Questo progetto è distribuito sotto licenza [GPL-3.0](LICENSE).