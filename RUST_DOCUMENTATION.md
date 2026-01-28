# Documentazione Rust del Progetto Ileana

Questo documento fornisce una guida pratica su come generare, utilizzare e comprendere la documentazione Rust del progetto Ileana.

> **Nota**: Per una panoramica completa della struttura del progetto, consultare [`PROJECT_STRUCTURE.md`](PROJECT_STRUCTURE.md). Per statistiche e qualità della documentazione, vedere [`DOCUMENTATION_SUMMARY.md`](DOCUMENTATION_SUMMARY.md).

## 📚 Generazione della Documentazione

La documentazione Rust è generata automaticamente da `rustdoc` e include:

- **Documentazione completa** di tutte le strutture e funzioni
- **Esempi funzionanti** testabili come doctest
- **Formule matematiche** utilizzate nei calcoli
- **Descrizioni dettagliate** di tutti i componenti

### Generare Documentazione Locale

```bash
cargo doc --no-deps --open
```

Questo comando:
1. Genera documentazione HTML per tutti i moduli Rust
2. Apre automaticamente la documentazione nel browser
3. Include tutti gli esempi e le formule

### Generare Documentazione Statica

```bash
cargo doc --no-deps
```

I file HTML saranno disponibili in:
```
target/doc/ileana_lib/index.html
```

## 🗂️ Struttura della Documentazione

### Moduli Principali

#### `ileana_lib` - Libreria Geometrica

La libreria contiene implementazioni complete di figure geometriche:

- **`geometria`**: Trait `FiguraGeometrica` con metodi comuni
- **`quadrato`**: Implementazione quadrato con formule e validazione
- **`rettangolo`**: Implementazione rettangolo con gestione casi speciali
- **`triangolo`**: Implementazione triangolo con validazione geometrica
- **`rombo`**: Implementazione rombo con costanti specifiche
- **`cerchio`**: Implementazione cerchio con formule matematiche
- **`logo`**: Funzione per visualizzare il logo ASCII

#### `ileana_cli` - Interfaccia a Riga di Comando

Il modulo CLI espone funzionalità per l'utilizzo da terminale:

- **`main()`**: Punto di ingresso con menu interattivo
- **`leggi_numero()`**: Funzione robusta per input numerici
- **`stampa_info()`**: Funzione generica per output formattato

## 💡 Esempi di Utilizzo

### Utilizzo della Libreria Rust

```rust
use ileana_lib::quadrato::Quadrato;
use ileana_lib::rettangolo::Rettangolo;
use ileana_lib::geometria::FiguraGeometrica;

fn main() {
    // Creazione e utilizzo di un quadrato
    let quadrato = Quadrato::new(5.0);
    println!("Area quadrato: {}", quadrato.area()); // 25.0
    println!("Perimetro quadrato: {}", quadrato.perimetro()); // 20.0
    
    // Creazione e utilizzo di un rettangolo
    let rettangolo = Rettangolo::new(4.0, 6.0);
    println!("Area rettangolo: {}", rettangolo.area()); // 24.0
    println!("Perimetro rettangolo: {}", rettangolo.perimetro()); // 20.0
}
```

### Utilizzo del CLI

```bash
# Esegui il programma CLI
cargo run --bin ileana-cli

# Segui le istruzioni del menu interattivo per:
# - Calcolare aree e perimetri
# - Visualizzare informazioni geometriche
# - Testare diverse figure
```

## ✍️ Convenzioni di Documentazione

La documentazione segue le best practice Rustdoc:

- **`///`**: Commenti di documentazione per elementi pubblici
- **`//!`**: Commenti di documentazione per moduli
- **`# Examples`**: Sezioni con esempi di codice funzionanti
- **`# Returns`**: Descrizione dei valori di ritorno
- **`# Panics`**: Condizioni che causano panic
- **`# Formulas`**: Formule matematiche utilizzate

## 🎯 Best Practice

### Per Sviluppatori

1. **Aggiornare la documentazione**: Ogni nuova funzionalità deve avere documentazione completa
2. **Esempi funzionanti**: Tutti gli esempi devono essere codice valido e testabile
3. **Formule matematiche**: Documentare sempre le formule utilizzate
4. **Unità di misura**: Specificare le unità quando rilevanti

### Per Utenti

1. **Leggere la documentazione**: Prima di utilizzare una funzione, leggere la sua documentazione
2. **Provare gli esempi**: Gli esempi sono testati e funzionanti
3. **Usare i doctest**: I doctest garantiscono che gli esempi siano corretti
4. **Esplorare i moduli**: La documentazione è organizzata per moduli logici

## 🔗 Accesso Rapido

Per accedere rapidamente alla documentazione delle figure geometriche (dopo aver generato la documentazione locale):

- **[Quadrato](target/doc/ileana_lib/quadrato/struct.Quadrato.html)**
- **[Rettangolo](target/doc/ileana_lib/rettangolo/struct.Rettangolo.html)**
- **[Triangolo](target/doc/ileana_lib/triangolo/struct.Triangolo.html)**
- **[Rombo](target/doc/ileana_lib/rombo/struct.Rombo.html)**
- **[Cerchio](target/doc/ileana_lib/cerchio/struct.Cerchio.html)**

> **Nota**: I percorsi sopra sono relativi alla generazione locale della documentazione.

## 📖 Documentazione Esterna

Per approfondire Rust e la documentazione:

- **[The Rust Programming Language - Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)**
- **[Rustdoc Book](https://doc.rust-lang.org/rustdoc/index.html)**
- **[Rust by Example - Documentation](https://doc.rust-lang.org/rust-by-example/meta/doc.html)**

## 🔧 Verifica della Documentazione

### Eseguire Doctest

```bash
cargo test --doc
```

### Eseguire tutti i Test

```bash
cargo test
```

### Verificare la Copertura

```bash
# Genera la documentazione e verifica che tutti gli elementi siano documentati
cargo doc --no-deps
```

## 📚 Documentazione Correlata

- **[Struttura del Progetto](PROJECT_STRUCTURE.md)**: Per informazioni sull'architettura generale
- **[Riassunto Documentazione](DOCUMENTATION_SUMMARY.md)**: Per statistiche e qualità
- **[README principale](README.md)**: Per una panoramica del progetto
