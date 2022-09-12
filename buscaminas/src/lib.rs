//! # Buscaminas
//!
//! Crate creado para el recuento de minas en un tablero de Buscaminas completo. El programa es capaz de contra el número de minas adyactenes a cada cuadrado vacío y los reemplaza por el resultado.
//! Dicha resolución es publicada en el [archivo][ARCHIVO_SOLUCION]. 
//! 
//! ## Contenido 
//! Dentro del `crate` se encuentran 3 módulos con componentes que permiten la funcionalidad requerida para este trabajo.
//!  - [Manejo de errores][error]
//!     - Todo lo relacionado a los potenciales errores que pueden ocurrir en el programa y en el uso de los distintos ítems.
//!  - [Manejo de la entrada y salida][my_io]
//!     - Contiene ítems para simplifican la lectura del archivo pasado por línea de comandos y la escritua de la resolución en un archivo de texto.
//!  - [Construcción y solución del problema][tablero]
//!     - Su contenido esta relacionado con los componentes que hacen al tablero, tanto la construcción del problema incial, como la resolución del problema.
//! 
pub mod error;
pub mod my_io;
pub mod tablero;

pub const ARCHIVO_SOLUCION: &str = "solucion.txt";
