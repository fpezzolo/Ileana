use std::io;

use ileana_lib::geometria::FiguraGeometrica;
use ileana_lib::cerchio::Cerchio;
use ileana_lib::quadrato::Quadrato;
use ileana_lib::rettangolo::Rettangolo;
use ileana_lib::rombo::Rombo;
use ileana_lib::triangolo::Triangolo;
use ileana_lib::logo;

/// Punto di ingresso principale del programma CLI Ileana
/// 
/// Questo programma permette di calcolare area e perimetro di diverse figure geometriche
/// attraverso un'interfaccia interattiva a riga di comando.
/// 
/// # Funzionalità
/// 
/// - Calcolo area e perimetro di quadrati
/// - Calcolo area e perimetro di rettangoli
/// - Calcolo area e perimetro di triangoli (con validazione)
/// - Calcolo area e perimetro di rombi
/// - Calcolo area e circonferenza di cerchi
/// 
/// # Esempio di utilizzo
/// 
/// ```bash
/// cargo run --bin ileana-cli
/// ```
fn main() {
    // Stampa il logo del programma
    logo::logo_ileana();

    loop {
        // Mostra il menu delle opzioni all'utente
        println!("\nSeleziona un'opzione:");
        println!("1 - Calcola l'area e il perimetro di un quadrato 🟩");
        println!("2 - Calcola l'area e il perimetro di un rettangolo 🟥🟥");
        println!("3 - Calcola l'area e il perimetro di un triangolo 🔺");
        println!("4 - Calcola l'area e il perimetro di un rombo 🔶");
        println!("5 - Calcola l'area e il perimetro di un cerchio ");
        println!("9 - Esci 👋");

        // Legge la scelta dell'utente
        let mut scelta = String::new();
        io::stdin()
            .read_line(&mut scelta)
            .expect("Errore nella lettura");

        // Converte l'input in un numero
        let scelta: u32 = match scelta.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Inserisci un numero valido.");
                continue; // Se l'input non è valido, ripete la richiesta
            }
        };

        // Gestisce le diverse opzioni del menu
        match scelta {
            1 => {
                println!("Inserisci il lato del quadrato:");
                let latoinserito: f64 = leggi_numero(); // Legge il valore del lato

                let quadrato: Quadrato = Quadrato { lato: latoinserito };
                stampa_info(&quadrato);
            }
            2 => {
                println!("Inserisci la base del rettangolo:");
                let baseinserita: f64 = leggi_numero(); // Legge la base
                println!("Inserisci l'altezza del rettangolo:");
                let altezzainserita: f64 = leggi_numero(); // Legge l'altezza

                let rettangolo: Rettangolo = Rettangolo {
                    base: baseinserita,
                    altezza: altezzainserita,
                };
                stampa_info(&rettangolo);

                /*

                               println!("Inserisci la base del rettangolo:");
                               let base: f64 = leggi_numero();  // Legge la base
                               println!("Inserisci l'altezza del rettangolo:");
                               let altezza: f64 = leggi_numero();  // Legge l'altezza
                               let area: f64 = rettangolo::calcola_area(base, altezza);  // Calcola l'area
                               println!("L'area del rettangolo con base {} e altezza {} è {}", base, altezza, area);

                               let perimetro: f64 = rettangolo::calcola_perimetro(base, altezza);  // Calcola il perimetro
                               println!("Il perimetro del rettangolo con base {} e altezza {} è {}", base, altezza, perimetro);
                */
            }
            3 => {
                println!("Inserisci la base del triangolo:");
                let baseletta: f64 = leggi_numero(); // Legge la base
                println!("Inserisci l'altezza del triangolo:");
                let altezzaletta: f64 = leggi_numero(); // Legge l'altezza

                println!("Inserisci il lato 1 del triangolo:");
                let lato1letto: f64 = leggi_numero(); // Legge la base
                println!("Inserisci il lato 2 del triangolo:");
                let lato2letto: f64 = leggi_numero(); // Legge l'altezza

                let triangolo: Triangolo = Triangolo {
                    lato1: lato1letto,
                    lato2: lato2letto,
                    lato_base: baseletta,
                    altezza: altezzaletta,
                };
                stampa_info(&triangolo);

                /*
                 println!("Inserisci la base del triangolo:");
                   let base: f64 = leggi_numero();  // Legge la base
                   println!("Inserisci l'altezza del triangolo:");
                   let altezza: f64 = leggi_numero();  // Legge l'altezza
                   let area: f64 = triangolo::calcola_area(base, altezza);  // Calcola l'area
                   println!("L'area del triangolo con base {} e altezza {} è {}", base, altezza, area);


                   println!("Inserisci il lato 1 del triangolo:");
                   let lato1: f64 = leggi_numero();  // Legge la base
                   println!("Inserisci il lato 2 del triangolo:");
                   let lato2: f64 = leggi_numero();  // Legge l'altezza

                   let perimetro: f64 = triangolo::calcola_perimetro(base,lato1,lato2);  // Calcola il perimetro
                   println!("Il perimetro del triangolo con base {} e lato1 {} e lato2 {} è {}", base, lato1, lato2,perimetro);


                */
            }
            4 => {
                println!("Inserisci la diagonale maggiore del rombo:");
                let diagonale_maggioreletto: f64 = leggi_numero(); // Legge la diaginale maggiore
                println!("Inserisci la diagonale minore del rombo:");
                let diagonale_minoreletto: f64 = leggi_numero(); // Legge la diagonale minore

                println!("Inserisci il lato del rombo:");
                let latoletto: f64 = leggi_numero(); // Legge la diagonale minore

                let rombo: Rombo = Rombo {
                    diagonale_maggiore: diagonale_maggioreletto,
                    diagonale_minore: diagonale_minoreletto,
                    lato: latoletto,
                };
                stampa_info(&rombo);

                /*

                 println!("Inserisci la diagonale maggiore del rombo:");
                let diagonale_maggiore:f64  = leggi_numero();  // Legge la diaginale maggiore
                println!("Inserisci la diagonale minore del rombo:");
                let diagonale_minore:f64  = leggi_numero();  // Legge la diagonale minore
                let area:f64  = rombo::calcola_area(diagonale_maggiore, diagonale_minore);  // Calcola l'area

                println!("L'area del rombo con diagonale maggiore {} e diagonale minore {} è {}",diagonale_maggiore ,diagonale_minore , area);

                println!("Inserisci il lato del rombo:");
                let lato: f64= leggi_numero();  // Legge la diagonale minore

                let perimetro:f64  = rombo::calcola_perimetro(lato);  // Calcola il perimetro
                println!("Il perimetro del rombone con lato {} è {}", lato, perimetro);

                 */
            }
            5 => {
                println!("Inserisci il raggio del cerchio:");
                let raggioinserito: f64 = leggi_numero(); // Legge il valore del raggio

                let cerchio: Cerchio = Cerchio {
                    raggio: raggioinserito,
                };
                stampa_info(&cerchio);
            }
            9 => {
                println!("Uscita dal programma. Arrivederci! 👋"); // Termina il programma
                break;
            }
            _ => println!("Opzione non valida, riprova."), // Gestisce errori di input
        }
    }
}

