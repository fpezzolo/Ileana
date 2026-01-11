//! # Ileana - Libreria Geometrica
//! 
//! Una libreria Rust per calcolare area, perimetro e altre proprietà di figure geometriche.
//! 
//! Questa libreria fornisce implementazioni per:
//! - Quadrati
//! - Rettangoli
//! - Triangoli
//! - Rombo
//! - Cerchi
//! 
//! Tutte le figure implementano il trait `FiguraGeometrica` che definisce i metodi comuni.

pub mod cerchio; ///< Modulo per calcolare area e perimetro del cerchio
pub mod geometria; ///< Modulo con il trait FiguraGeometrica e definizioni comuni
pub mod logo; ///< Modulo per stampare il logo del programma
pub mod quadrato; ///< Modulo per calcolare area e perimetro del quadrato
pub mod rettangolo; ///< Modulo per calcolare area e perimetro del rettangolo
pub mod rombo; ///< Modulo per calcolare area e perimetro del rombo
pub mod triangolo; ///< Modulo per calcolare area e perimetro del triangolo

#[cfg(test)]
mod tests {

    use crate::cerchio::Cerchio;
    use crate::geometria::FiguraGeometrica;
    use crate::quadrato::Quadrato;
    use crate::rettangolo::Rettangolo;
    use crate::rombo::Rombo;
    use crate::triangolo::Triangolo;

    // ---------------------  quadrato  ---------

    #[test]
    fn test_area_quadrato() {
        let q: Quadrato = Quadrato { lato: 3.0 };
        assert_eq!(q.calcola_area(), 9.0);
    }

    #[test]
    fn test_2_area_quadrato() {
        let q: Quadrato = Quadrato { lato: 3.5 };
        assert_eq!(q.calcola_area(), 12.25);
    }

    #[test]
    fn test_perimetro_quadrato() {
        let q = Quadrato { lato: 2.5 };
        assert_eq!(q.calcola_perimetro(), 10.0); // 2.5 * 4.0
    }

    #[test]
    fn test_descrizione_quadrato() {
        let q = Quadrato { lato: 1.0 };
        assert_eq!(q.descrizione(), "Quadrato");
    }

    #[test]
    fn test_descrizione_errata_quadrato() {
        let q: Quadrato = Quadrato { lato: 1.0 };
        assert_ne!(q.descrizione(), "Pippo");
    }

    // Test aggiuntivi per quadrato
    #[test]
    fn test_quadrato_edge_cases() {
        // Test con lato 0
        let q_zero = Quadrato { lato: 0.0 };
        assert_eq!(q_zero.calcola_area(), 0.0);
        assert_eq!(q_zero.calcola_perimetro(), 0.0);

        // Test con lato molto grande
        let q_grande = Quadrato { lato: 1e6 };
        assert_eq!(q_grande.calcola_area(), 1e12);
        assert_eq!(q_grande.calcola_perimetro(), 4e6);
    }

    #[test]
    fn test_quadrato_uguaglianza() {
        let q1 = Quadrato { lato: 5.0 };
        let q2 = Quadrato { lato: 5.0 };
        let q3 = Quadrato { lato: 3.0 };

        assert_eq!(q1, q2); // Stesso lato
        assert_ne!(q1, q3); // Lato diverso
    }

    #[test]
    fn test_quadrato_proprieta() {
        // In un quadrato, se il lato è d, allora perimetro = 4 * sqrt(area)
        let q = Quadrato { lato: 4.0 };
        let area = q.calcola_area();
        let perimetro = q.calcola_perimetro();

        assert_eq!(perimetro, 4.0 * area.sqrt());
    }

    // ---------------------  rettangolo  ---------

    #[test]
    fn test_area_rettangolo() {
        let q: Rettangolo = Rettangolo {
            base: 3.0,
            altezza: 2.0,
        };
        assert_eq!(q.calcola_area(), 6.0);
    }

    #[test]
    fn test_2_area_rettangolo() {
        let q: Rettangolo = Rettangolo {
            base: 3.5,
            altezza: 2.5,
        };
        assert_eq!(q.calcola_area(), 8.75);
    }

    #[test]
    fn test_perimetro_rettangolo() {
        let q: Rettangolo = Rettangolo {
            base: 3.0,
            altezza: 2.0,
        };
        assert_eq!(q.calcola_perimetro(), 10.0); // 2.5 * 4.0
    }

    #[test]
    fn test_descrizione_rettangolo() {
        let q: Rettangolo = Rettangolo {
            base: 3.0,
            altezza: 2.0,
        };
        assert_eq!(q.descrizione(), "Rettangolo");
    }

    #[test]
    fn test_descrizione_errata_rettangolo() {
        let q: Rettangolo = Rettangolo {
            base: 3.0,
            altezza: 2.0,
        };
        assert_ne!(q.descrizione(), "Pippo");
    }

