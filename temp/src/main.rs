//importacion 
//modulo io de la libreria estandar std para manejar la entrada y salida de datos del usuario
use std::io;

//funciones de conversion de temperatura
//el tipo de entrada celsius es un flotante 64 bits (f64)
// La funcion retorna un f64
fn celsius_to_fahrenheit(celsius: f64) -> f64 { //funcion para convertir celsius a fahrenheit
    //la formula (C * 1.8) + 32
    (celsius * 1.8) + 32.0
}

//define una funcion para convertir fahrenheit a celsius
//la funcion tambien toma un f64 y retorna un f64
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    //la formula es: (F - 32) / 1.8
    (fahrenheit - 32.0) / 1.8
}

fn main() {
    println!("--- Conversor de temperaturas en rust ---");

    //loop infinito para mantener el programa corriendo hasta que el usuario decida salir
    loop {
        println!("\nSelecciona la conversion que deseas realizar:");
        println!("1. Celsius a Fahrenheit (C -> F)");
        println!("2. Fahrenheit a Celsius (F -> C)");
        println!("3. Salir");

        //variable mutable choice (mut) para almacenar la entrada del usuario
        //tipo String
        let mut choice = String::new();

        //lee la linea de entrada del usuario
        io::stdin()
            .read_line(&mut choice) //lee la entrada y la guarda en la variable choice
            .expect("Fallo al leer la línea de entrada."); //manejo basico de errores

        //match para manejar la logica de control
        // Usamos trim() para eliminar el salto de línea y espacios, y parse() para convertir a entero
        let choice = match choice.trim().parse::<u32>() {
            Ok(num) => num, //conversion es exitosa (Ok) usa el numero
            Err(_) => {     //si hay un error (ingresó texto, etc) muestra un mensaje y continua el loop
                println!("¡Opcion no valida! Por favor, ingresa 1, 2 o 3.");
                continue;   //vuelve al inicio del loop
            }
        };

        //si la opcion es 3, sale 
        if choice == 3 {
            println!("Saliendo del programa");
            break;
        }

        //si la opcion no es 1 o 2 volvemos a pedir la entrada
        if choice != 1 && choice != 2 {
            println!("Opcion fuera de rango");
            println!("Por favor, intenta de nuevo.");
            continue;
        }
        //entrada de datos
        println!("Ingresa la temperatura a convertir:");
        let mut temp_input = String::new();

        io::stdin()
            .read_line(&mut temp_input)
            .expect("Fallo al leer la temperatura");

        //convierte la entrada a un f64
        let temp: f64 = match temp_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("¡Error! La temperatura debe ser un numero valido");
                continue;
            }
        };

        //conversion y salida
        let result = if choice == 1 {
            //llama a la función C -> F
            let converted = celsius_to_fahrenheit(temp);
            format!("{}°C es igual a {:.2}°F", temp, converted)
        } else {
            //llama a la función F -> C
            let converted = fahrenheit_to_celsius(temp);
            format!("{}°F es igual a {:.2}°C", temp, converted)
        };

        println!("\n--- Resultado ---");
        println!("{}", result);
    }
}
