use crate::geometria::FiguraGeometrica;

/// Costante che rappresenta il fattore per il calcolo dell'area (2)
/// Formula: (diagonale_maggiore × diagonale_minore) / 2
const PROPRIETA_MAGICA_ROMBO: f64 = 2.0;

/// Costante che rappresenta il numero di lati di un rombo (4)
/// Formula: lato × 4
const PROPRIETA_MAGICA_PERIMETRO_ROMBO: f64 = 4.0;

/// Nome della figura geometrica
const NOME: &str = "Rombo";

/// Struttura che rappresenta un rombo geometrico
/// 
/// # Esempio
/// ```
/// use ileana_lib::rombo::Rombo;
/// use ileana_lib::geometria::FiguraGeometrica;
/// 
/// let rombo = Rombo {
///     diagonale_minore: 6.0,
///     diagonale_maggiore: 8.0,
///     lato: 5.0
/// };
/// let area = rombo.calcola_area(); // 24.0
/// ```
#[derive(Debug, PartialEq)]
pub struct Rombo {
    /// Lunghezza della diagonale minore
    pub diagonale_minore: f64,
    /// Lunghezza della diagonale maggiore
    pub diagonale_maggiore: f64,
    /// Lunghezza del lato
    pub lato: f64,
}

impl FiguraGeometrica for Rombo {
    /// Calcola l'area del rombo
    /// 
    /// Formula: (diagonale_maggiore × diagonale_minore) / 2
    /// 
    /// # Returns
    /// 
    /// `f64` - Area del rombo
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::rombo::Rombo;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let rombo = Rombo {
    ///     diagonale_minore: 6.0,
    ///     diagonale_maggiore: 8.0,
    ///     lato: 5.0
    /// };
    /// assert_eq!(rombo.calcola_area(), 24.0);
    /// ```
    fn calcola_area(&self) -> f64 {
        (self.diagonale_maggiore * self.diagonale_minore) / PROPRIETA_MAGICA_ROMBO
    }

    /// Calcola il perimetro del rombo
    /// 
    /// Formula: lato × 4
    /// 
    /// # Returns
    /// 
    /// `f64` - Perimetro del rombo
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::rombo::Rombo;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let rombo = Rombo {
    ///     diagonale_minore: 6.0,
    ///     diagonale_maggiore: 8.0,
    ///     lato: 5.0
    /// };
    /// assert_eq!(rombo.calcola_perimetro(), 20.0);
    /// ```
    fn calcola_perimetro(&self) -> f64 {
        self.lato * PROPRIETA_MAGICA_PERIMETRO_ROMBO
    }

    /// Restituisce il nome della figura
    /// 
    /// # Returns
    /// 
    /// `&str` - "Rombo"
    fn descrizione(&self) -> &str {
        NOME
    }
}
