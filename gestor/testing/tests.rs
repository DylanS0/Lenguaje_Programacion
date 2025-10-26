use gestor::tarea::{Tarea, Estado};
use gestor::{agregar_tarea, eliminar_tarea, actualizar_estado_tarea, GestorDeTareas};

//modulo para manejar tiempo y dormir el hilo
use std::thread;
use std::time::Duration;

#[test]
fn test_agregar_tarea_y_verificar_fechas() {
    //preparcion
    let mut gestor = GestorDeTareas::default();
    
    //actuacion 
    agregar_tarea(&mut gestor, "Tarea con fechas".to_string());

    //asercion
    assert_eq!(gestor.tareas.len(), 1);
    assert_eq!(gestor.ultimo_id, 1);
    
    let tarea = &gestor.tareas[0];
    assert_eq!(tarea.id, 1);
    
    //verificar que las fechas se hayan establecido correctamente
    assert!(tarea.fecha_creacion <= chrono::Utc::now());
    //cuando se crea la fecha de modificacion es la misma que la de creacion
    assert_eq!(tarea.fecha_creacion, tarea.fecha_modificacion);
    //no debe tener fecha de realizacion
    assert!(tarea.fecha_realizacion.is_none());
}

#[test]
fn test_actualizar_estado_y_fechas() {
    //preparcion
    let mut gestor = GestorDeTareas {
        tareas: vec![Tarea::nueva(1, "Probar fechas de actualizacion".to_string())],
        ultimo_id: 1,
    };
    
    let fecha_creacion_original = gestor.tareas[0].fecha_creacion;
    
    //duerme el hilo por 2 milisegundos
    // Esto garantiza que el prox Utc::now() sea diferente
    thread::sleep(Duration::from_millis(2));

    //marcamos como Realizada
    actualizar_estado_tarea(&mut gestor, 1, Estado::Realizada);

    //asercion
    let tarea_realizada = &gestor.tareas[0];
    assert_eq!(tarea_realizada.estado, Estado::Realizada);
    //la fecha de creacion nunca debe cambiar
    assert_eq!(tarea_realizada.fecha_creacion, fecha_creacion_original);
    //la fecha de modificacion debe ser mas reciente que la de creacion
    assert!(tarea_realizada.fecha_modificacion > fecha_creacion_original);
    //la fecha de realizacion debe existir y ser igual a la de modificacion
    assert!(tarea_realizada.fecha_realizacion.is_some());
    assert_eq!(tarea_realizada.fecha_realizacion.unwrap(), tarea_realizada.fecha_modificacion);

    //guarda la fecha de la primera modificacion
    let fecha_primera_mod = tarea_realizada.fecha_modificacion;
    
    //duerme el hilo otra vez
    thread::sleep(Duration::from_millis(2));
    
    //la devolvemos a pendiente
    actualizar_estado_tarea(&mut gestor, 1, Estado::Pendiente);
    
    //asercion 1
    let tarea_revertida = &gestor.tareas[0];
    assert_eq!(tarea_revertida.estado, Estado::Pendiente);
    //la fecha de modificacion debe haberse actualizado otra vez
    assert!(tarea_revertida.fecha_modificacion > fecha_primera_mod);
    //la fecha de realizacion debe haberse borrado ser None
    assert!(tarea_revertida.fecha_realizacion.is_none());
}

//verifica que la logica de id sigue funcionando
//incluso con las nuevas tareas que incluyen fechas
#[test]
fn test_id_no_se_repite_al_borrar_la_ultima_tarea() {
    let mut gestor = GestorDeTareas {
        tareas: vec![
            Tarea::nueva(1, "Tarea A".to_string()),
            Tarea::nueva(2, "Tarea B".tostring()),
        ],
        ultimo_id: 2,
    };

    eliminar_tarea(&mut gestor, 2);
    assert_eq!(gestor.tareas.len(), 1);
    assert_eq!(gestor.ultimo_id, 2); //el contador se mantiene

    agregar_tarea(&mut gestor, "Tarea C".to_string());
    
    assert_eq!(gestor.tareas.len(), 2);
    assert_eq!(gestor.ultimo_id, 3); //el contador sube a 3
    let ultima_tarea = gestor.tareas.last().unwrap();
    assert_eq!(ultima_tarea.id, 3); //el id es 3, no 2
}