//para ejcutar pruebas: go test -v ./task o go test ./...
package task

import (
	"os"
	"path/filepath"
	"testing"
)

// prepararPrueba crea un archivo json temporal y un GestorTareas para las pruebas
func prepararPrueba(t *testing.T) (*GestorTareas, string, func()) {
	// Crear un directorio temporal para la base de datos de prueba
	dirTemporal, err := os.MkdirTemp("", "pruebas_gestor_*")
	if err != nil {
		t.Fatalf("No se pudo crear el directorio temporal: %v", err)
	}

	archivoTemporal := filepath.Join(dirTemporal, "test_tasks.json")

	// Crear el gestor apuntando al archivo temporal
	gestor, err := NuevoGestorTareas(archivoTemporal)
	if err != nil {
		t.Fatalf("No se pudo crear el GestorTareas: %v", err)
	}

	// La función de limpieza elimina el directorio temporal
	limpieza := func() {
		os.RemoveAll(dirTemporal)
	}

	return gestor, archivoTemporal, limpieza
}

func TestAddTask(t *testing.T) {
	gestor, _, limpieza := prepararPrueba(t)
	defer limpieza() // Asegurarse de que la limpieza se ejecute al final

	titulo := "Tarea de prueba"
	descripcion := "Descripción de prueba"
	id, err := gestor.AgregarTarea(titulo, descripcion)
	if err != nil {
		t.Fatalf("AgregarTarea falló: %v", err)
	}

	if id != 1 {
		t.Errorf("Se esperaba ID 1, pero se obtuvo %d", id)
	}

	tareas := gestor.ListarTareas()
	if len(tareas) != 1 {
		t.Fatalf("Se esperaba 1 tarea, pero se encontraron %d", len(tareas))
	}

	if tareas[0].Titulo != titulo || tareas[0].Descripcion != descripcion {
		t.Errorf("Los datos de la tarea no coinciden. Obtenido: %+v", tareas[0])
	}
	if tareas[0].Estado != EstadoIniciado {
		t.Errorf("Se esperaba el estado '%s', pero se obtuvo '%s'", EstadoIniciado, tareas[0].Estado)
	}
}

func TestDeleteTask(t *testing.T) {
	gestor, _, limpieza := prepararPrueba(t)
	defer limpieza()

	gestor.AgregarTarea("Tarea para borrar", "")
	tareas := gestor.ListarTareas()
	if len(tareas) != 1 {
		t.Fatal("La configuración de la prueba de borrado falló")
	}

	idABorrar := tareas[0].ID
	err := gestor.EliminarTarea(idABorrar)
	if err != nil {
		t.Fatalf("EliminarTarea falló: %v", err)
	}

	if len(gestor.ListarTareas()) != 0 {
		t.Error("La tarea no fue eliminada correctamente")
	}

	// Probar eliminar una tarea que no existe
	err = gestor.EliminarTarea(999)
	if err == nil {
		t.Error("Se esperaba un error al eliminar una tarea inexistente, pero no se obtuvo ninguno")
	}
}

