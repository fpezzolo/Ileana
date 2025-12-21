/// Stampa il logo ASCII del programma Ileana
/// 
/// Questo logo viene visualizzato all'avvio del programma CLI
/// 
/// # Esempio
/// ```
/// use ileana_lib::logo;
/// 
/// fn main() {
///     logo::logo_ileana();
/// }
/// ```
pub fn logo_ileana() {
    println!(
        "
 IIIIIII  L       EEEEEEE  AAAAAAA  N     N  AAAAAAA  
    I     L       E        A     A  NN    N  A     A  
    I     L       EEEEE    AAAAAAA  N N   N  AAAAAAA  
    I     L       E        A     A  N  N  N  A     A  
 IIIIIII  LLLLLL  EEEEEEE  A     A  N   N N  A     A   
"
    );
}
