package task

import (
	"encoding/json"
	"fmt"
	"os"
)

//define el formato del archivo json
type archivoTareas struct {
	UltimoID int     `json:"ultimo_id"`
	Tareas   []Tarea `json:"tasks"`
}

//gestiona las tareas
type GestorTareas struct {
	Tareas      []Tarea
	rutaArchivo string
	ultimoID    int
}

//crea un nuevo gestor
func NuevoGestorTareas(rutaArchivo string) (*GestorTareas, error) {
	gt := &GestorTareas{
		rutaArchivo: rutaArchivo,
		ultimoID:    0,
	}
	if err := gt.cargarTareas(); err != nil {
		return nil, err
	}
	return gt, nil
}

//lee el archivo json
func (gt *GestorTareas) cargarTareas() error {
	_, err := os.Stat(gt.rutaArchivo)
	if os.IsNotExist(err) {
		gt.Tareas = []Tarea{}
		gt.ultimoID = 0
		return nil
	}

	data, err := os.ReadFile(gt.rutaArchivo)
	if err != nil {
		return err
	}
	if len(data) == 0 {
		gt.Tareas = []Tarea{}
		gt.ultimoID = 0
		return nil
	}

	var archivo archivoTareas
	if err := json.Unmarshal(data, &archivo); err == nil {
		gt.Tareas = archivo.Tareas
		gt.ultimoID = archivo.UltimoID
		return nil
	}

	// tareas antiguas 
	var tareasAntiguas []Tarea
	if err := json.Unmarshal(data, &tareasAntiguas); err != nil {
		return fmt.Errorf("archivo %s inválido", gt.rutaArchivo)
	}
	gt.Tareas = tareasAntiguas
	gt.ultimoID = 0
	for _, t := range tareasAntiguas {
		if t.ID > gt.ultimoID {
			gt.ultimoID = t.ID
		}
	}
	return nil
}

// guarda en el nuevo formato
func (gt *GestorTareas) guardarTareas() error {
	archivo := archivoTareas{
		UltimoID: gt.ultimoID,
		Tareas:   gt.Tareas,
	}
	data, err := json.MarshalIndent(archivo, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(gt.rutaArchivo, data, 0644)
}

//devuelve un id nico
func (gt *GestorTareas) obtenerSiguienteID() int {
	gt.ultimoID++
	return gt.ultimoID
}