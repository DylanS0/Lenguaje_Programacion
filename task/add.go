package task

import "fmt"

func (gt *GestorTareas) AgregarTarea(titulo, descripcion string) (int, error) {
	if titulo == "" {
		return 0, fmt.Errorf("el título no puede estar vacío")
	}
	nuevaTarea := Tarea{
		ID:          gt.obtenerSiguienteID(),
		Titulo:      titulo,
		Descripcion: descripcion,
		Estado:      EstadoIniciado,
	}
	gt.Tareas = append(gt.Tareas, nuevaTarea)
	if err := gt.guardarTareas(); err != nil {
		return 0, err
	}
	return nuevaTarea.ID, nil
}