package task

import "fmt"

//los estados se hacen const
const (
	EstadoIniciado   = "Iniciado"
	EstadoEnProceso  = "En Proceso"
	EstadoCompletado = "Completado"
)

//defino la estructura de como va a ser tarea
type Tarea struct {
	ID          int    `json:"id"`
	Titulo      string `json:"title"`
	Descripcion string `json:"description"`
	Estado string `json:"status"`
}

//String de como se muestra la tarea
func (t Tarea) String() string {
	icono := "○" //iniciado
	if t.Estado == EstadoEnProceso {
		icono = "→" //proceso
	} else if t.Estado == EstadoCompletado {
		icono = "✔" //completado
	}
	return fmt.Sprintf("[%s] %d: %s (%s)\n   Descripción: %s\n", icono, t.ID, t.Titulo, t.Estado, t.Descripcion)
}