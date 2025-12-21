use crate::geometria::FiguraGeometrica;

/// Nome della figura geometrica
const NOME: &str = "Cerchio";

/// Struttura che rappresenta un cerchio geometrico
/// 
/// # Esempio
/// ```
/// use ileana_lib::cerchio::Cerchio;
/// use ileana_lib::geometria::FiguraGeometrica;
/// 
/// let cerchio = Cerchio { raggio: 2.5 };
/// let area = cerchio.calcola_area(); // ~19.63
/// ```
#[derive(Debug, PartialEq)]
pub struct Cerchio {
    /// Lunghezza del raggio del cerchio
    pub raggio: f64,
}

impl FiguraGeometrica for Cerchio {
    /// Calcola l'area del cerchio
    /// 
    /// Formula: raggio² × π
    /// 
    /// # Returns
    /// 
    /// `f64` - Area del cerchio
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::cerchio::Cerchio;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let cerchio = Cerchio { raggio: 2.5 };
    /// assert_eq!(cerchio.calcola_area(), 19.634954084936208);
    /// ```
    fn calcola_area(&self) -> f64 {
        self.raggio * self.raggio * std::f64::consts::PI
    }

    /// Calcola la circonferenza del cerchio (perimetro)
    /// 
    /// Formula: 2 × raggio × π
    /// 
    /// # Returns
    /// 
    /// `f64` - Circonferenza del cerchio
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::cerchio::Cerchio;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let cerchio = Cerchio { raggio: 2.5 };
    /// assert_eq!(cerchio.calcola_perimetro(), 15.707963267948966);
    /// ```
    fn calcola_perimetro(&self) -> f64 {
        self.raggio * 2.0 * std::f64::consts::PI
    }

    /// Restituisce il nome della figura
    /// 
    /// # Returns
    /// 
    /// `&str` - "Cerchio"
    fn descrizione(&self) -> &str {
        NOME
    }
}
