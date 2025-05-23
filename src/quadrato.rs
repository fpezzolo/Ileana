use crate::geometria::FiguraGeometrica;
const PROPRIETA_MAGICA_QUADRATO: f64 = 4.0;
const NOME: &str = "Quadrato";

pub struct Quadrato{
    pub lato: f64,
}

impl FiguraGeometrica for Quadrato {
   
     fn calcola_area(&self) -> f64 {
        self.lato * self.lato
    }
    
     fn calcola_perimetro(&self) -> f64 {
        self.lato * PROPRIETA_MAGICA_QUADRATO
    }
    
    fn descrizione(&self) -> &str {
        NOME
    }





}