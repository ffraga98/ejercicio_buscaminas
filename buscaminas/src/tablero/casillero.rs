//! Casillero
//! `casillero` es un submodulo que contiene los posibles tipos de celdas en un Tablero.
use crate::tablero::builder::{ESPACIO_ICONO, MINA_ICONO};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
/// `enum` cuyas variantes representan los distintos tipos de celdas presentes en el Tablero del buscaminas.
pub enum Casillero {
    /// Variante que representa a las minas del Tablero.
    Mina,
    /// Variante que representa a los espacios vacios dentro de un Tablero.
    Espacio(u8),
    /// Variante que indica una nueva linea, indicando el final de una de las filas del Tablero. Necesario para la construccion del Tablero.
    NuevaLinea,
}

impl fmt::Display for Casillero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Casillero::Mina => write!(f, "{}", MINA_ICONO as char),
            Casillero::Espacio(x) => {
                if *x == 0 {
                    write!(f, "{}", ESPACIO_ICONO as char)
                } else {
                    write!(f, "{}", *x)
                }
            }
            Casillero::NuevaLinea => writeln!(f),
        }
    }
}
