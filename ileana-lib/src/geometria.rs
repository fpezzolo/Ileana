pub trait FiguraGeometrica {
    fn calcola_area(&self) -> f64;
    fn calcola_perimetro(&self) -> f64;
    fn descrizione(&self) -> &str;
}