    // Test aggiuntivi per rettangolo
    #[test]
    fn test_rettangolo_edge_cases() {
        // Test con base 0
        let r_base_zero = Rettangolo { base: 0.0, altezza: 5.0 };
        assert_eq!(r_base_zero.calcola_area(), 0.0);
        assert_eq!(r_base_zero.calcola_perimetro(), 10.0);

        // Test con altezza 0
        let r_altezza_zero = Rettangolo { base: 5.0, altezza: 0.0 };
        assert_eq!(r_altezza_zero.calcola_area(), 0.0);
        assert_eq!(r_altezza_zero.calcola_perimetro(), 10.0);

        // Test con entrambi 0
        let r_zero = Rettangolo { base: 0.0, altezza: 0.0 };
        assert_eq!(r_zero.calcola_area(), 0.0);
        assert_eq!(r_zero.calcola_perimetro(), 0.0);
    }

    #[test]
    fn test_rettangolo_uguaglianza() {
        let r1 = Rettangolo { base: 3.0, altezza: 2.0 };
        let r2 = Rettangolo { base: 3.0, altezza: 2.0 };
        let r3 = Rettangolo { base: 4.0, altezza: 2.0 };

        assert_eq!(r1, r2); // Stesse dimensioni
        assert_ne!(r1, r3); // Dimensioni diverse
    }

    #[test]
    fn test_rettangolo_proprieta() {
        // In un rettangolo, area = base * altezza
        // E perimetro = 2*(base + altezza)
        let r = Rettangolo { base: 4.0, altezza: 6.0 };

        let area = r.calcola_area();
        let perimetro = r.calcola_perimetro();

        // Verifica la formula dell'area
        assert_eq!(area, 4.0 * 6.0);
        
        // Verifica la formula del perimetro
        assert_eq!(perimetro, 2.0 * (4.0 + 6.0));
    }

    #[test]
    fn test_rettangolo_quadrato() {
        // Un quadrato è un caso speciale di rettangolo
        let quadrato = Rettangolo { base: 5.0, altezza: 5.0 };
        let area = quadrato.calcola_area();
        let perimetro = quadrato.calcola_perimetro();

        // Dovrebbe comportarsi come un quadrato
        assert_eq!(area, 25.0); // 5*5
        assert_eq!(perimetro, 20.0); // 2*(5+5)
    }

    // --------------- rombo --------------- per ora il rombo non usa pitagora

    #[test]
    fn test_area_rombo() {
        let q: Rombo = Rombo {
            diagonale_maggiore: 6.0,
            diagonale_minore: 8.0,
            lato: 5.0,
        };
        assert_eq!(q.calcola_area(), 24.0);
    }

    #[test]
    fn test_2_area_rombo() {
        let q: Rombo = Rombo {
            diagonale_maggiore: 10.0,
            diagonale_minore: 8.3,
            lato: 6.5,
        };
        assert_eq!(q.calcola_area(), 41.5);
    }

    #[test]
    fn test_perimetro_rombo() {
        let q: Rombo = Rombo {
            diagonale_maggiore: 6.0,
            diagonale_minore: 8.0,
            lato: 5.0,
        };
        assert_eq!(q.calcola_perimetro(), 20.0);
    }

    #[test]
    fn test_2_perimetro_rombo() {
        let q: Rombo = Rombo {
            diagonale_maggiore: 10.0,
            diagonale_minore: 8.3,
            lato: 6.5,
        };
        assert_eq!(q.calcola_perimetro(), 26.0);
    }

    #[test]
    fn test_descrizione_rombo() {
        let q: Rombo = Rombo {
            diagonale_maggiore: 6.0,
            diagonale_minore: 8.0,
            lato: 5.0,
        };
        assert_eq!(q.descrizione(), "Rombo");
    }

    #[test]
    fn test_descrizione_errata_rombo() {
        let q: Rombo = Rombo {
            diagonale_maggiore: 6.0,
            diagonale_minore: 8.0,
            lato: 5.0,
        };
        assert_ne!(q.descrizione(), "Pippo");
    }

    // Test aggiuntivi per rombo
    #[test]
    fn test_rombo_edge_cases() {
        // Test con diagonali 0
        let q_zero = Rombo {
            diagonale_maggiore: 0.0,
            diagonale_minore: 0.0,
            lato: 0.0,
        };
        assert_eq!(q_zero.calcola_area(), 0.0);
        assert_eq!(q_zero.calcola_perimetro(), 0.0);
    }

