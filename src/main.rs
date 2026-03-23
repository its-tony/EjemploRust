//Antony Portillo 25615
// Ejemplo de un gestor de tareas en Rust con características orientada a objetos
//
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Estado {
    Pendiente,
    EnProgreso,
    Completada,
}

impl std::fmt::Display for Estado {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Estado::Pendiente => write!(f, "Pendiente"),
            Estado::EnProgreso => write!(f, "En progreso"),
            Estado::Completada => write!(f, "Completada"),
        }
    }
}

#[derive(Debug, Clone)]
enum Prioridad {
    Baja,
    Media,
    Alta,
}

impl std::fmt::Display for Prioridad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Prioridad::Baja => write!(f, "Baja"),
            Prioridad::Media => write!(f, "Media"),
            Prioridad::Alta => write!(f, "Alta"),
        }
    }
}

// Trait para mostrar una idea de programación orientada a objetos en Rust
trait Describible {
    fn describir(&self) -> String;
}

#[derive(Debug, Clone)]
struct Tarea {
    id: u32,
    titulo: String,
    descripcion: String,
    prioridad: Prioridad,
    estado: Estado,
}

impl Describible for Tarea {
    fn describir(&self) -> String {
        format!(
            "[{}] {} | Prioridad: {} | Estado: {}\n    Desc: {}",
            self.id, self.titulo, self.prioridad, self.estado, self.descripcion
        )
    }
}

struct GestorTareas {
    tareas: Vec<Tarea>,
    siguiente_id: u32,
}

impl GestorTareas {
    fn new() -> Self {
        Self {
            tareas: Vec::new(),
            siguiente_id: 1,
        }
    }

    fn agregar_tarea(&mut self, titulo: String, descripcion: String, prioridad: Prioridad) {
        let tarea = Tarea {
            id: self.siguiente_id,
            titulo,
            descripcion,
            prioridad,
            estado: Estado::Pendiente,
        };

        self.tareas.push(tarea);
        self.siguiente_id += 1;
    }

    fn cambiar_estado(&mut self, id: u32, nuevo_estado: Estado) -> bool {
        for tarea in &mut self.tareas {
            if tarea.id == id {
                tarea.estado = nuevo_estado;
                return true;
            }
        }
        false
    }

    fn listar_todas(&self) {
        if self.tareas.is_empty() {
            println!("No hay tareas registradas.");
            return;
        }

        println!("\n..............LISTA DE TAREAS........... ");
        for tarea in &self.tareas {
            println!("{}", tarea.describir());
        }
    }

    fn listar_no_completadas(&self) {
        let pendientes: Vec<&Tarea> = self
            .tareas
            .iter()
            .filter(|t| t.estado != Estado::Completada)
            .collect();

        if pendientes.is_empty() {
            println!("No hay tareas pendientes o en progreso YEI");
            return;
        }

        println!("\n--- TAREAS NO COMPLETADAS :(    ");
        for tarea in pendientes {
            println!("{}", tarea.describir());
        }
    }

    fn resumen(&self) {
        let total = self.tareas.len();
        let pendientes = self
            .tareas
            .iter()
            .filter(|t| t.estado == Estado::Pendiente)
            .count();
        let en_progreso = self
            .tareas
            .iter()
            .filter(|t| t.estado == Estado::EnProgreso)
            .count();
        let completadas = self
            .tareas
            .iter()
            .filter(|t| t.estado == Estado::Completada)
            .count();
        let altas = self
            .tareas
            .iter()
            .filter(|t| matches!(t.prioridad, Prioridad::Alta))
            .count();

        let titulos: Vec<&str> = self
            .tareas
            .iter()
            .map(|t| t.titulo.as_str())
            .collect();

        println!("\n--- RESUMEN ---");
        println!("Total de tareas: {}", total);
        println!("Pendientes: {}", pendientes);
        println!("En progreso: {}", en_progreso);
        println!("Completadas: {}", completadas);
        println!("Tareas de prioridad alta: {}", altas);

        if !titulos.is_empty() {
            println!("Títulos registrados: {}", titulos.join(", "));
        }
    }
}

fn leer_linea(mensaje: &str) -> String {
    let mut entrada = String::new();
    print!("{}", mensaje);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut entrada).unwrap();
    entrada.trim().to_string()
}

fn leer_u32(mensaje: &str) -> u32 {
    loop {
        let entrada = leer_linea(mensaje);
        match entrada.parse::<u32>() {
            Ok(numero) => return numero,
            Err(_) => println!(" Escribe un número >:( "),
        }
    }
}

fn leer_prioridad() -> Prioridad {
    loop {
        println!("Selecciona la prioridad:");
        println!("1. Baja");
        println!("2. Media");
        println!("3. Alta");

        let opcion = leer_linea("Opción: ");

        match opcion.as_str() {
            "1" => return Prioridad::Baja,
            "2" => return Prioridad::Media,
            "3" => return Prioridad::Alta,
            _ => println!("Opción inválida."),
        }
    }
}

fn leer_estado() -> Estado {
    loop {
        println!("Nuevo estado:");
        println!("1. Pendiente");
        println!("2. En progreso");
        println!("3. Completada");

        let opcion = leer_linea("Opción: ");

        match opcion.as_str() {
            "1" => return Estado::Pendiente,
            "2" => return Estado::EnProgreso,
            "3" => return Estado::Completada,
            _ => println!("Opción inválida."),
        }
    }
}

fn mostrar_menu() {
    println!("\n ............. GESTOR DE TAREAAAS................ ");
    println!("1. Agregar tarea");
    println!("2. Cambiar estado de una tarea");
    println!("3. Listar todas las tareas");
    println!("4. Listar tareas no completadas");
    println!("5. Ver resumen");
    println!("0. Salir");
}

fn main() {
    let mut gestor = GestorTareas::new();

    loop {
        mostrar_menu();
        let opcion = leer_linea("Elige una opción: ");

        match opcion.as_str() {
            "1" => {
                let titulo = leer_linea("Título: ");
                let descripcion = leer_linea("Descripción: ");
                let prioridad = leer_prioridad();

                gestor.agregar_tarea(titulo, descripcion, prioridad);
                println!("Tarea agregada correctamente yei");
            }
            "2" => {
                let id = leer_u32("ID de la tarea: ");
                let estado = leer_estado();

                if gestor.cambiar_estado(id, estado) {
                    println!("Estado actualizado correctamente.");
                } else {
                    println!("No se encontró una tarea con ese ID.");
                }
            }
            "3" => {
                gestor.listar_todas();
            }
            "4" => {
                gestor.listar_no_completadas();
            }
            "5" => {
                gestor.resumen();
            }
            "0" => {
                println!("Saliendo del programa... :D");
                break;
            }
            _ => {
                println!("Opción no válida.");
            }
        }
    }
}