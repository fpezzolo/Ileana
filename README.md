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

```md
Usa il comando `cargo run -p ileana-cli` per lanciare il progetto.
```

## log attivita

### 07/2025
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