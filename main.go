package main

import (
	"flag"
	"fmt"
	"os"
	"strings"
	"task-manager/task" // ← Importa el paquete task
)

const archivoBD = "tasks.json"

func trunca(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen-3] + "..."
}

func main() {
	gestor, err := task.NuevoGestorTareas(archivoBD)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error al inicializar el gestor de tareas: %v\n", err)
		os.Exit(1)
	}

	comandoAgregar := flag.NewFlagSet("add", flag.ExitOnError)
	tituloAgregar := comandoAgregar.String("title", "", "Título de la tarea (obligatorio)")
	descAgregar := comandoAgregar.String("desc", "", "Descripción de la tarea")

	comandoActualizar := flag.NewFlagSet("update", flag.ExitOnError)
	idActualizar := comandoActualizar.Int("id", 0, "ID de la tarea a actualizar (obligatorio)")
	tituloActualizar := comandoActualizar.String("title", "", "Nuevo título")
	descActualizar := comandoActualizar.String("desc", "", "Nueva descripción")
	estadoActualizar := comandoActualizar.String("status", "", "Nuevo estado")

	comandoEliminar := flag.NewFlagSet("delete", flag.ExitOnError)
	idEliminar := comandoEliminar.Int("id", 0, "ID de la tarea a eliminar (obligatorio)")

	if len(os.Args) < 2 {
		imprimirAyuda()
		os.Exit(1)
	}

	switch os.Args[1] {
	case "add":
		comandoAgregar.Parse(os.Args[2:])
		if *tituloAgregar == "" {
			fmt.Fprintln(os.Stderr, "Error: --title es obligatorio")
			os.Exit(1)
		}
		id, err := gestor.AgregarTarea(*tituloAgregar, *descAgregar)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		fmt.Printf("Tarea añadida con ID: %d\n", id)

	case "list":
		tareas := gestor.ListarTareas()
		if len(tareas) == 0 {
			fmt.Println("No hay tareas.")
			return
		}
		fmt.Printf("%-4s %-20s %-40s %-12s\n", "ID", "TÍTULO", "DESCRIPCIÓN", "ESTADO")
		fmt.Println(strings.Repeat("-", 80))
		for _, t := range tareas {
			fmt.Printf("%-4d %-20s %-40s %-12s\n",
				t.ID,
				trunca(t.Titulo, 18),
				trunca(t.Descripcion, 38),
				t.Estado)
		}

	case "update":
		comandoActualizar.Parse(os.Args[2:])
		if *idActualizar == 0 {
			fmt.Fprintln(os.Stderr, "Error: --id es obligatorio")
			os.Exit(1)
		}
		// Buscar tarea para validar que existe (opcional, ya lo hace ActualizarTarea)
		err := gestor.ActualizarTarea(
			*idActualizar,
			*tituloActualizar,
			*descActualizar,
			*estadoActualizar,
		)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		fmt.Printf("Tarea %d actualizada.\n", *idActualizar)

	case "delete":
		comandoEliminar.Parse(os.Args[2:])
		if *idEliminar == 0 {
			fmt.Fprintln(os.Stderr, "Error: --id es obligatorio")
			os.Exit(1)
		}
		err := gestor.EliminarTarea(*idEliminar)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: %v\n", err)
			os.Exit(1)
		}
		fmt.Printf("Tarea %d eliminada.\n", *idEliminar)

	case "help":
		imprimirAyuda()

	default:
		imprimirAyuda()
		os.Exit(1)
	}
}

func imprimirAyuda() {
	fmt.Println("Gestor de Tareas en Go")
	fmt.Println("Uso: task-manager <comando> [argumentos]")
	fmt.Println("\nComandos:")
	fmt.Println("  add      --title=\"...\" [--desc=\"...\"]")
	fmt.Println("  list")
	fmt.Println("  update   --id=N [--title] [--desc] [--status]")
	fmt.Println("  delete   --id=N")
	fmt.Println("  help")
}