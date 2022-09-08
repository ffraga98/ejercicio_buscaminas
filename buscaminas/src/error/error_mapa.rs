use std::fmt;

#[derive(PartialEq, Eq)]
/// `enum` que representa los posibles errores a la hora de reconocer el tablero del buscaminas.
pub enum ErrorMapa {
    /// Variante que indica que el tablero contiene un caracter que no es reconocido por el juego, por lo que no puede identificar que tipo de [Casillero](../tablero/casillero/enum.Casillero.html) es.
    CaracterDesconocido(char),
    /// Variante que indica que el tablero se encuentra malformado en sus dimensiones.
    MapaMalformado,
    /// Variante que indica que el tablero es vacio, por lo que no existe el problema a resolver.
    MapaVacio,
    /// Variante que indica que se intento acceder a una celda del [Tablero](../tablero/mod.rs/struct.Tablero.html) que no existe.
    CeldaInexistente,
}

impl fmt::Debug for ErrorMapa {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorMapa: ")?;
        match self {
            ErrorMapa::CaracterDesconocido(c) => write!(f, "El caracter {} no es válido", c),
            ErrorMapa::MapaMalformado => write!(f, "El mapa no es rectangular."),
            ErrorMapa::MapaVacio => write!(f, "El mapa no existe."),
            ErrorMapa::CeldaInexistente => {
                write!(f, "Se intentó acceder a una celda que no existe")
            }
        }
    }
}
