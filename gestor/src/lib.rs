use serde::{Deserialize, Serialize};

pub mod tarea;
pub mod manejador_archivos;

//importar lo que necesitamos
use tarea::{Tarea, Estado};
use manejador_archivos::{leer_gestor, escribir_gestor};

//estructura que envuelve y se guarda en json
#[derive(Serialize, Deserialize, Default)]
pub struct GestorDeTareas {
    pub tareas: Vec<Tarea>,
    pub ultimo_id: u32, //contador que nunca se reinicia
}

//funciones ahora reciben una referencia al GestorDeTareas completo
pub fn agregar_tarea(gestor: &mut GestorDeTareas, descripcion: String) {
    let nuevo_id = gestor.ultimo_id + 1; //incrementa el contador guardado en el gestor
    gestor.ultimo_id = nuevo_id; //actualiza el contador en el gestor
    let nueva_tarea = Tarea::nueva(nuevo_id, descripcion); //crea la tarea con este nuevo id
    println!("Tarea agregada: [ID: {}] {}", nueva_tarea.id, nueva_tarea.descripcion);
    gestor.tareas.push(nueva_tarea); //añadir la tarea a la lista dentro del gestor
}

//accede a la lista de tareas a traves de gestor.tareas
pub fn listar_tareas(gestor: &GestorDeTareas, filtro: Option<&str>) {
    println!("\n--- Lista de Tareas  ---");

    let tareas_a_mostrar: Vec<&Tarea> = match filtro {
        Some("pendientes") => gestor.tareas.iter()
        .filter(|t| t.estado == Estado::Pendiente).collect(),
        Some("en-progreso") => gestor.tareas.iter()
        .filter(|t| t.estado == Estado::EnProgreso).collect(),
        Some("realizadas") => gestor.tareas.iter()
        .filter(|t| t.estado == Estado::Realizada).collect(),
        _ => gestor.tareas.iter().collect(),
    };

    if tareas_a_mostrar.is_empty() {
        println!("No hay tareas para mostrar con ese filtro");
        return; //sale de la funcion si no hay nada que mostrar
    }

    //formato de tabla
    // {:<N} significa alinear a la izquierda, con N caracteres de ancho
    println!("{:<4} | {:<11} | {:<35} | {:<10} | {:<10}",
             "ID", "ESTADO", "DESCRIPCIÓN", "CREACIÓN", "TERMINADA");
    
    //imprime una linea separadora que coincida con el ancho
    println!("{:-<4} | {:-<11} | {:-<35} | {:-<10} | {:-<10}",
             "-", "-", "-", "-", "-");

    //itera y muestra cada tarea en una fila formateada
    for tarea in tareas_a_mostrar {
        
        //formate el estado a texto
        let estado_str = format!("{:?}", tarea.estado);
        
        //trunca la descripcion si es muy larga (33 + 2 puntos = 35)
        let mut desc_corta = tarea.descripcion.clone();
        if desc_corta.len() > 33 {
            desc_corta.truncate(33);
            desc_corta.push_str(".."); //".." para indicar que esta cortada
        }

        //formatea las fechas a un formato simple año mes dia
        let formato_fecha = "%Y-%m-%d";
        let creacion = tarea.fecha_creacion.format(formato_fecha).to_string();
        
        //fecha de realizacion opcional
        let realizada = match tarea.fecha_realizacion {
            Some(fecha) => fecha.format(formato_fecha).to_string(),
            None => "---".to_string(), //si no hay fecha, muestra ---
        };

        //imprime la fila con el mismo formato que la cabecera
        println!("{:<4} | {:<11} | {:<35} | {:<10} | {:<10}",
                 tarea.id, estado_str, desc_corta, creacion, realizada);
    }
}

//busca la tarea 
pub fn actualizar_estado_tarea(gestor: &mut GestorDeTareas, id: u32, estado: Estado) {
    if let Some(tarea) = gestor.tareas.iter_mut().find(|t| t.id == id) {
        tarea.actualizar_estado(estado.clone()); // Se clona el estado
        println!("Tarea {} actualizada a: {:?}", id, tarea.estado);
    } else {
        println!("Error: No se encontro ninguna tarea con el ID: {}", id);
    }
}

//elimina la tarea 
pub fn eliminar_tarea(gestor: &mut GestorDeTareas, id: u32) {
    let tamano_inicial = gestor.tareas.len(); 
    gestor.tareas.retain(|t| t.id != id); //elimina la tarea de gestor.tareas

    if gestor.tareas.len() < tamano_inicial {
        println!("Tarea con ID {} eliminada", id);
    } else {
        println!("Error: No se encontró ninguna tarea con el ID: {}", id);
    }
}

//muestra la ayuda
pub fn mostrar_ayuda() {
    println!("\nBienvenido al Gestor de Tareas");
    println!("Uso: gestor <comando> [argumentos]\n");
    println!("Comandos:");
    println!("  agregar <descripción>      - Agrega una nueva tarea.");
    println!("  listar [filtro]            - Muestra las tareas. Filtros: 'pendientes', 'en-progreso', 'realizadas'.");
    println!("  actualizar-estado <ID> <estado> - Cambia el estado de una tarea.");
    println!("  eliminar <ID>              - Borra una tarea por su ID.");
}

//funcion principal 
pub fn ejecutar() {
    let argumentos: Vec<String> = std::env::args().collect();

    if argumentos.len() < 2 {
        mostrar_ayuda();
        return;
    }

    let comando = &argumentos[1];

    let mut gestor = match leer_gestor() {
        Ok(gestor) => gestor,
        Err(e) => {
            eprintln!("Error fatal al leer el archivo de tareas: {}", e);
            return;
        }
    };

    match comando.as_str() {
        "agregar" => {
            if argumentos.len() < 3 {
                println!("Error: Debes proporcionar una descripcion para la tarea");
            } else {
                let descripcion = argumentos[2..].join(" ");
                agregar_tarea(&mut gestor, descripcion);
            }
        }
        "listar" => {
            let filtro = argumentos.get(2).map(|s| s.as_str());
            listar_tareas(&gestor, filtro);
        }
        "actualizar-estado" => {
            if argumentos.len() < 4 {
                println!("Uso correcto: actualizar-estado <ID> <pendiente|en-progreso|realizada>");
            } else {
                let id = match argumentos[2].parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Error: El ID debe ser un numero valido");
                        return;
                    }
                };
                let estado = match argumentos[3].as_str() {
                    "pendiente" => Estado::Pendiente,
                    "en-progreso" => Estado::EnProgreso,
                    "realizada" => Estado::Realizada,
                    _ => {
                        println!("Error: Estado no valido. Usa: pendiente, en-progreso, realizada");
                        return;
                    }
                };
                actualizar_estado_tarea(&mut gestor, id, estado);
            }
        }
        "eliminar" => {
            if argumentos.len() < 3 {
                println!("Uso correcto: eliminar <ID>");
            } else {
                let id = match argumentos[2].parse::<u32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Error: El ID debe ser un numero valido");
                        return;
                    }
                };
                eliminar_tarea(&mut gestor, id);
            }
        }
        _ => mostrar_ayuda(),
    }

    if let Err(e) = escribir_gestor(&gestor) {
        eprintln!("Error al guardar las tareas: {}", e);
    }
}
