use crate::geometria::FiguraGeometrica;

/// Costante che rappresenta il numero di lati di un quadrato (4)
const PROPRIETA_MAGICA_QUADRATO: f64 = 4.0;

/// Nome della figura geometrica
const NOME: &str = "Quadrato";

/// Struttura che rappresenta un quadrato geometrico
/// 
/// # Esempio
/// ```
/// use ileana_lib::quadrato::Quadrato;
/// use ileana_lib::geometria::FiguraGeometrica;
/// 
/// let quadrato = Quadrato { lato: 5.0 };
/// let area = quadrato.calcola_area(); // 25.0
/// ```
#[derive(Debug, PartialEq)]
pub struct Quadrato {
    /// Lunghezza del lato del quadrato
    pub lato: f64,
}

impl FiguraGeometrica for Quadrato {
    /// Calcola l'area del quadrato
    /// 
    /// Formula: lato × lato
    /// 
    /// # Returns
    /// 
    /// `f64` - Area del quadrato
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::quadrato::Quadrato;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let quadrato = Quadrato { lato: 3.0 };
    /// assert_eq!(quadrato.calcola_area(), 9.0);
    /// ```
    fn calcola_area(&self) -> f64 {
        self.lato * self.lato
    }

    /// Calcola il perimetro del quadrato
    /// 
    /// Formula: lato × 4
    /// 
    /// # Returns
    /// 
    /// `f64` - Perimetro del quadrato
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::quadrato::Quadrato;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let quadrato = Quadrato { lato: 2.5 };
    /// assert_eq!(quadrato.calcola_perimetro(), 10.0);
    /// ```
    fn calcola_perimetro(&self) -> f64 {
        self.lato * PROPRIETA_MAGICA_QUADRATO
    }

    /// Restituisce il nome della figura
    /// 
    /// # Returns
    /// 
    /// `&str` - "Quadrato"
    fn descrizione(&self) -> &str {
        NOME
    }
}
