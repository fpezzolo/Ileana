use wasm_bindgen::prelude::*;


use ileana_lib::geometria::FiguraGeometrica;
// use ileana_lib::logo;
use ileana_lib::quadrato::Quadrato;
use ileana_lib::rettangolo::Rettangolo;
// use ileana_lib::rombo::Rombo;
// use ileana_lib::triangolo::Triangolo;
// use ileana_lib::cerchio::Cerchio;

#[wasm_bindgen]
pub fn area_quadrato_js(latoinserito: f64) -> f64 {
    let quadrato: Quadrato = Quadrato { lato: latoinserito };
    quadrato.calcola_area()
}

#[wasm_bindgen]
pub fn perimetro_quadrato_js(latoinserito: f64) -> f64 {
    let quadrato: Quadrato = Quadrato { lato: latoinserito };
    quadrato.calcola_perimetro()
}

#[wasm_bindgen]
pub fn area_rettangolo_js(altezza: f64,base: f64) -> f64 {
    let rettangolo: Rettangolo = Rettangolo { altezza,base };
    rettangolo.calcola_area()
}

#[wasm_bindgen]
pub fn perimetro_rettangolo_js(altezza: f64,base: f64) -> f64 {
    let rettangolo: Rettangolo = Rettangolo { altezza,base };
    rettangolo.calcola_perimetro()
}


#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
