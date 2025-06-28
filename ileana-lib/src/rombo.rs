use crate::geometria::FiguraGeometrica;
const PROPRIETA_MAGICA_ROMBO: f64 = 2.0;
const PROPRIETA_MAGICA_PERIMETRO_ROMBO: f64 = 4.0;
const NOME: &str = "Rombo";
pub struct Rombo {
    pub diagonale_minore: f64,
    pub diagonale_maggiore: f64,
    pub lato: f64,
}

impl FiguraGeometrica for Rombo {
    fn calcola_area(&self) -> f64 {
        (self.diagonale_maggiore * self.diagonale_minore) / PROPRIETA_MAGICA_ROMBO
    }

    fn calcola_perimetro(&self) -> f64 {
        self.lato * PROPRIETA_MAGICA_PERIMETRO_ROMBO
    }

    fn descrizione(&self) -> &str {
        NOME
    }
}
