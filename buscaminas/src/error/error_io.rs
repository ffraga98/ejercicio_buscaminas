//! # ErrorIO
//! 
//! `error_io` permite representar todos los errores que pueden presentarse a la hora de la lectura o escritura de los archivos. 
use std::fmt;
use std::io;

/// `enum` que representa los posibles errores de librerías de terceros a la hora de la escritura y lectura de archivos.
pub enum ErrorIO {
    /// Variante para representar que el error fue lanzado por el `struct` [`BufReader`][std::io::BufReader], utilizado en [Lector](../my_io/lector/struct.Lector.html#leer_archivo).
    ErrorBufReader(io::Error),
    /// Variante para representar que el error fue lanzado por el  `Trait` [`Write`][std::io::Write], utilizado en [Escritor](../my_io/escritor/struct.Escritor.html#imprimir_item).
    ErrorWrite(io::Error),
    /// Variante para representar que el error fue lanzado por el `struct` [`File`][std::fs::File], utilizado en [Lector](../my_io/lector/struct.Lector.html#leer_archivo) y [Escritor](../my_io/escritor/struct.Escritor.html#imprimir_item).
    ErrorFile(io::Error),
    /// Variante para representar que el error fue lanzado por no cumplir con los parámetros necesarios desde la línea de comandos
    ErrorCLI,
}

impl fmt::Debug for ErrorIO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorIO: ")?;
        match self {
            ErrorIO::ErrorBufReader(error) => write!(f, "{}", error),
            ErrorIO::ErrorWrite(error) => write!(f, "{}", error),
            ErrorIO::ErrorFile(error) => write!(f, "{}", error),
            ErrorIO::ErrorCLI => write!(
                f,
                "No se ha pasado la ruta del archivo con el problema a resolver."
            ),
        }
    }
}