func TestUpdateTask(t *testing.T) {
	gestor, _, limpieza := prepararPrueba(t)
	defer limpieza()

	id, _ := gestor.AgregarTarea("Título original", "Desc original")

	// --- PRUEBA DE ACTUALIZACION CORRECTA ---
	nuevoTitulo := "Título actualizado"
	nuevaDescripcion := "Desc actualizada"
	nuevoEstado := EstadoEnProceso

	err := gestor.ActualizarTarea(id, nuevoTitulo, nuevaDescripcion, nuevoEstado)
	if err != nil {
		t.Fatalf("ActualizarTarea falló: %v", err)
	}

	tareas := gestor.ListarTareas()
	if len(tareas) != 1 {
		t.Fatalf("El número de tareas cambió inesperadamente")
	}

	tareaActualizada := tareas[0]
	// Se comprueban los nuevos valores
	if tareaActualizada.Titulo != nuevoTitulo || tareaActualizada.Descripcion != nuevaDescripcion || tareaActualizada.Estado != nuevoEstado {
		t.Errorf("La tarea no se actualizó correctamente. Obtenido: %+v", tareaActualizada)
	}

	// --- PRUEBA DE ESTADO NO VALIDO ---
	estadoNoValido := "Pendiente"
	err = gestor.ActualizarTarea(id, nuevoTitulo, nuevaDescripcion, estadoNoValido)
	if err == nil {
		t.Errorf("Se esperaba un error al actualizar con estado no válido ('%s'), pero no se obtuvo ninguno", estadoNoValido)
	}

	// --- PRUEBA DE ACTUALIZAR TAREA INEXISTENTE ---
	err = gestor.ActualizarTarea(999, "a", "b", EstadoCompletado) // id que no existe
	if err == nil {
		t.Error("Se esperaba un error al actualizar una tarea inexistente, pero no se obtuvo ninguno")
	}
}

func TestPersistence(t *testing.T) {
	gestor, archivoBD, limpieza := prepararPrueba(t)
	defer limpieza()

	// Añadir una tarea y "cerrar" el gestor (simulado al perder la referencia)
	titulo := "Persistencia"
	descripcion := "Probar guardado y carga"
	gestor.AgregarTarea(titulo, descripcion) // Por defecto, Estado es 'Iniciado'

	// Crear un nuevo gestor que cargue desde el mismo archivo
	nuevoGestor, err := NuevoGestorTareas(archivoBD)
	if err != nil {
		t.Fatalf("Fallo al crear un nuevo gestor desde un archivo existente: %v", err)
	}

	tareas := nuevoGestor.ListarTareas()
	if len(tareas) != 1 {
		t.Fatalf("Se esperaba 1 tarea cargada, pero se encontraron %d", len(tareas))
	}

	if tareas[0].Titulo != titulo || tareas[0].Descripcion != descripcion || tareas[0].Estado != EstadoIniciado {
		t.Errorf("Los datos de la tarea cargada no coinciden o el estado por defecto es incorrecto. Obtenido: %+v", tareas[0])
	}
}

// Test para verificar que los IDs son irrepetibles tras eliminación y reinicio
func TestUniqueIDAfterDeleteAndRestart(t *testing.T) {
	gestor, archivoBD, limpieza := prepararPrueba(t)
	defer limpieza()

	// Agregar dos tareas
	id1, _ := gestor.AgregarTarea("Tarea 1", "")
	id2, _ := gestor.AgregarTarea("Tarea 2", "")

	if id1 != 1 || id2 != 2 {
		t.Fatalf("IDs iniciales incorrectos: %d, %d", id1, id2)
	}

	// Eliminar la tarea con ID más alto
	err := gestor.EliminarTarea(id2)
	if err != nil {
		t.Fatalf("No se pudo eliminar la tarea: %v", err)
	}

	// Agregar una nueva tarea → debe tener ID = 3
	id3, err := gestor.AgregarTarea("Tarea 3", "")
	if err != nil {
		t.Fatalf("No se pudo agregar la tercera tarea: %v", err)
	}
	if id3 != 3 {
		t.Errorf("Se esperaba ID 3, pero se obtuvo %d", id3)
	}

	// Reiniciar el gestor (simula reinicio del programa)
	nuevoGestor, err := NuevoGestorTareas(archivoBD)
	if err != nil {
		t.Fatalf("No se pudo reiniciar el gestor: %v", err)
	}

	// Agregar otra tarea → debe ser ID = 4
	id4, err := nuevoGestor.AgregarTarea("Tarea 4", "")
	if err != nil {
		t.Fatalf("No se pudo agregar la cuarta tarea: %v", err)
	}
	if id4 != 4 {
		t.Errorf("Se esperaba ID 4 tras reinicio, pero se obtuvo %d", id4)
	}
}