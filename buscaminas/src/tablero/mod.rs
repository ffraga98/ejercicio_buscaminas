//! Tablero
//! `tablero` es el modulo que contiene todo lo relacionado al mapa del problema a resolver.
use crate::tablero::casillero::Casillero;
use crate::tablero::coordenada::Coordenadas2D;

use std::fmt;

pub mod builder;
pub mod casillero;
pub mod coordenada;

#[derive(Debug, PartialEq)]
/// Esctructura que contiene toda la informacin necesaria para resolver y contener la solucion del problema.
/// # Observaciones
/// El Tablero solo puede contener mapas que sea rectangulares, por eso es que posee los campos `ancho` y `largo`.
pub struct Tablero {
    /// Campo utilizado para indicar el largo del tablero.
    largo: usize,
    /// Campo utilizado para indicar el ancho del tablero.
    ancho: usize,
    /// Campo que contiene la composicion del mapa. Este campo es utilizado tanto para almacenar el problema inicial, como la solucion del problema.
    mapa: Vec<Casillero>,
}

impl fmt::Display for Tablero {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _i in 0..self.largo {
            for _j in 0..self.ancho {
                //FIXME: Quitar unwrap
                write!(f, "{}", self.mapa.get(_i * self.ancho + _j).unwrap())?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl Tablero {
    /// Retorna un nuevo Tablero con la solucion del problema almacenada en el Tablero invocador.
    /// # Ejempos
    /// ```
    /// let tablero = Tablero {
    ///   largo: 2,
    ///   ancho: 2,
    ///   mapa: vec![Casillero::Espacio(0), Casillero::Mina, Casillero::Espacio(0), Casillero::Espacio(0)],
    /// };
    ///
    /// let resultado = tablero.resolver();
    /// let esperado = Tablero {
    ///   largo: 2,
    ///   ancho: 2,
    ///   mapa: vec![Casillero::Espacio(1), Casillero::Mina, Casillero::Espacio(1), Casillero::Espacio(1)],
    /// };
    ///
    /// assert_eq!(resultado, esperado);
    /// ```
    fn resolver(&self) -> Tablero {
        let mut solucion = Vec::new();
        for (index, _c) in self.mapa.iter().enumerate() {
            match _c {
                Casillero::Espacio(_) => {
                    solucion.push(Casillero::Espacio(self.calcular_minas_adyacentes(index)))
                }
                Casillero::Mina => solucion.push(Casillero::Mina),
                _ => (),
            }
        }
        Tablero {
            ancho: self.ancho,
            largo: self.largo,
            mapa: solucion,
        }
    }
    
    /// Retorna la cantidad de minas adyacentes a un casillero determinado. No le interesa si el casillero corresponde a un `Casillero::Espacio` o `Casillero::Mina`. Para ver la numeracion de los casilleros leer el metodo `obtener_indice(&self, coordenada)`.
    /// # Ejemplos
    /// ```
    /// // Tablero 2x2 
    /// // Numeracion
    /// // 1 2 
    /// // 3 4
    /// let tablero = Tablero {
    ///   largo: 2,
    ///   ancho: 2,
    ///   mapa: vec![Casillero::Mina, Casillero::Mina, Casillero::Espacio(0), Casillero::Espacio(0)],
    /// };
    ///
    /// let resultado = tablero.calcular_minas_adyacentes(4);
    /// let esperado = 2;
    /// assert_eq!(resultado, esperado);
    ///
    /// let resultado = tablero.calcular_minas_adyacentes(2);
    /// let esperado = 1;
    /// assert_eq!(resultado, esperado);
    /// ```
    fn calcular_minas_adyacentes(&self, num_casillero: usize) -> u8 {
        let fila = num_casillero / self.ancho;
        let columna = num_casillero % self.ancho;
        let coordenada = Coordenadas2D::new(columna, fila);
        let mut contador = 0;

        let adyacentes = match self.obtener_adyacentes(coordenada) {
            None => return contador,
            Some(coordenadas) => coordenadas,
        };

        for ady in &adyacentes {
            if let Casillero::Mina = ady {
                contador += 1
            }
        }
        contador
    }
    /// FIX ME: Cambiar el None() por un error. Wrappear el Option con un Result.
    /// Casillero inválido!
    /// Retorna en dentro de un Option<>, un vector de Casilleros adyacentes    
    /// None() cuando no puedo hacer el get y cuando el casillero es una Mina (evita el conteo al pedo)
    /// 
    fn obtener_adyacentes(&self, coordenada: Coordenadas2D) -> Option<Vec<&Casillero>> {
        let indice = self.obtener_indice(&coordenada);
        match self.mapa.get(indice) {
            None => return None,
            Some(c) => {
                if let Casillero::Mina = c {
                    return None;
                }
            }

        }
        let mut casilleros = Vec::new();

        let coordenadas_ady = coordenada.coordenadas_adyacentes(self.ancho, self.largo);
        for _c in coordenadas_ady {
            casilleros.push(self.mapa.get(self.obtener_indice(&_c))?);
        }
        Some(casilleros)
    }
    /// Retorna el indice correspondiente a una coordenada dentro del campo `solucion`. La numeracion esta dada por como se almaceno el mapa en el vector `solucion`. 
    /// # Ejemplos
    /// ```
    /// // Tablero 3x2
    /// // Numeracion
    /// // 1 2 3 
    /// // 4 5 6 
    /// let tablero = Tablero {
    ///    largo: 2,
    ///    ancho: 3,
    ///   mapa: vec![...] // No es relevante como se inicializa.
    ///}
    /// let resultado = tablero.obtener_indice(&Coordenada2D::new(1,0));
    /// let esperado = 2;
    /// assert_eq!(resultado,esperado);
    ///
    /// let resultado = tablero.obtener_indice(&Coordenada2D::new(0,1));
    /// let esperado = 4;
    /// assert_eq!(resultado,esperado);
    ///
    /// let resultado = tablero.obtener_indice(&Coordenada2D::new(2,1));
    /// let esperado = 6;
    /// assert_eq!(resultado,esperado);
    /// ```
    fn obtener_indice(&self, coordenada: &Coordenadas2D) -> usize {
        (coordenada.y() + 1) * self.ancho - (self.ancho - coordenada.x())
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
            let resultado = tablero.calcular_minas_adyacentes(_i);
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
            let resultado = tablero.calcular_minas_adyacentes(_i);
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
        let resultado = tablero.calcular_minas_adyacentes(1);
        assert_eq!(resultado, esperado);

        let esperado = 2;
        let resultado = tablero.calcular_minas_adyacentes(3);
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
        let resultado = tablero.resolver();
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
        let resultado = tablero.resolver();
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
        let resultado = tablero.resolver();
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
        let resultado = tablero.obtener_indice(&coordenada);
        assert_eq!(resultado, esperado);
    }
}
