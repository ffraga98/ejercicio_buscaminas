//! # Error
//! `error` es un modulo con `enums` que representan los distintos tipos de errores que pueden ocurrir en el programa.
use std::fmt;
use std::io;

const ESC_EXIT: &str = "\x1b[0;0m";
const COLOR_ERROR: &str = "\x1b[31m";

#[derive(Debug, PartialEq)]
/// `enum` que representa los posibles errores a la hora de reconocer el tablero del buscaminas.
pub enum ErrorMapa<'a> {
    /// Variante que indica que el tablero contiene un caracter que no es reconocido por el juego, por lo que no puede identificar que tipo de [Casillero](../../tablero/casillero/enum.Casillero.html) es.
    CaracterDesconocido(char),
    /// Variante que indica que el tablero se encuentra malformado, es decir, que no cumple con los requisitos para poder ser procesado por el programa.
    Malformacion(&'a str),
    /// Variante que indica que el tablero es vacio, por lo que no existe el problema a resolver.
    MapaVacio,
}

#[derive(Debug)]
/// `enum` que representa los posibles errores de librerías de terceros a la hora de la escritura y lectura de archivos.
pub enum ErrorIO {
    /// Variante para representar que el error fue lanzado por el `struct` BufReader.
    ErrorBuffReader(io::Error),
    /// Variante para representar que el error fue lanzado por el  `Trait` Write.
    ErrorWrite(io::Error),
    /// Variante para representar que el error fue lanzado por el `struct` File.
    ErrorFile(io::Error),
}

impl<'a> fmt::Display for ErrorMapa<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{COLOR_ERROR} ErrorMapa:")?;
        match self {
            ErrorMapa::CaracterDesconocido(c) => write!(f, "El caracter {} no es válido", c)?,
            ErrorMapa::Malformacion(s) => write!(f, "{}", s)?,
            ErrorMapa::MapaVacio => write!(f, "El mapa no existe.")?,
        };
        writeln!(f, "{ESC_EXIT}")
    }
}

impl fmt::Display for ErrorIO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorIO::ErrorBuffReader(error) => {
                writeln!(f, "{COLOR_ERROR}ErrorIO: {}{ESC_EXIT}", error)
            }
            ErrorIO::ErrorWrite(error) => writeln!(f, "{COLOR_ERROR}ErrorIO: {}{ESC_EXIT}", error),
            ErrorIO::ErrorFile(error) => writeln!(f, "{COLOR_ERROR}ErrorIO: {}{ESC_EXIT}", error),
        }
    }
}
