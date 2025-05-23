use crate::geometria::FiguraGeometrica;
const PROPRIETA_MAGICA_RETTANGOLO: f64 = 2.0;
const NOME: &str = "Rettangolo";
pub struct Rettangolo{
   pub base: f64,
   pub altezza: f64,
}

impl FiguraGeometrica for Rettangolo {
     fn calcola_area(&self) -> f64 {
        self.base * self.altezza
    }
    
     fn calcola_perimetro(&self) -> f64 {
        (self.base + self.altezza) * PROPRIETA_MAGICA_RETTANGOLO
    }

    fn descrizione(&self) -> &str {
        NOME
    }
}