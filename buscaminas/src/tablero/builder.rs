//! # Builder
//! `builder` es un submódulo que contiene el builder para el tablero [TableroBuilder](./struct.TableroBuilder.html)
use crate::error::ErrorMapa;
use crate::tablero::casillero::Casillero;
use crate::tablero::Tablero;

/// Constante usada para separar las lineas dentro del campo casilleros.
pub const NUEVA_LINEA_ICONO: u8 = b'-';
/// Constante usada para representar a la mina dentro del campo casilleros.
const MINA_ICONO: u8 = b'*';
/// Constante usada para representar a un casillero vacio dentro del campo casilleros.
const ESPACIO_ICONO: u8 = b'.';

#[derive(Debug, PartialEq)]
/// Estructura que contiene la representación del tablero en
pub struct TableroBuilder<'a> {
    /// `&str` que representa al tablero del juego. Es la base utilizada para poder construir el [Tablero](../struct.Tablero.html)
    casilleros: &'a str,
}

impl<'a> TableroBuilder<'a> {
    ///Construye un nuevo TableroBuilder con el `&str`, que representa al tablero, inicializado que se pase por paramentro.
    /// # Ejemplos
    /// ```
    /// // Tablero 4x3 con minas y vacios.
    /// let builder = TableroBuilder::new("**.-..*-***-...-");
    /// // Tablero 2x3 con minas y vacios.
    /// let builder = TableroBuilder::new("**.-..*-");
    /// // Tablero vacio
    /// let builder_error= TableroBuilder::new("");
    /// assert!(builder_error.is_err());
    /// ```
    /// # Errores
    /// El TableroBuilder impide que se construyan Tableros sin casilleros, por lo que lanzara un error si se le pasa un string vacío.
    pub fn new(casilleros: &str) -> Result<TableroBuilder, ErrorMapa> {
        if casilleros == "" {
            return Err(ErrorMapa::MapaVacio);
        }
        Ok(TableroBuilder { casilleros })
    }
    ///Construye el tablero resuelto. Retorna un Result<Tablero,[ErrorMapa](../../error/io/enum.ErrorMapa.html)
    /// # Ejemplos
    /// ```
    /// // Tablero 2x2 con minas y espacios vacios.
    /// let builder = TableroBuilder::new("*.-.*-");
    /// let resultado = builder.crear_tablero().unwrap();
    /// let esperado = Tablero {
    ///  ancho: 2,
    ///  largo: 2,  
    ///  mapa: vec![Casillero::Mina, Casillero::Espacio(2), Casillero::Espacio(2), Casillero::Mina],  
    /// };
    ///
    /// assert_eq!(resultado, esperado);
    /// ```
    /// # Errores
    /// Retorna el mismo error que el método `cargar_tablero()`. 
    pub fn crear_tablero(&self) -> Result<Tablero,ErrorMapa> {
        let tablero = self.cargar_tablero()?;
        Ok(tablero.resolver())
    }

    ///Construye el tablero segun el contenido en `casilleros`, sin resolver. Retorna un Result<Tablero,[ErrorMapa](../../error/io/enum.ErrorMapa.html)
    /// # Ejemplos
    /// ```
    /// // Tablero 2x2 con minas y espacios vacios.
    /// let builder = TableroBuilder::new("*.-.*-");
    /// let resultado = builder.cargar_tablero().unwrap();
    /// let esperado = Tablero {
    ///  ancho: 2,
    ///  largo: 2,  
    ///  mapa: vec![Casillero::Mina, Casillero::Espacio(0), Casillero::Espacio(0), Casillero::Mina],  
    /// };
    ///
    /// assert_eq!(resultado, esperado);
    /// ```
    /// # Errores
    /// Retorna un error en caso de querer construir un Tablero cuyas dimensiones no corresponden a un Tablero rectangular. Además, retorna los mismos errores que el metodo `identificar()`.
    fn cargar_tablero(&self) -> Result<Tablero, ErrorMapa> {
        let (mut mapa, mut aux_ancho, mut ancho, mut largo) = (vec![], 0, 0, 0);

        for _c in self.casilleros.as_bytes() {
            match Self::identificar(*_c) {
                Ok(value) => match value {
                    Casillero::NuevaLinea => {
                        largo += 1;
                        match ancho != aux_ancho {
                            true => {
                                return Err(ErrorMapa::Malformacion("El mapa no es rectangular."))
                            }
                            false => aux_ancho = 0,
                        }
                    }
                    other => {
                        mapa.push(other);
                        aux_ancho += 1;
                    }
                },
                Err(error) => return Err(error),
            }

            if largo == 0 {
                ancho += 1;
            }
        }
        Ok(Tablero { largo, ancho, mapa })
    }

