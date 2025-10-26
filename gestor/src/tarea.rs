use serde::{Deserialize, Serialize};
//importa los tipos de fecha y hora de la biblioteca chrono
//DateTime es una fecha con hora, y Utc es la zona horaria universal
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Estado {
    Pendiente,
    EnProgreso,
    Realizada,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tarea {
    pub id: u32,
    pub descripcion: String,
    pub estado: Estado,
    
    //se establece una vez al crear la tarea
    pub fecha_creacion: DateTime<Utc>,
    
    //se actualiza cada vez que se llama a actualizar_estado
    pub fecha_modificacion: DateTime<Utc>,
    
    //es un Option porque solo tendra valor Some si el estado es Realizada si no None
    pub fecha_realizacion: Option<DateTime<Utc>>,
}

impl Tarea {
    //inicialice las fechas
    pub fn nueva(id: u32, descripcion: String) -> Tarea {
        //obtenemos la fecha y hora exacta de este momento
        let ahora = Utc::now(); 
        
        Tarea {
            id,
            descripcion,
            estado: Estado::Pendiente,
            //al crear, la fecha de creacion y modificacion son la misma
            fecha_creacion: ahora,
            fecha_modificacion: ahora,
            //aun no esta terminada, asi que es None
            fecha_realizacion: None,
        }
    }

    //modificamos actualizar_estado para que maneje las fechas
    pub fn actualizar_estado(&mut self, nuevo_estado: Estado) {
        self.estado = nuevo_estado.clone(); //.clone() para poder usarla en el match
        
        //cada que actualiza, actualiza fecha de modificacion
        self.fecha_modificacion = Utc::now();

        //verifica el estado para saber que hacer con la fecha_realizacion
        match nuevo_estado {
            Estado::Realizada => {
                //si la tarea se marca como realizada y no tenia ya una fecha, establece la fecha de realizacion ahora
                if self.fecha_realizacion.is_none() {
                    self.fecha_realizacion = Some(self.fecha_modificacion);
                }
            }
            _ => {
                //si la tarea se marca como Pendiente o EnProgreso, significa que ya no esta terminada, borra la fecha
                self.fecha_realizacion = None;
            }
        }
    }
}