use crate::geometria::FiguraGeometrica;
const NOME: &str = "Cerchio";

pub struct Cerchio {
    pub raggio: f64,
}

impl FiguraGeometrica for Cerchio {
    fn calcola_area(&self) -> f64 {
        self.raggio * self.raggio * std::f64::consts::PI
    }

    fn calcola_perimetro(&self) -> f64 {
        self.raggio * 2.0 * std::f64::consts::PI
    }

    fn descrizione(&self) -> &str {
        NOME
    }
}
