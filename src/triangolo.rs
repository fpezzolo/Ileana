use crate::geometria::FiguraGeometrica;
const PROPRIETA_MAGICA_TRIANGOLO: f64 = 2.0;
const NOME: &str = "Triangolo";

pub struct Triangolo {
    pub lato1: f64,
    pub lato2: f64,
    pub lato_base: f64,
    pub altezza: f64,
}

impl FiguraGeometrica for Triangolo {
    fn calcola_area(&self) -> f64 {
        (self.lato_base * self.altezza) / PROPRIETA_MAGICA_TRIANGOLO
    }

    fn calcola_perimetro(&self) -> f64 {
        self.lato1 + self.lato2 + self.lato_base
    }

    fn descrizione(&self) -> &str {
        NOME
    }
}
