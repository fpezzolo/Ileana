pub mod cerchio; // bla bla bla bla...
pub mod geometria;
pub mod logo;
pub mod quadrato;
pub mod rettangolo; // Modulo per calcolare area e perimetro del rettangolo
pub mod rombo; // modolo per calcolare l area e il perimetro del rombo
pub mod triangolo; // Modulo per calcolare area e perimetro del triangolo // Modulo per stampare il logo

#[cfg(test)]
mod tests {

use crate::geometria::FiguraGeometrica;
use crate::quadrato::Quadrato;
use crate::rettangolo::Rettangolo;


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
        let q: Quadrato = Quadrato {  lato: 1.0  };
        assert_ne!(q.descrizione(), "Pippo");
    }


//********* */

  #[test]
    fn test_area_rettangolo() {
        let q: Rettangolo = Rettangolo { base: 3.0, altezza: 2.0};
        assert_eq!(q.calcola_area(), 6.0);
    }

    #[test]
    fn test_2_area_rettangolo() {
        let q: Rettangolo = Rettangolo { base: 3.5, altezza: 2.5 };
        assert_eq!(q.calcola_area(), 8.75);
    }




    #[test]
    fn test_perimetro_rettangolo() {
        let q: Rettangolo = Rettangolo { base: 3.0, altezza: 2.0 };
        assert_eq!(q.calcola_perimetro(), 10.0); // 2.5 * 4.0
    }

    #[test]
    fn test_descrizione_rettangolo() {
        let q: Rettangolo = Rettangolo { base: 3.0, altezza: 2.0 };
        assert_eq!(q.descrizione(), "Rettangolo");
    }
    
    #[test]
    fn test_descrizione_errata_rettangolo() {
        let q: Rettangolo = Rettangolo { base: 3.0, altezza: 2.0 };
        assert_ne!(q.descrizione(), "Pippo");
    }


}
