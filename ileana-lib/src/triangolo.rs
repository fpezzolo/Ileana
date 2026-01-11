use crate::geometria::FiguraGeometrica;

/// Costante che rappresenta il fattore per il calcolo dell'area (2)
/// Formula: (base × altezza) / 2
const PROPRIETA_MAGICA_TRIANGOLO: f64 = 2.0;

/// Nome della figura geometrica
const NOME: &str = "Triangolo";

/// Messaggio di errore per triangoli non validi
const ERRORE_NOME: &str = "Errore, Non è un Triangolo !";

/// Struttura che rappresenta un triangolo geometrico
/// 
/// # Esempio
/// ```
/// use ileana_lib::triangolo::Triangolo;
/// use ileana_lib::geometria::FiguraGeometrica;
/// 
/// let triangolo = Triangolo {
///     lato1: 3.0,
///     lato2: 4.0,
///     lato_base: 5.0,
///     altezza: 2.4
/// };
/// let area = triangolo.calcola_area(); // 6.0
/// ```
#[derive(Debug, PartialEq)]
pub struct Triangolo {
    /// Lunghezza del primo lato
    pub lato1: f64,
    /// Lunghezza del secondo lato
    pub lato2: f64,
    /// Lunghezza della base
    pub lato_base: f64,
    /// Altezza relativa alla base
    pub altezza: f64,
}

impl Triangolo {
    /// Verifica la validità del triangolo usando la disuguaglianza triangolare
    /// 
    /// Un triangolo è valido se la somma di due lati è maggiore del terzo lato
    /// per tutte e tre le combinazioni
    /// 
    /// # Returns
    /// 
    /// `bool` - `true` se il triangolo è valido, `false` altrimenti
    fn disuguaglianza_triangolosa(&self) -> bool {
        // Controlla tutte e tre le disuguaglianze triangolari
        self.lato1 + self.lato2 > self.lato_base &&
        self.lato1 + self.lato_base > self.lato2 &&
        self.lato2 + self.lato_base > self.lato1
    }
}

impl FiguraGeometrica for Triangolo {
    /// Calcola l'area del triangolo
    /// 
    /// Formula: (base × altezza) / 2
    /// 
    /// # Returns
    /// 
    /// `f64` - Area del triangolo, o 0.0 se il triangolo non è valido
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::triangolo::Triangolo;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let triangolo = Triangolo {
    ///     lato1: 3.0,
    ///     lato2: 4.0,
    ///     lato_base: 5.0,
    ///     altezza: 2.4
    /// };
    /// assert_eq!(triangolo.calcola_area(), 6.0);
    /// ```
    fn calcola_area(&self) -> f64 {
        if self.disuguaglianza_triangolosa() {
            (self.lato_base * self.altezza) / PROPRIETA_MAGICA_TRIANGOLO
        } else {
            0.0
        }
    }

    /// Calcola il perimetro del triangolo
    /// 
    /// Formula: lato1 + lato2 + base
    /// 
    /// # Returns
    /// 
    /// `f64` - Perimetro del triangolo, o 0.0 se il triangolo non è valido
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::triangolo::Triangolo;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let triangolo = Triangolo {
    ///     lato1: 3.0,
    ///     lato2: 4.0,
    ///     lato_base: 5.0,
    ///     altezza: 2.4
    /// };
    /// assert_eq!(triangolo.calcola_perimetro(), 12.0);
    /// ```
    fn calcola_perimetro(&self) -> f64 {
        if self.disuguaglianza_triangolosa() {
            self.lato1 + self.lato2 + self.lato_base
        } else {
            0.0
        }
    }

    /// Restituisce il nome della figura o un messaggio di errore
    /// 
    /// # Returns
    /// 
    /// `&str` - "Triangolo" se valido, altrimenti un messaggio di errore
    /// 
    /// # Esempio
    /// ```
    /// use ileana_lib::triangolo::Triangolo;
    /// use ileana_lib::geometria::FiguraGeometrica;
    /// 
    /// let triangolo = Triangolo {
    ///     lato1: 3.0,
    ///     lato2: 4.0,
    ///     lato_base: 5.0,
    ///     altezza: 2.4
    /// };
    /// assert_eq!(triangolo.descrizione(), "Triangolo");
    /// ```
    fn descrizione(&self) -> &str {
        
        if self.disuguaglianza_triangolosa() {
            NOME
        } else {
           ERRORE_NOME
        }

    }
}