    #[test]
    fn test_rombo_uguaglianza() {
        let r1 = Rombo {
            diagonale_maggiore: 6.0,
            diagonale_minore: 8.0,
            lato: 5.0,
        };
        let r2 = Rombo {
            diagonale_maggiore: 6.0,
            diagonale_minore: 8.0,
            lato: 5.0,
        };
        let r3 = Rombo {
            diagonale_maggiore: 10.0,
            diagonale_minore: 8.0,
            lato: 5.0,
        };

        assert_eq!(r1, r2); // Stesse dimensioni
        assert_ne!(r1, r3); // Dimensioni diverse
    }

    #[test]
    fn test_rombo_proprieta() {
        // In un rombo, area = (d1 * d2) / 2
        // E perimetro = 4 * lato
        let r = Rombo {
            diagonale_maggiore: 6.0,
            diagonale_minore: 8.0,
            lato: 5.0,
        };

        let area = r.calcola_area();
        let perimetro = r.calcola_perimetro();

        // Verifica la formula dell'area
        assert_eq!(area, (6.0 * 8.0) / 2.0);
        
        // Verifica la formula del perimetro
        assert_eq!(perimetro, 4.0 * 5.0);
    }

    //------------------ triangolo ------------------

    #[test]
    fn test_area_triangolo_scaleno() {
        let q: Triangolo = Triangolo {
            lato1: 5.0,
            lato2: 6.0,
            lato_base: 7.0,
            altezza: 4.28,
        };
        assert_eq!(q.calcola_area(), 14.98);
    }

    #[test]
    fn test_perimetro_triangolo_scaleno() {
        let q: Triangolo = Triangolo {
            lato1: 5.0,
            lato2: 6.0,
            lato_base: 7.0,
            altezza: 4.28,
        };
        assert_eq!(q.calcola_perimetro(), 18.0);
    }





    #[test]
    fn test_area_triangolo_equilatero() {
        let q: Triangolo = Triangolo {
            lato1: 4.0,
            lato2: 4.0,
            lato_base: 4.0,
            altezza: 3.46,
        };
        assert_eq!(q.calcola_area(), 6.92);
    }

    #[test]
    fn test_perimetro_triangolo_equilatero() {
        let q: Triangolo = Triangolo {
            lato1: 4.0,
            lato2: 4.0,
            lato_base: 4.0,
            altezza: 3.46,
        };
        assert_eq!(q.calcola_perimetro(), 12.0);
    }






   #[test]
    fn test_area_triangolo_rettangolo() {
        let q: Triangolo = Triangolo {
            lato1: 3.0,
            lato2: 4.0,
            lato_base: 5.0,
            altezza: 2.4,
        };
        assert_eq!(q.calcola_area(), 6.0);
    }

    #[test]
    fn test_perimetro_triangolo_rettangolo() {
        let q: Triangolo = Triangolo {
            lato1: 4.0,
            lato2: 4.0,
            lato_base: 4.0,
            altezza: 3.46,
        };
        assert_eq!(q.calcola_perimetro(), 12.0);
    }




    #[test]
    fn test_descrizione_triangolo() {
        let q: Triangolo = Triangolo {
            lato1: 5.0,
            lato2: 6.0,
            lato_base: 7.0,
            altezza: 4.28,
        };
        assert_eq!(q.descrizione(), "Triangolo");
    }

    #[test]
    fn test_descrizione_errata_triangolo() {
        let q: Triangolo = Triangolo {
            lato1: 5.0,
            lato2: 6.0,
            lato_base: 7.0,
            altezza: 4.28,
        };
        assert_ne!(q.descrizione(), "Pippo");
    }

    // Test aggiuntivi per triangolo
    #[test]
    fn test_triangolo_3_4_5_valido() {
        // Test con triangolo rettangolo 3-4-5 (valido)
        // Questo test verifica che il fix per la disuguaglianza triangolare funzioni correttamente
        let t = Triangolo {
            lato1: 3.0,
            lato2: 4.0,
            lato_base: 5.0,
            altezza: 2.4,
        };

        // Il triangolo dovrebbe essere valido
        assert_eq!(t.descrizione(), "Triangolo");
        assert_eq!(t.calcola_area(), 6.0); // (5.0 * 2.4) / 2.0
        assert_eq!(t.calcola_perimetro(), 12.0); // 3.0 + 4.0 + 5.0
    }

    #[test]
    fn test_triangolo_non_valido() {
        // Triangolo non valido: 1 + 2 non è > 5 (disuguaglianza triangolare)
        let t_invalido = Triangolo {
            lato1: 1.0,
            lato2: 2.0,
            lato_base: 5.0,
            altezza: 1.0,
        };

        // Un triangolo non valido dovrebbe restituire 0 per area e perimetro
        assert_eq!(t_invalido.calcola_area(), 0.0);
        assert_eq!(t_invalido.calcola_perimetro(), 0.0);
        assert_eq!(t_invalido.descrizione(), "Errore, Non è un Triangolo !");
    }

