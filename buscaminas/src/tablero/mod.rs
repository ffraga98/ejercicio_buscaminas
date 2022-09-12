//! # Tablero
//!
//! `tablero` es el módulo que contiene todo lo relacionado al mapa del problema a resolver.
use crate::error::error_mapa::ErrorMapa;
use crate::tablero::casillero::Casillero;
use crate::tablero::coordenada::Coordenadas2D;

use std::fmt;

pub mod builder;
pub mod casillero;
pub mod coordenada;

#[derive(Debug, PartialEq, Eq)]
/// Estructura que contiene toda la información necesaria para resolver y contener la solución del problema.
///
/// ### Observacion
///
/// - El Tablero solo puede contener mapas que sea *rectangulares*.
pub struct Tablero {
    /// Campo utilizado para indicar el largo del tablero.
    largo: usize,
    /// Campo utilizado para indicar el ancho del tablero.
    ancho: usize,
    /// Campo que contiene la composicion del mapa. Este campo es utilizado tanto para almacenar el problema inicial, como la solucion del problema. Cada celda del tablero es representado por un [`Casillero`].
    mapa: Vec<Casillero>,
}

impl fmt::Display for Tablero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.mapa
            .iter()
            .enumerate()
            .try_for_each(|(i, c)| match i % self.ancho == self.ancho - 1 {
                true => writeln!(f, "{}", c),
                false => write!(f, "{}", c),
            })
    }
}

impl Tablero {
    /// Retorna un nuevo [`Tablero`] con la solucion del problema almacenado en el invocador.
    ///
    /// # Errores
    ///
    /// Retornara el error dado por el [método] que calcula la cantidad de minas adyacentes.
    ///
    /// [método]: ./struct.Tablero.html#method.calcular_minas_adyacentes
    ///
    fn resolver(&self) -> Result<Tablero, ErrorMapa> {
        let mut solucion = Vec::with_capacity(self.ancho * self.largo);
        for (index, _c) in self.mapa.iter().enumerate() {
            match _c {
                Casillero::Espacio(_) => {
                    solucion.push(Casillero::Espacio(self.calcular_minas_adyacentes(index)?))
                }
                Casillero::Mina => solucion.push(Casillero::Mina),
                _ => (),
            }
        }
        Ok(Tablero {
            ancho: self.ancho,
            largo: self.largo,
            mapa: solucion,
        })
    }

    /// Retorna la cantidad de [minas][Mina] adyacentes a una celda determinada. En caso de que la coodenada corresponda a una [`Mina`][Mina], retornará `0`. Para ver la numeracion de las celdas dentro de [mapa] leer el siguiente [método].
    ///
    /// [Mina]: Casillero::Mina
    /// [método]: ./struct.Tablero.html#method.obtener_indice
    /// [mapa]: ./struct.Tablero.html#structfield.mapa
    ///
    /// # Errores
    ///
    /// Retornara el error lanzado por el [método] que obtiene los casilleros adyacentes.
    ///
    /// [método]: ./struct.Tablero.html#method.obtener_adyacentes
    ///
    fn calcular_minas_adyacentes(&self, num_casillero: usize) -> Result<u8, ErrorMapa> {
        let (fila, columna) = (num_casillero / self.ancho, num_casillero % self.ancho);
        let coordenada = Coordenadas2D::new(columna, fila);

        let adyacentes = self.obtener_adyacentes(coordenada)?;
        Ok(adyacentes
            .iter()
            .filter(|&c| (**c) == Casillero::Mina)
            .count() as u8)
    }

    /// Retorna la cantidad de minas adyacentes a un [`Espacio`], en caso de ser una [`Mina`] se retornará un [vector][Vec] vacío.
    ///
    /// # Errores
    ///
    /// Retornara el mismo error que retorne el [método] encargado de obtener casilleros del mapa.
    ///
    /// [`Mina`]: Casillero::Mina
    /// [`Espacio`]: Casillero::Espacio
    /// [método]: ./struct.Tablero.html#method.obtener_casillero
    ///
    fn obtener_adyacentes(&self, coordenada: Coordenadas2D) -> Result<Vec<&Casillero>, ErrorMapa> {
        if let Casillero::Mina = self.obtener_casillero(&coordenada)? {
            return Ok(Vec::with_capacity(0));
        }
        coordenada
            .coordenadas_adyacentes(self.ancho, self.largo)
            .iter()
            .map(|c| self.obtener_casillero(c))
            .collect()
    }