    /// Identifica el tipo de casillero segun el `u8` que se presente.
    /// # Ejemplos
    /// ```
    /// let resultado = TableroBuilder::identificar(MINA_ICONO).unwrap();
    /// let esperado = Casillero::Mina;
    /// assert_eq!(resultado, esperado);
    ///
    /// let resultado = TableroBuilder::identificar(ESPACIO_ICONO).unwrap();
    /// let esperado = Casillero::Espacio(0);
    /// assert_eq!(resultado, esperado);
    ///
    /// let resultado = TableroBuilder::identificar(NUEVA_LINEA_ICONO).unwrap();
    /// let esperado = Casillero::NuevaLinea;
    /// assert_eq!(resultado, esperado);
    ///
    /// ```
    /// # Errores
    /// Retorna ErrorMapa::CaracterDesconocido en caso de que no sea un `u8` que no pertenezca a los iconos aceptados.
    fn identificar(caracter: u8) -> Result<Casillero, ErrorMapa<'a>> {
        match caracter {
            MINA_ICONO => Ok(Casillero::Mina),
            ESPACIO_ICONO => Ok(Casillero::Espacio(0)),
            NUEVA_LINEA_ICONO => Ok(Casillero::NuevaLinea),
            other => Err(ErrorMapa::CaracterDesconocido(other as char)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crear_tablero_builder() {
        let resultado = TableroBuilder::new("...-...-...").unwrap();
        let esperado = TableroBuilder {
            casilleros: "...-...-...",
        };

        assert_eq!(resultado, esperado);
    }
    
    #[test]
    fn crear_tablero_builder_vacio(){
        let resultado = TableroBuilder::new("");
        assert!(resultado.is_err());
    }

    #[test]
    fn identificar_casillero_mediante_u8() {
        let resultado = TableroBuilder::identificar(MINA_ICONO).unwrap();
        let esperado = Casillero::Mina;
        assert_eq!(resultado, esperado);

        let resultado = TableroBuilder::identificar(ESPACIO_ICONO).unwrap();
        let esperado = Casillero::Espacio(0);
        assert_eq!(resultado, esperado);

        let resultado = TableroBuilder::identificar(NUEVA_LINEA_ICONO).unwrap();
        let esperado = Casillero::NuevaLinea;
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn identificar_con_u8_no_valido() {
        let resultado = TableroBuilder::identificar(b'a');
        assert!(resultado.is_err());
    }
    #[test]
    fn cargar_tablero_sin_minas() {
        let builder = TableroBuilder::new("...-...-").unwrap();
        let resultado = builder.cargar_tablero().unwrap();
        let esperado = Tablero {
            ancho: 3,
            largo: 2,
            mapa: vec![
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
            ],
        };
        assert_eq!(resultado, esperado);
    }
    #[test]
    fn cargar_tablero_con_minas() {
        let builder = TableroBuilder::new("**-**-**-").unwrap();
        let resultado = builder.cargar_tablero().unwrap();
        let esperado = Tablero {
            ancho: 2,
            largo: 3,
            mapa: vec![
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
            ],
        };
        assert_eq!(resultado, esperado);
    }
    #[test]
    fn cargar_tablero_mixto() {
        let builder = TableroBuilder::new("*.*-.*.-*.*-").unwrap();
        let resultado = builder.cargar_tablero().unwrap();
        let esperado = Tablero {
            ancho: 3,
            largo: 3,
            mapa: vec![
                Casillero::Mina,
                Casillero::Espacio(0),
                Casillero::Mina,
                Casillero::Espacio(0),
                Casillero::Mina,
                Casillero::Espacio(0),
                Casillero::Mina,
                Casillero::Espacio(0),
                Casillero::Mina,
            ],
        };
        assert_eq!(resultado, esperado);
    }


    #[test]
    fn crear_tablero_sin_minas() {
        let builder = TableroBuilder::new("...-...-").unwrap();
        let resultado = builder.crear_tablero().unwrap();
        let esperado = Tablero {
            ancho: 3,
            largo: 2,
            mapa: vec![
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
            ],
        };
        assert_eq!(resultado, esperado);
    }
    #[test]
    fn crear_tablero_con_minas() {
        let builder = TableroBuilder::new("**-**-**-").unwrap();
        let resultado = builder.crear_tablero().unwrap();
        let esperado = Tablero {
            ancho: 2,
            largo: 3,
            mapa: vec![
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
            ],
        };
        assert_eq!(resultado, esperado);
    }
    #[test]
    fn crear_tablero_mixto() {
        let builder = TableroBuilder::new("*.*-.*.-*.*-").unwrap();
        let resultado = builder.crear_tablero().unwrap();
        let esperado = Tablero {
            ancho: 3,
            largo: 3,
            mapa: vec![
                Casillero::Mina,
                Casillero::Espacio(3),
                Casillero::Mina,
                Casillero::Espacio(3),
                Casillero::Mina,
                Casillero::Espacio(3),
                Casillero::Mina,
                Casillero::Espacio(3),
                Casillero::Mina,
            ],
        };
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn crear_tablero_malformado_no_rectangular() {
        let builder = TableroBuilder::new("*.*-.*.-*.-").unwrap();
        let resultado = builder.crear_tablero();
        assert!(resultado.is_err());
    }

    #[test]
    fn crear_tablero_malformado_caracter_no_identificable() {
        let builder = TableroBuilder::new("*.*-.*.-*.a-").unwrap();
        let resultado = builder.crear_tablero();
        assert!(resultado.is_err());
    }
}
