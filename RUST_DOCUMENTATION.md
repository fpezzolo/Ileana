# Documentazione Rust del Progetto Ileana

Questo documento descrive come accedere e utilizzare la documentazione della parte Rust del progetto Ileana.

## Generazione della Documentazione

La documentazione Rust è stata generata utilizzando `rustdoc` e include:

- Documentazione completa di tutte le strutture dati
- Descrizione dei metodi e delle funzioni
- Esempi di utilizzo
- Formule matematiche utilizzate

## Come Generare la Documentazione

Per generare la documentazione locale, eseguire:

```bash
cargo doc --no-deps --open
```

Questo comando:
1. Genera la documentazione per tutti i moduli Rust
2. Apre automaticamente la documentazione nel browser predefinito

## Struttura della Documentazione

### Moduli Principali

#### `ileana_lib` - Libreria Geometrica

- **`geometria`**: Trait `FiguraGeometrica` con metodi comuni
- **`quadrato`**: Implementazione della figura quadrato
- **`rettangolo`**: Implementazione della figura rettangolo  
- **`triangolo`**: Implementazione della figura triangolo con validazione
- **`rombo`**: Implementazione della figura rombo
- **`cerchio`**: Implementazione della figura cerchio
- **`logo`**: Funzione per stampare il logo ASCII

#### `ileana_cli` - Interfaccia a Riga di Comando

- Funzione `main()`: Punto di ingresso del programma CLI
- Funzione `leggi_numero()`: Lettura di input numerici
- Funzione `stampa_info()`: Visualizzazione dei risultati

## Esempi di Utilizzo

### Utilizzo della Libreria

```rust
use ileana_lib::quadrato::Quadrato;
use ileana_lib::geometria::FiguraGeometrica;

fn main() {
    let quadrato = Quadrato { lato: 5.0 };
    println!("Area: {}", quadrato.calcola_area()); // 25.0
    println!("Perimetro: {}", quadrato.calcola_perimetro()); // 20.0
}
```

### Utilizzo del CLI

```bash
# Esegui il programma CLI
cargo run --bin ileana-cli

# Segui le istruzioni del menu interattivo
```

## Convenzioni di Documentazione

La documentazione segue le convenzioni Rustdoc:

- **`///`**: Commenti di documentazione per elementi pubblici
- **`//!`**: Commenti di documentazione per moduli
- **`# Examples`**: Sezioni con esempi di codice
- **`# Returns`**: Descrizione dei valori di ritorno
- **`# Panics`**: Condizioni che causano panic

## Documentazione Online

La documentazione può essere generata anche in formato HTML statico:

```bash
cargo doc --no-deps
```

I file HTML saranno disponibili in:
```
target/doc/ileana_lib/index.html
```

## Best Practice

1. **Aggiornare la documentazione**: Quando si aggiungono nuove funzionalità, aggiornare sempre i commenti di documentazione
2. **Esempi funzionanti**: Tutti gli esempi nei commenti devono essere codice valido e testabile
3. **Formule matematiche**: Documentare sempre le formule utilizzate nei calcoli
4. **Unità di misura**: Specificare le unità di misura quando rilevanti

## Accesso Rapido

Per accedere rapidamente alla documentazione delle figure geometriche:

- [Quadrato](target/doc/ileana_lib/quadrato/struct.Quadrato.html)
- [Rettangolo](target/doc/ileana_lib/rettangolo/struct.Rettangolo.html)  
- [Triangolo](target/doc/ileana_lib/triangolo/struct.Triangolo.html)
- [Rombo](target/doc/ileana_lib/rombo/struct.Rombo.html)
- [Cerchio](target/doc/ileana_lib/cerchio/struct.Cerchio.html)

> **Nota**: I percorsi sopra sono relativi alla generazione locale della documentazione.

## Documentazione Esterna

Per ulteriori informazioni su Rust e la documentazione:

- [The Rust Programming Language - Documentation](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html)
- [Rustdoc Book](https://doc.rust-lang.org/rustdoc/index.html)
- [Rust by Example - Documentation](https://doc.rust-lang.org/rust-by-example/meta/doc.html)
