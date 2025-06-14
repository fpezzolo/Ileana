use crate::geometria::FiguraGeometrica;
const PROPRIETA_MAGICA_TRIANGOLO: f64 = 2.0;
const NOME: &str = "Triangolo";
const ERRORE_NOME: &str = "Errore, Non è un Triangolo !"; // 🖕

pub struct Triangolo {
    pub lato1: f64,
    pub lato2: f64,
    pub lato_base: f64,
    pub altezza: f64,
}

impl Triangolo {
    fn disuguaglianza_triangolosa(&self) -> bool {
        if self.lato1 + self.lato2 > self.lato_base{
            return true
        }
         false
    }
}

impl FiguraGeometrica for Triangolo {
    fn calcola_area(&self) -> f64 {
        if self.disuguaglianza_triangolosa() {
            (self.lato_base * self.altezza) / PROPRIETA_MAGICA_TRIANGOLO
        } else {
            0.0
        }
    }

    fn calcola_perimetro(&self) -> f64 {
        if self.disuguaglianza_triangolosa() {
            self.lato1 + self.lato2 + self.lato_base
        } else {
            0.0
        }
    }

    fn descrizione(&self) -> &str {
        
        if self.disuguaglianza_triangolosa() {
            NOME
        } else {
           ERRORE_NOME
        }

    }
}
