use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use super::GestorDeTareas;

const RUTA_ARCHIVO: &str = "tareas.json";

//devuelve un GestorDeTareas
pub fn leer_gestor() -> Result<GestorDeTareas, io::Error> {
    let mut archivo = OpenOptions::new().read(true).write(true).create(true).open(RUTA_ARCHIVO)?;

    let mut contenido = String::new();
    archivo.read_to_string(&mut contenido)?;

    if contenido.is_empty() {
       /*si el archivo esta vacio, crea un gestor nuevo usando default()
        default() viene del #[derive(Default)] se añade en lib.rs
        crea un gestor con una lista vacia y ultimo_id = 0 */
        return Ok(GestorDeTareas::default());
    }

    //convierte el json en nuestra estructura GestorDeTareas
    //si falla, devuelve uno por defecto para no romperlo
    let gestor: GestorDeTareas = serde_json::from_str(&contenido)
        .unwrap_or_else(|_| GestorDeTareas::default());

    Ok(gestor)
}

//ahora recibe un GestorDeTareas para guardarlo
pub fn escribir_gestor(gestor: &GestorDeTareas) -> Result<(), io::Error> {
    //convertimos la estructura completa tareas + ultimo_id a json
    let datos_json = serde_json::to_string_pretty(gestor)?;

    let mut archivo = OpenOptions::new().write(true).create(true).truncate(true).open(RUTA_ARCHIVO)?;

    archivo.write_all(datos_json.as_bytes())?;

    Ok(())
}