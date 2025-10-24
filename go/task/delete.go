package task

import "fmt"

func (gt *GestorTareas) EliminarTarea(id int) error {
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
	gt.Tareas = append(gt.Tareas[:indiceTarea], gt.Tareas[indiceTarea+1:]...)
	return gt.guardarTareas()
}