    /// Retorna el casillero correspondiente a la [coordenada] que se pase por parámetro.
    ///
    /// # Errores
    ///
    /// Retornara [`CeldaInexistente`] caso la [coordenada] corresponda a un [indice] que sobrepase los límites del campo [mapa].
    ///
    /// [`CeldaInexistente`]: ErrorMapa::CeldaInexistente
    /// [mapa]: ./struct.Tablero.html#structfield.mapa
    /// [indice]: ./struct.Tablero.html#method.obtener_indice
    /// [coordenada]: coordenada::Coordenadas2D
    fn obtener_casillero(&self, coordenada: &Coordenadas2D) -> Result<&Casillero, ErrorMapa> {
        self.mapa
            .get(self.obtener_indice(coordenada)?)
            .ok_or(ErrorMapa::CeldaInexistente)
    }
    /// Retorna el indice correspondiente a una coordenada dentro del campo [mapa].
    /// La numeracion esta dada por como se almaceno el [mapa].
    ///
    /// [mapa]: ./struct.Tablero.html#structfield.mapa
    ///
    /// ```
    ///  
    /// // Numeracion de Tablero 3x2
    ///  
    /// // 1 2 3
    /// // 4 5 6
    ///
    /// ```
    fn obtener_indice(&self, coordenada: &Coordenadas2D) -> Result<usize, ErrorMapa> {
        let (x, y) = (coordenada.x(), coordenada.y());
        match self.ancho < x && self.largo < y {
            false => Ok((y + 1) * self.ancho - (self.ancho - x)),
            true => Err(ErrorMapa::CeldaInexistente),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn calcular_minas_adyacentes_tablero_vacio() {
        let tablero = Tablero {
            ancho: 2,
            largo: 2,
            mapa: vec![
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
            ],
        };
        let esperado = 0;
        for _i in 0..4 {
            let resultado = tablero.calcular_minas_adyacentes(_i).unwrap();
            assert_eq!(resultado, esperado)
        }
    }
    #[test]
    fn calcular_minas_adyacentes_tablero_lleno() {
        let tablero = Tablero {
            ancho: 2,
            largo: 2,
            mapa: vec![
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
            ],
        };
        let esperado = 0;
        for _i in 0..4 {
            let resultado = tablero.calcular_minas_adyacentes(_i).unwrap();
            assert_eq!(resultado, esperado)
        }
    }

    #[test]
    fn calcular_minas_adyacentes_tablero_mixto() {
        let tablero = Tablero {
            ancho: 3,
            largo: 2,
            mapa: vec![
                Casillero::Mina,
                Casillero::Espacio(0),
                Casillero::Mina,
                Casillero::Espacio(0),
                Casillero::Mina,
                Casillero::Mina,
            ],
        };
        // *.* > *4*
        // .** > 2**

        let esperado = 4;
        let resultado = tablero.calcular_minas_adyacentes(1).unwrap();
        assert_eq!(resultado, esperado);

        let esperado = 2;
        let resultado = tablero.calcular_minas_adyacentes(3).unwrap();
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn resolver_tablero_vacio() {
        let tablero = Tablero {
            ancho: 2,
            largo: 2,
            mapa: vec![
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
                Casillero::Espacio(0),
            ],
        };
        let resultado = tablero.resolver().unwrap();
        assert_eq!(resultado, tablero);
    }

    #[test]
    fn resolver_tablero_lleno() {
        let tablero = Tablero {
            ancho: 2,
            largo: 2,
            mapa: vec![
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
                Casillero::Mina,
            ],
        };
        let resultado = tablero.resolver().unwrap();
        assert_eq!(resultado, tablero);
    }

    #[test]
    fn resolver_tablero_mixto() {
        let tablero = Tablero {
            ancho: 3,
            largo: 2,
            mapa: vec![
                Casillero::Mina,
                Casillero::Espacio(0),
                Casillero::Mina,
                Casillero::Espacio(0),
                Casillero::Mina,
                Casillero::Mina,
            ],
        };
        let resultado = tablero.resolver().unwrap();
        let esperado = Tablero {
            ancho: 3,
            largo: 2,
            mapa: vec![
                Casillero::Mina,
                Casillero::Espacio(4),
                Casillero::Mina,
                Casillero::Espacio(2),
                Casillero::Mina,
                Casillero::Mina,
            ],
        };
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn transformar_coordenada_a_indice() {
        let tablero = Tablero {
            ancho: 5,
            largo: 5,
            mapa: vec![],
        };
        let coordenada = Coordenadas2D::new(4, 3);
        //  0, 1, 2, 3, 4
        //  5, 6, 7, 8, 9
        // 10,11,12,13,14
        // 15,16,17,18,[19]
        // 20,21,22,23,24
        let esperado = 19;
        let resultado = tablero.obtener_indice(&coordenada).unwrap();
        assert_eq!(resultado, esperado);
    }
}
