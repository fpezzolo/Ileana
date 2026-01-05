// 1. Importa la tua libreria e le struct geometriche
use ileana_lib::quadrato::Quadrato;
use ileana_lib::rettangolo::Rettangolo;
use ileana_lib::geometria::FiguraGeometrica; 

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


// 2. I TUOI NUOVI COMMANDS
// Command per il calcolo dell'area (usa ileana-lib)
#[tauri::command]
fn calcola_area_quadrato(lato: f64) -> f64 {
    let q = Quadrato { lato };
    q.calcola_area() 
}

// Command per il calcolo del perimetro (usa ileana-lib)
#[tauri::command]
fn calcola_perimetro_quadrato(lato: f64) -> f64 {
    let q = Quadrato { lato };
    q.calcola_perimetro() 
}

// Command per il calcolo dell'area del rettangolo (usa ileana-lib)
#[tauri::command]
fn calcola_area_rettangolo(base: f64, altezza: f64) -> f64 {
    let r = Rettangolo { base, altezza };
    r.calcola_area() 
}

// Command per il calcolo del perimetro del rettangolo (usa ileana-lib)
#[tauri::command]
fn calcola_perimetro_rettangolo(base: f64, altezza: f64) -> f64 {
    let r = Rettangolo { base, altezza };
    r.calcola_perimetro() 
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 3. REGISTRAZIONE DI TUTTI I COMMANDS
        .invoke_handler(tauri::generate_handler![
            greet,                      // Manteniamo il command originale
            calcola_area_quadrato,      // Registriamo l'Area del quadrato
            calcola_perimetro_quadrato, // Registriamo il Perimetro del quadrato
            calcola_area_rettangolo,    // Registriamo l'Area del rettangolo
            calcola_perimetro_rettangolo // Registriamo il Perimetro del rettangolo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}