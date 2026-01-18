// 1. Importa la tua libreria e le struct geometriche
use ileana_lib::quadrato::Quadrato;
use ileana_lib::rettangolo::Rettangolo;
use ileana_lib::cerchio::Cerchio;
use ileana_lib::triangolo::Triangolo;
use ileana_lib::geometria::FiguraGeometrica;

// Import per il logging con tracing
use tracing::{debug, info, warn, error, instrument};
use tracing_subscriber::{fmt, EnvFilter, prelude::*};
use tracing_appender::rolling;

// Import per informazioni di sistema
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    // Test per Quadrato
    #[test]
    fn test_calcola_area_quadrato() {
        let result = calcola_area_quadrato(3.0);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn test_calcola_perimetro_quadrato() {
        let result = calcola_perimetro_quadrato(2.5);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_quadrato_edge_cases() {
        // Test con valore 0
        assert_eq!(calcola_area_quadrato(0.0), 0.0);
        assert_eq!(calcola_perimetro_quadrato(0.0), 0.0);

        // Test con valore molto grande
        let area_grande = calcola_area_quadrato(1e6);
        let perimetro_grande = calcola_perimetro_quadrato(1e6);
        assert!(area_grande > 0.0);
        assert!(perimetro_grande > 0.0);
    }

    // Test per Rettangolo
    #[test]
    fn test_calcola_area_rettangolo() {
        let result = calcola_area_rettangolo(3.0, 2.0);
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_calcola_perimetro_rettangolo() {
        let result = calcola_perimetro_rettangolo(3.0, 2.0);
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_rettangolo_edge_cases() {
        // Test con base 0
        assert_eq!(calcola_area_rettangolo(0.0, 5.0), 0.0);
        assert_eq!(calcola_perimetro_rettangolo(0.0, 5.0), 10.0);

        // Test con altezza 0
        assert_eq!(calcola_area_rettangolo(5.0, 0.0), 0.0);
        assert_eq!(calcola_perimetro_rettangolo(5.0, 0.0), 10.0);

        // Test con entrambi 0
        assert_eq!(calcola_area_rettangolo(0.0, 0.0), 0.0);
        assert_eq!(calcola_perimetro_rettangolo(0.0, 0.0), 0.0);
    }

    #[test]
    fn test_rettangolo_quadrato() {
        // Un quadrato è un caso speciale di rettangolo
        let quadrato_area = calcola_area_rettangolo(5.0, 5.0);
        let quadrato_perimetro = calcola_perimetro_rettangolo(5.0, 5.0);
        
        assert_eq!(quadrato_area, 25.0); // 5*5
        assert_eq!(quadrato_perimetro, 20.0); // 2*(5+5)
    }

    // Test per Cerchio
    #[test]
    fn test_calcola_area_cerchio() {
        let result = calcola_area_cerchio(2.5);
        let expected = 2.5 * 2.5 * std::f64::consts::PI;
        assert!((result - expected).abs() < 1e-10);
    }

    #[test]
    fn test_calcola_perimetro_cerchio() {
        let result = calcola_perimetro_cerchio(2.5);
        let expected = 2.5 * 2.0 * std::f64::consts::PI;
        assert!((result - expected).abs() < 1e-10);
    }

    #[test]
    fn test_cerchio_edge_cases() {
        // Test con raggio 0
        assert_eq!(calcola_area_cerchio(0.0), 0.0);
        assert_eq!(calcola_perimetro_cerchio(0.0), 0.0);

        // Test con raggio molto grande
        let area_grande = calcola_area_cerchio(1e6);
        let perimetro_grande = calcola_perimetro_cerchio(1e6);
        assert!(area_grande > 0.0);
        assert!(perimetro_grande > 0.0);
    }

    #[test]
    fn test_cerchio_comandi_integrazione() {
        // Test con raggio 1.0 (valori noti)
        let area = calcola_area_cerchio(1.0);
        let perimetro = calcola_perimetro_cerchio(1.0);
        
        let expected_area = std::f64::consts::PI;
        let expected_perimetro = 2.0 * std::f64::consts::PI;
        
        assert!((area - expected_area).abs() < 1e-10);
        assert!((perimetro - expected_perimetro).abs() < 1e-10);
    }

    #[test]
    fn test_triangolo_comandi_integrazione() {
        // Test con triangolo rettangolo 3-4-5 (valori noti)
        let area = calcola_area_triangolo(3.0, 4.0, 5.0, 2.4);
        let perimetro = calcola_perimetro_triangolo(3.0, 4.0, 5.0, 2.4);
        
        // Area dovrebbe essere (5.0 * 2.4) / 2.0 = 6.0
        assert_eq!(area, 6.0);
        
        // Perimetro dovrebbe essere 3.0 + 4.0 + 5.0 = 12.0
        assert_eq!(perimetro, 12.0);
    }

    #[test]
    fn test_triangolo_3_4_5_con_altezza_2_5() {
        // Test con triangolo 3-4-5 e altezza 2.5 (valido)
        let area = calcola_area_triangolo(3.0, 4.0, 5.0, 2.5);
        let perimetro = calcola_perimetro_triangolo(3.0, 4.0, 5.0, 2.5);
        
        // Area dovrebbe essere (5.0 * 2.5) / 2.0 = 6.25
        assert_eq!(area, 6.25);
        
        // Perimetro dovrebbe essere 3.0 + 4.0 + 5.0 = 12.0
        assert_eq!(perimetro, 12.0);
    }

    #[test]
    fn test_triangolo_non_valido() {
        // Test con triangolo non valido (1+2 non > 5)
        let area = calcola_area_triangolo(1.0, 2.0, 5.0, 1.0);
        let perimetro = calcola_perimetro_triangolo(1.0, 2.0, 5.0, 1.0);
        
        // Dovrebbe restituire 0.0 per entrambi
        assert_eq!(area, 0.0);
        assert_eq!(perimetro, 0.0);
    }

    // Test di integrazione completa
    #[test]
    fn test_tutte_le_figure_integrazione() {
        // Test che tutti i comandi funzionino insieme
        let area_quadrato = calcola_area_quadrato(2.0);
        let area_rettangolo = calcola_area_rettangolo(3.0, 2.0);
        let area_cerchio = calcola_area_cerchio(1.0);

        assert!(area_quadrato > 0.0);
        assert!(area_rettangolo > 0.0);
        assert!(area_cerchio > 0.0);
    }

    #[test]
    fn test_proprieta_matematiche() {
        // Test che verifica le proprietà matematiche
        let lato = 4.0;
        let area_quadrato = calcola_area_quadrato(lato);
        let perimetro_quadrato = calcola_perimetro_quadrato(lato);
        
        // In un quadrato: perimetro = 4 * sqrt(area)
        assert!((perimetro_quadrato - 4.0 * area_quadrato.sqrt()).abs() < 1e-10);
        
        // Test per cerchio: perimetro = 2 * sqrt(π * area)
        let raggio = 2.0;
        let area_cerchio = calcola_area_cerchio(raggio);
        let perimetro_cerchio = calcola_perimetro_cerchio(raggio);
        
        assert!((perimetro_cerchio - 2.0 * (std::f64::consts::PI * area_cerchio).sqrt()).abs() < 1e-10);
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
#[instrument]
fn greet(name: &str) -> String {
    debug!("Greet command called with name: {}", name);
    let response = format!("Hello, {}! You've been greeted from Rust!", name);
    debug!("Greet response: {}", response);
    response
}


// 2. I TUOI NUOVI COMMANDS
// Command per il calcolo dell'area (usa ileana-lib)
#[tauri::command]
#[instrument]
fn calcola_area_quadrato(lato: f64) -> Result<f64, String> {
    debug!("Calcolando area quadrato con lato: {}", lato);
    
    if lato < 0.0 {
        let err_msg = format!("Lato negativo non valido: {}", lato);
        error!("{}", err_msg);
        return Err(err_msg);
    }
    
    let q = Quadrato { lato };
    let result = q.calcola_area();
    debug!("Risultato area quadrato: {}", result);
    Ok(result)
}

// Command per il calcolo del perimetro (usa ileana-lib)
#[tauri::command]
#[instrument]
fn calcola_perimetro_quadrato(lato: f64) -> Result<f64, String> {
    debug!("Calcolando perimetro quadrato con lato: {}", lato);
    
    if lato < 0.0 {
        let err_msg = format!("Lato negativo non valido: {}", lato);
        error!("{}", err_msg);
        return Err(err_msg);
    }
    
    let q = Quadrato { lato };
    let result = q.calcola_perimetro();
    debug!("Risultato perimetro quadrato: {}", result);
    Ok(result)
}

// Command per il calcolo dell'area del rettangolo (usa ileana-lib)
#[tauri::command]
#[instrument]
fn calcola_area_rettangolo(base: f64, altezza: f64) -> f64 {
    debug!("Calcolando area rettangolo con base: {}, altezza: {}", base, altezza);
    let r = Rettangolo { base, altezza };
    let result = r.calcola_area();
    debug!("Risultato area rettangolo: {}", result);
    result
}

// Command per il calcolo del perimetro del rettangolo (usa ileana-lib)
#[tauri::command]
#[instrument]
fn calcola_perimetro_rettangolo(base: f64, altezza: f64) -> f64 {
    debug!("Calcolando perimetro rettangolo con base: {}, altezza: {}", base, altezza);
    let r = Rettangolo { base, altezza };
    let result = r.calcola_perimetro();
    debug!("Risultato perimetro rettangolo: {}", result);
    result
}

// Command per il calcolo dell'area del cerchio (usa ileana-lib)
#[tauri::command]
#[instrument]
fn calcola_area_cerchio(raggio: f64) -> f64 {
    debug!("Calcolando area cerchio con raggio: {}", raggio);
    let c = Cerchio { raggio };
    let result = c.calcola_area();
    debug!("Risultato area cerchio: {}", result);
    result
}

// Command per il calcolo della circonferenza del cerchio (usa ileana-lib)
#[tauri::command]
#[instrument]
fn calcola_perimetro_cerchio(raggio: f64) -> f64 {
    debug!("Calcolando circonferenza cerchio con raggio: {}", raggio);
    let c = Cerchio { raggio };
    let result = c.calcola_perimetro();
    debug!("Risultato circonferenza cerchio: {}", result);
    result
}

// Command per il calcolo dell'area del triangolo (usa ileana-lib)
#[tauri::command]
#[instrument]
fn calcola_area_triangolo(lato1: f64, lato2: f64, lato_base: f64, altezza: f64) -> f64 {
    debug!("Calcolando area triangolo con lati: {}, {}, {}, altezza: {}", lato1, lato2, lato_base, altezza);
    let t = Triangolo { lato1, lato2, lato_base, altezza };
    let result = t.calcola_area();
    debug!("Risultato area triangolo: {}", result);
    result
}

// Command per il calcolo del perimetro del triangolo (usa ileana-lib)
#[tauri::command]
#[instrument]
fn calcola_perimetro_triangolo(lato1: f64, lato2: f64, lato_base: f64, altezza: f64) -> f64 {
    debug!("Calcolando perimetro triangolo con lati: {}, {}, {}, altezza: {}", lato1, lato2, lato_base, altezza);
    let t = Triangolo { lato1, lato2, lato_base, altezza };
    let result = t.calcola_perimetro();
    debug!("Risultato perimetro triangolo: {}", result);
    result
}

/// Configura il sistema di logging con tracing
/// Usa formato JSON per debug, formato compatto per release
fn setup_logging() {
    // Crea un file rotante per i log
    let file_appender = rolling::daily("./logs", "ileana-app.log");
    
    // Configura il livello di log: DEBUG in debug mode, INFO in release
    let filter_layer = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            if cfg!(debug_assertions) {
                EnvFilter::new("debug")
            } else {
                EnvFilter::new("info")
            }
        });

    // Configura il subscriber
    let subscriber = tracing_subscriber::registry()
        .with(filter_layer);

    if cfg!(debug_assertions) {
        // Formato JSON per debug (ricco di dettagli)
        let json_layer = fmt::layer()
            .json()
            .with_file(true)
            .with_line_number(true)
            .with_target(true)
            .with_thread_ids(true);
        
        // Layer per file in formato JSON
        let file_layer = fmt::layer()
            .with_writer(file_appender)
            .json()
            .with_file(true)
            .with_line_number(true);
        
        subscriber
            .with(json_layer)
            .with(file_layer)
            .init();
    } else {
        // Formato compatto per release
        let compact_layer = fmt::layer()
            .compact()
            .without_time()
            .with_target(false);
        
        // Layer per file in formato JSON
        let file_layer = fmt::layer()
            .with_writer(file_appender)
            .json()
            .with_file(true)
            .with_line_number(true);
        
        subscriber
            .with(compact_layer)
            .with(file_layer)
            .init();
    }

    info!("Logging configured - Debug mode: {}", cfg!(debug_assertions));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[instrument]
pub fn run() {
    // Inizializza il sistema di logging
    setup_logging();
    
    info!("Starting Ileana App");
    debug!("Debug logging is active");
    
    // Log dettagliato dell'ambiente
    debug!("Running in debug mode: {}", cfg!(debug_assertions));
    debug!("Target OS: {}", std::env::consts::OS);
    debug!("Target architecture: {}", std::env::consts::ARCH);
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 3. REGISTRAZIONE DI TUTTI I COMMANDS
        .invoke_handler(tauri::generate_handler![
            greet,                      // Manteniamo il command originale
            calcola_area_quadrato,      // Registriamo l'Area del quadrato
            calcola_perimetro_quadrato, // Registriamo il Perimetro del quadrato
            calcola_area_rettangolo,    // Registriamo l'Area del rettangolo
            calcola_perimetro_rettangolo, // Registriamo il Perimetro del rettangolo
            calcola_area_cerchio,      // Registriamo l'Area del cerchio
            calcola_perimetro_cerchio,  // Registriamo la Circonferenza del cerchio
            calcola_area_triangolo,    // Registriamo l'Area del triangolo
            calcola_perimetro_triangolo // Registriamo il Perimetro del triangolo
        ])
        .run(tauri::generate_context!())
        .map_err(|e| {
            error!("Failed to start Tauri application: {}", e);
            e
        })
        .expect("error while running tauri application");
    
    info!("Ileana App started successfully");
}