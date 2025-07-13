pub mod cerchio; // bla bla bla bla...
pub mod geometria;
pub mod logo;
pub mod quadrato;
pub mod rettangolo; // Modulo per calcolare area e perimetro del rettangolo
pub mod rombo; // modolo per calcolare l area e il perimetro del rombo
pub mod triangolo; // Modulo per calcolare area e perimetro del triangolo // Modulo per stampare il logo

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


}
