

pub mod cerchio; // bla bla bla bla...
pub mod geometria;
pub mod logo;
pub mod quadrato;
pub mod rettangolo; // Modulo per calcolare area e perimetro del rettangolo
pub mod rombo; // modolo per calcolare l area e il perimetro del rombo
pub mod triangolo; // Modulo per calcolare area e perimetro del triangolo // Modulo per stampare il logo




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