/// Legge un numero in virgola mobile dall'input standard
/// 
/// # Returns
/// 
/// `f64` - Il numero inserito dall'utente
/// 
/// # Panics
/// 
/// Panic se l'input non può essere convertito in un numero valido
/// 
/// # Esempio
/// 
/// ```
/// let numero = leggi_numero(); // L'utente inserisce "3.14"
/// assert_eq!(numero, 3.14);
/// ```
fn leggi_numero() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Errore nella lettura");
    input.trim().parse().expect("Inserisci un numero valido") // Converte l'input in numero
}

/// Stampa le informazioni di area e perimetro di una figura geometrica
/// 
/// # Argomenti
/// 
/// * `figura` - Un riferimento a qualsiasi tipo che implementa il trait `FiguraGeometrica`
/// 
/// # Esempio
/// 
/// ```
/// let quadrato = Quadrato { lato: 5.0 };
/// stampa_info(&quadrato);
/// // Stampa:
/// // Area del Quadrato: 25
/// // Perimetro del Quadrato: 20
/// ```
fn stampa_info(figura: &impl FiguraGeometrica) {
    println!(
        "Area del {}: {}",
        figura.descrizione(),
        figura.calcola_area()
    );
    println!(
        "Perimetro del {}: {}",
        figura.descrizione(),
        figura.calcola_perimetro()
    );
}
