use crate::geometria::FiguraGeometrica;

/// Costante che rappresenta il fattore per il calcolo del perimetro (2)
/// Formula: (base + altezza) × 2
const PROPRIETA_MAGICA_RETTANGOLO: f64 = 2.0;

/// Nome della figura geometrica
const NOME: &str = "Rettangolo";

/// Struttura che rappresenta un rettangolo geometrico
/// 
/// # Esempio
/// ```
/// use ileana_lib::rettangolo::Rettangolo;
/// use ileana_lib::geometria::FiguraGeometrica;
/// 
/// let rettangolo = Rettangolo { base: 3.0, altezza: 2.0 };
/// let area = rettangolo.calcola_area(); // 6.0
/// ```
#[derive(Debug, PartialEq)]
pub struct Rettangolo {
    /// Lunghezza della base del rettangolo
    pub base: f64,
    /// Lunghezza dell'altezza del rettangolo
    pub altezza: f64,
}

impl FiguraGeometrica for Rettangolo {
    /// Calcola l'area del rettangolo
    /// 
    /// Formula: base × altezza
    /// 
    /// # Returns
    /// 
    /// `f64` - Area del rettangolo
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::rettangolo::Rettangolo;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let rettangolo = Rettangolo { base: 3.0, altezza: 2.0 };
    /// assert_eq!(rettangolo.calcola_area(), 6.0);
    /// ```
    fn calcola_area(&self) -> f64 {
        self.base * self.altezza
    }

    /// Calcola il perimetro del rettangolo
    /// 
    /// Formula: (base + altezza) × 2
    /// 
    /// # Returns
    /// 
    /// `f64` - Perimetro del rettangolo
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::rettangolo::Rettangolo;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let rettangolo = Rettangolo { base: 3.0, altezza: 2.0 };
    /// assert_eq!(rettangolo.calcola_perimetro(), 10.0);
    /// ```
    fn calcola_perimetro(&self) -> f64 {
        (self.base + self.altezza) * PROPRIETA_MAGICA_RETTANGOLO
    }

    /// Restituisce il nome della figura
    /// 
    /// # Returns
    /// 
    /// `&str` - "Rettangolo"
    fn descrizione(&self) -> &str {
        NOME
    }
}
