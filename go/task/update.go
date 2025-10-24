package task

import "fmt"

func (gt *GestorTareas) ActualizarTarea(id int, titulo, descripcion, estado string) error {
	indiceTarea := -1
	for i, tarea := range gt.Tareas {
		if tarea.ID == id {
			indiceTarea = i
			break
		}
	}
	if indiceTarea == -1 {
		return fmt.Errorf("tarea con ID %d no encontrada", id)
	}
	if estado != EstadoIniciado && estado != EstadoEnProceso && estado != EstadoCompletado {
		return fmt.Errorf("estado no válido: '%s'. Use 'Iniciado', 'En Proceso' o 'Completado'", estado)
	}
	gt.Tareas[indiceTarea].Titulo = titulo
	gt.Tareas[indiceTarea].Descripcion = descripcion
	gt.Tareas[indiceTarea].Estado = estado
	return gt.guardarTareas()
}
