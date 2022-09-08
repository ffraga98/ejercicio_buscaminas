//! # Error
//! `error` es un modulo con `enums` que representan los distintos tipos de errores que pueden ocurrir en el programa.
const ESC_EXIT: &str = "\x1b[0;0m";
const COLOR_ERROR: &str = "\x1b[31m";
use crate::error::error_io::ErrorIO;
use crate::error::error_mapa::ErrorMapa;
use std::fmt;
use std::fmt::Debug;

pub mod error_io;
pub mod error_mapa;

pub enum Error {
    Emapa(ErrorMapa),
    Eio(ErrorIO),
}

impl From<ErrorIO> for Error {
    fn from(error: ErrorIO) -> Self {
        Error::Eio(error)
    }
}

impl From<ErrorMapa> for Error {
    fn from(error: ErrorMapa) -> Self {
        Error::Emapa(error)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{COLOR_ERROR}")?;
        match self {
            Error::Emapa(e) => write!(f, "{:?}", e)?,
            Error::Eio(e) => write!(f, "{:?}", e)?,
        }
        write!(f, "{ESC_EXIT}")
    }
}
