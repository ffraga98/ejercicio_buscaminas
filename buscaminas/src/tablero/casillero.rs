//! Casillero
//!
//! `casillero` es un submódulo que contiene los posibles tipos de celdas en un [`Tablero`].
//!
//! [`Tablero`]: ./struct.Tablero.html
use crate::tablero::builder::{ESPACIO_ICONO, MINA_ICONO};
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
/// `enum` cuyas variantes representan los distintos tipos de celdas presentes en el [`Tablero`] del buscaminas.
///
/// [`Tablero`]: ./struct.Tablero.html
pub enum Casillero {
    /// Variante que representa a las minas.
    Mina,
    /// Variante que representa a los espacios vacios.
    Espacio(u8),
    /// Variante que indica una nueva linea, indicando el final de una de las filas. Necesario para la construccion del [`Tablero`].
    ///
    /// [`Tablero`]: ./struct.Tablero.html
    NuevaLinea,
}

impl fmt::Display for Casillero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Casillero::Espacio(x) => match *x == 0 {
                true => write!(f, "{}", ESPACIO_ICONO as char),
                false => write!(f, "{}", *x),
            },
            Casillero::Mina => write!(f, "{}", MINA_ICONO as char),
            Casillero::NuevaLinea => writeln!(f),
        }
    }
}
