//zona de importacion
use std::io; //para leer la entrada del usuario

fn main() {
    println!("--- Generador de secuencia de fibonacci en rust ---");
    println!("Calcula los primeros 'n' numeros de la secuencia");

    //variable mutable para almacenar el numero de elementos que el usuario desea generar
    let mut n_input = String::new();
    let n: u32;

    //bucle para asegurar una entrada valida
    loop {
        println!("\n¿Cuantos numeros de la secuencia de fibonacci desea generar? (max. 93 para u64):");
        n_input.clear(); //limpia la entrada en cada intento

        io::stdin()
            .read_line(&mut n_input)
            .expect("Fallo al leer la linea de entrada");

        //convierte la entrada a un entero sin signo (u32) y usamos match para gestionar el resultado
        match n_input.trim().parse::<u32>() {
            Ok(num) => {
                //limita la entrada a 93, pq secuencia crece muy rapido, u64 (entero sin signo de 64 bits) desborda despues del 93
                if num == 0 {
                    println!("Por favor, ingresa un numero mayor que 0");
                    continue;
                } else if num > 93 {
                    println!("El maximo soportado es 93 para evitar desbordamiento (overflow) de u64");
                    continue;
                }
                n = num; //asigna el valor valido y sale del bucle
                break;
            },
            Err(_) => {
                println!("Entrada no valida. Por favor, ingresa un numero entero");
                continue;
            }
        }
    }

    //loigica
    if n == 1 {
        println!("\n[0]");
        return; //caso base: si n es 1, solo imprimimos el 0 y terminamos
    }

    //inicializa los dos primeros numeros de la secuencia
    //usa u64 para evitar el desbordamiento
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    //imprime el primer elemento de la secuencia
    print!("[{}", a);

    //itera desde 2 hasta n = el número de elementos
    //el for loop en rust itera sobre un rango (2..n) no inclusivp o (2..=n) si es inclusivo
    //se quiere generar n numeros, y ya tenemos 1 (a=0), por lo que se necesita n-1 iteraciones
    //usa el rango 1..(n as usize) para la iteracion
    for _i in 1..(n as usize) {
        //imprime el siguiente elemento (b)
        print!(", {}", b);
        //calcula el siguiente termino: c = a + b
        let c = a + b;
        
        //actualiza los valores para la siguiente iteracion
        //el antiguo b se convierte en el nuevo a
        a = b;
        //el nuevo termino c se convierte en el nuevo b
        b = c;
    }

    println!("]"); //cierre de la secuencia
    println!("\nSecuencia de fibonacci generada exitosamente ({} elementos)", n);
}
