use wasm_bindgen::prelude::*;

use ileana_lib::cerchio::Cerchio;
use ileana_lib::geometria::FiguraGeometrica;
use ileana_lib::logo;
use ileana_lib::quadrato::Quadrato;
use ileana_lib::rettangolo::Rettangolo;
use ileana_lib::rombo::Rombo;
use ileana_lib::triangolo::Triangolo;

#[wasm_bindgen]
pub fn area_rettangolo_js(latoinserito: f64) -> f64 {
    let quadrato: Quadrato = Quadrato { lato: latoinserito };
    quadrato.calcola_area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
