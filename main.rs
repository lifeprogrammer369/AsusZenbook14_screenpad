use std::io::{self, Write};
use std::process::Command;
use std::fs;
use std::path::Path;

// CONFIGURACION - AJUSTAR SEGUN EL SISTEMA
const SCREENPAD_NAME: &str = "HDMI-2";  // nombre del ScreenPad en xrandr
const MAIN_DISPLAY_NAME: &str = "eDP-1";   // nombre de la pantalla principal

fn main() {
    println!("\nControl ASUS ScreenPad - UX435EG\n");
    
    loop {
        println!("2. Cambiar resolución");
        println!("3. Ajustar escala");
        println!("4. Apagar ScreenPad");
        println!("5. Encender ScreenPad");
        println!("6. Salir");
        print!("Seleccione opción: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => break,
            "2" => change_resolution(SCREENPAD_NAME),
            "3" => adjust_scale(SCREENPAD_NAME),
            "4" => set_screenpad_power(false),
            "5" => set_screenpad_power(true),
            "6" => break,
            _ => println!("Opción inválida!"),
        }
    }
}


/// apaga o enciende el ScreenPad
fn set_screenpad_power(on: bool) {
    let state = if on { "--auto" } else { "--off" };
    
    let status = Command::new("xrandr")
        .args(&["--output", SCREENPAD_NAME, state])
        .status();
    
    match status {
        Ok(_) => println!("ScreenPad {}", if on { "encendido" } else { "apagado" }),
        Err(_) => println!("Error: Verifique el nombre del ScreenPad"),
    }
}

/// cambia la resolución del ScreenPad
fn change_resolution(output: &str) {
    let resolutions = Command::new("sh")
        .arg("-c")
        .arg(&format!("xrandr | grep -A2 '{} connected' | tail -n 2
", output))
        .output()
        .expect("Fallo al ejecutar xrandr");

    if !resolutions.status.success() {
        println!("No se encontraron resoluciones disponibles");
        return;
    }

    let res_str = String::from_utf8_lossy(&resolutions.stdout);
    let modes: Vec<&str> = res_str.split_whitespace().collect();

    println!("\nResoluciones disponibles:");
    for (i, mode) in modes.iter().enumerate() {
        if mode.contains('x') {
            println!("{}. {}", i + 1, mode);
        }
    }

    print!("Seleccione resolución: ");
    io::stdout().flush().unwrap();
    let mut sel = String::new();
    io::stdin().read_line(&mut sel).unwrap();
    
    if let Ok(idx) = sel.trim().parse::<usize>() {
        if idx > 0 && idx <= modes.len() {
            let res = modes[idx - 1];
            Command::new("xrandr")
                .args(&["--output", output, "--mode", res])
                .status()
                .expect("Fallo al cambiar resolución");
            println!("Resolución cambiada: {}", res);
            return;
        }
    }
    println!("Selección inválida!");
}

/// ajusta la escala del contenido
fn adjust_scale(output: &str) {
    print!("Ingrese porcentaje de escala (50-200): ");
    io::stdout().flush().unwrap();
    let mut scale = String::new();
    io::stdin().read_line(&mut scale).unwrap();
    
    if let Ok(percent) = scale.trim().parse::<f64>() {
        if percent >= 50.0 && percent <= 200.0 {
            let factor = percent / 100.0;
            Command::new("xrandr")
                .args(&[
                    "--output", output,
                    "--scale",
                    &format!("{}x{}", factor, factor)
                ])
                .status()
                .expect("Fallo al ajustar escala");
            println!("Escala ajustada: {}%", percent);
            return;
        }
    }
    println!("Valor inválido! Use valores entre 50-200");
}