    #[test]
    fn test_triangolo_uguaglianza() {
        let t1 = Triangolo {
            lato1: 3.0,
            lato2: 4.0,
            lato_base: 5.0,
            altezza: 2.4,
        };
        let t2 = Triangolo {
            lato1: 3.0,
            lato2: 4.0,
            lato_base: 5.0,
            altezza: 2.4,
        };
        let t3 = Triangolo {
            lato1: 5.0,
            lato2: 6.0,
            lato_base: 7.0,
            altezza: 4.28,
        };

        assert_eq!(t1, t2); // Stessi lati
        assert_ne!(t1, t3); // Lati diversi
    }

    #[test]
    fn test_triangolo_proprieta() {
        // In un triangolo rettangolo 3-4-5, l'area dovrebbe essere (3*4)/2 = 6
        let t = Triangolo {
            lato1: 3.0,
            lato2: 4.0,
            lato_base: 5.0,
            altezza: 2.4,
        };

        let area = t.calcola_area();
        let perimetro = t.calcola_perimetro();

        // Verifica che sia un triangolo valido
        assert_eq!(t.descrizione(), "Triangolo");
        
        // Verifica l'area: (base * altezza) / 2
        assert_eq!(area, (5.0 * 2.4) / 2.0);
        
        // Verifica il perimetro: somma di tutti i lati
        assert_eq!(perimetro, 3.0 + 4.0 + 5.0);
    }

//-------------- cerchio ---------------


    #[test]
    fn test_area_cerchio() {
        let q: Cerchio = Cerchio { raggio: 2.5};
        assert_eq!(q.calcola_area(), 19.634954084936208);
    }



    #[test]
    fn test_perimetro_cerchio() {
       let q: Cerchio = Cerchio { raggio: 2.5};
        assert_eq!(q.calcola_perimetro(), 15.707963267948966); 
    }

    #[test]
    fn test_2_area_cerchio() {
        let q: Cerchio = Cerchio { raggio: 4.0};
        assert_eq!(q.calcola_area(), 50.26548245743669);
    }

    #[test]
    fn test_2_perimetro_cerchio() {
       let q: Cerchio = Cerchio { raggio: 4.0};
        assert_eq!(q.calcola_perimetro(), 25.132741228718345); 
    }

    #[test]
    fn test_descrizione_cerchio() {
        let q: Cerchio = Cerchio { raggio: 4.0};
        assert_eq!(q.descrizione(), "Cerchio");
    }

    #[test]
    fn test_descrizione_errata_cerchio() {
       let q: Cerchio = Cerchio { raggio: 4.0};
        assert_ne!(q.descrizione(), "Pippo");
    }

    // Test aggiuntivi per cerchio
    #[test]
    fn test_cerchio_edge_cases() {
        // Test con raggio 0
        let c_zero = Cerchio { raggio: 0.0 };
        assert_eq!(c_zero.calcola_area(), 0.0);
        assert_eq!(c_zero.calcola_perimetro(), 0.0);

        // Test con raggio molto grande
        let c_grande = Cerchio { raggio: 1e6 };
        let area_grande = c_grande.calcola_area();
        let perimetro_grande = c_grande.calcola_perimetro();
        
        // Verifica che i valori siano ragionevoli
        assert!(area_grande > 0.0);
        assert!(perimetro_grande > 0.0);
    }

    #[test]
    fn test_cerchio_uguaglianza() {
        let c1 = Cerchio { raggio: 2.5 };
        let c2 = Cerchio { raggio: 2.5 };
        let c3 = Cerchio { raggio: 3.0 };

        assert_eq!(c1, c2); // Stesso raggio
        assert_ne!(c1, c3); // Raggio diverso
    }

    #[test]
    fn test_cerchio_proprieta() {
        // In un cerchio, area = π * r² e perimetro = 2 * π * r
        let c = Cerchio { raggio: 2.0 };
        let area = c.calcola_area();
        let perimetro = c.calcola_perimetro();
        let pi = std::f64::consts::PI;

        // Verifica la formula dell'area con tolleranza per floating point
        assert!((area - (pi * 2.0 * 2.0)).abs() < 1e-10);
        
        // Verifica la formula del perimetro
        assert!((perimetro - (2.0 * pi * 2.0)).abs() < 1e-10);
        
        // Verifica la relazione: perimetro = 2 * sqrt(π * area)
        assert!((perimetro - 2.0 * (pi * area).sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_cerchio_precisione() {
        // Test di precisione per valori noti
        let c = Cerchio { raggio: 1.0 };
        let area = c.calcola_area();
        let perimetro = c.calcola_perimetro();
        let pi = std::f64::consts::PI;

        // Area dovrebbe essere π (circa 3.1415926535...)
        assert!((area - pi).abs() < 1e-10);
        
        // Perimetro dovrebbe essere 2π (circa 6.283185307...)
        assert!((perimetro - (2.0 * pi)).abs() < 1e-10);
    }

}
