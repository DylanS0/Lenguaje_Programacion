//zona de importacion 
use std::io;       //para leer entrada del usuario
use rand::Rng; //para generar numeros aleatorios

//constante inmutable para el limite de intentos
const MAX_INTENTOS: u8 = 3;

fn main() {
    println!("--- Juego de adivinar el numero en rust ---");
    println!("El objetivo es adivinar un numero entre 1 y 20");
    println!("Tiene 3 intentos para adivinar el numero correcto");
    
    //'thread_rng() obtiene el generador de numeros aleatorios local al hilo
    // gen_range(1..=20) genera un entero aleatorio inclusivo entre 1 y 20
    let numero_secreto: u32 = rand::thread_rng().gen_range(1..=20); //genera el numero secreto

    //variable mutable para contar los intentos
    let mut intentos_realizados = 0;

    //loop principal del juego
    loop {
        //incrementa el contador de intentos al inicio de cada iteracion
        intentos_realizados += 1;

        //comprurba si el usuario ha agotado sus 3 intentos
        if intentos_realizados > MAX_INTENTOS as u32 {
             println!("\n¡Fin del juego! Agotaste tus {} intentos", MAX_INTENTOS);
             println!("El numero secreto era: {}", numero_secreto);
             break; //rompe loop
        }

        let intentos_restantes = MAX_INTENTOS as u32 - (intentos_realizados - 1);
        println!("\n--- Intento {} de {} ---", intentos_realizados, MAX_INTENTOS);
        println!("Te quedan {} intentos restantes", intentos_restantes);
        println!("Por favor, ingresa tu suposicion:");

        //variable mutable para almacenar la entrada del usuario
        let mut suposicion = String::new();

        io::stdin()
            .read_line(&mut suposicion)
            .expect("Fallo al leer la linea de entrada");

        //manejo de la entrada y conversion de tipos 
        //parsear la suposicion String a un entero sin signo de 32 bits (u32)
        let suposicion: u32 = match suposicion.trim().parse() {
            // Caso Ok(num) La conversion se hizo
            Ok(num) => num,
            // Caso Err(_) La conversion fallo (ingreso texto, etc)
            Err(_) => {
                println!("¡Entrada no valida! Debe ingresar un numero");
                //si la entrada es invalida, restamos para anular el intento que se acaba de registrar
                intentos_realizados -= 1;
                continue; //saltar a la siguiente iteracion del loop
            }
        };

        //logica
        if suposicion < numero_secreto {
            println!("Demasiado pequeño ¡Intentalo de nuevo!");
        } else if suposicion > numero_secreto {
            println!("Demasiado grande ¡Intentalo de nuevo!");
        } else {
            // Caso en que suposicion == numero_secreto
            println!("\n¡Felicidades! ¡Adivinaste el numero {} en {} intentos!", numero_secreto, intentos_realizados);
            break; //usuario gano, salimos del loop
        }
    }
}

//con match exclusivo full ai

/* // --- 1. Importación de Módulos Necesarios ---
use std::io;       // Para leer la entrada del usuario.
use std::cmp::Ordering; // Para la función de comparación (less, greater, equal).
// Para la generación de números aleatorios, simulamos la importación de la librería 'rand'.
// En un proyecto real, se requiere añadir 'rand' a Cargo.toml.
use rand::Rng; // El trait Rng contiene el método gen_range.

// --- 2. Constantes ---
// Una constante inmutable, escrita en mayúsculas, para el límite de intentos.
const MAX_INTENTOS: u8 = 3;

// --- 3. Función Principal (main) ---
fn main() {
    println!("--- Juego de Adivinar el Número en Rust (3 Intentos) ---");
    println!("El objetivo es adivinar un número entre 1 y 20.");

    // Generamos el número secreto.
    // 'thread_rng()' obtiene el generador de números aleatorios local al hilo.
    // 'gen_range(1..=20)' genera un entero aleatorio inclusivo entre 1 y 20.
    let numero_secreto = rand::thread_rng().gen_range(1..=20);

    // Variable mutable para contar los intentos.
    let mut intentos_realizados = 0;

    // Loop principal del juego.
    loop {
        // Incrementamos el contador de intentos al inicio de cada iteración.
        intentos_realizados += 1;

        // Comprobamos si el usuario ha agotado sus 3 intentos.
        // Utilizamos un match exclusivo para gestionar el flujo de intentos/fin del juego.
        match intentos_realizados.cmp(&MAX_INTENTOS) {
            // Caso 'Greater': Ya gastó los 3 intentos.
            Ordering::Greater => {
                println!("\n¡Fin del juego! Agotaste tus {} intentos.", MAX_INTENTOS);
                println!("El número secreto era: {}", numero_secreto);
                break; // Salimos del loop.
            }
            // Caso 'Less' o 'Equal': Todavía tiene intentos. Continuamos.
            _ => {
                let intentos_restantes = MAX_INTENTOS - (intentos_realizados - 1);
                println!("\n--- Intento {} de {} ---", intentos_realizados, MAX_INTENTOS);
                println!("Te quedan {} intentos restantes.", intentos_restantes);
            }
        }

        println!("Por favor, ingresa tu suposición:");

        // Variable mutable para almacenar la entrada del usuario.
        let mut suposicion = String::new();

        io::stdin()
            .read_line(&mut suposicion)
            .expect("Fallo al leer la línea de entrada.");

        // --- Manejo de la Entrada y Conversión de Tipos (con match) ---
        // Intentamos parsear la suposición (String) a un entero sin signo de 32 bits (u32).
        let suposicion: u32 = match suposicion.trim().parse() {
            // Caso 'Ok(num)': La conversión fue exitosa.
            Ok(num) => num,
            // Caso 'Err(_)': La conversión falló (ej. ingresó texto).
            Err(_) => {
                println!("¡Entrada no válida! Debes ingresar un número.");
                // Si la entrada es inválida, restamos 1 para "anular" el intento que se acaba de registrar.
                intentos_realizados -= 1;
                continue; // Saltar a la siguiente iteración del loop.
            }
        };

        // --- Lógica del Juego (match exclusivo para la comparación) ---
        // Comparamos la suposición con el número secreto.
        // El método 'cmp' devuelve un 'Ordering' (Less, Greater, Equal).
        match suposicion.cmp(&numero_secreto) {
            Ordering::Less => println!("Demasiado pequeño. ¡Inténtalo de nuevo!"),
            Ordering::Greater => println!("Demasiado grande. ¡Inténtalo de nuevo!"),
            Ordering::Equal => {
                println!("\n¡Felicidades! ¡Adivinaste el número {} en {} intentos!", numero_secreto, intentos_realizados);
                break; // El usuario ganó, salimos del loop.
            }
        }
    }
}
 */