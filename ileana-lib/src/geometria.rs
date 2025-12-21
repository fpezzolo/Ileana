/// Trait che definisce le operazioni comuni per tutte le figure geometriche
pub trait FiguraGeometrica {
    /// Calcola l'area della figura geometrica
    /// 
    /// # Returns
    /// 
    /// `f64` - Il valore dell'area
    fn calcola_area(&self) -> f64;
    
    /// Calcola il perimetro della figura geometrica
    /// 
    /// # Returns
    /// 
    /// `f64` - Il valore del perimetro
    fn calcola_perimetro(&self) -> f64;
    
    /// Restituisce una descrizione testuale della figura
    /// 
    /// # Returns
    /// 
    /// `&str` - Il nome della figura geometrica
    fn descrizione(&self) -> &str;
}
