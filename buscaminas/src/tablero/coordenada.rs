//! Coordenada
//! `coordenada` es un submódulo dedicado al `struct` que da noción de posición dentro del Tablero.
#[derive(Debug, PartialEq, Eq)]
/// `struct` que representa la posición dentro del Tablero de dos dimensiones. No es posible construir una Coordenadas2D con posiciones negativas.
/// # Observación
/// A lo largo del programa `buscaminas` se utilizó el origen de coordenadas como la esquina superior izquierda. Dicha suposicion
/// afecta al uso de este ítem.
pub struct Coordenadas2D {
    x: usize,
    y: usize,
}

impl Coordenadas2D {
    ///Construye un par de [coordenadas][Coordenadas2D] con las posiciones (x,y).
    ///
    ///# Ejemplos
    ///
    ///```
    /// # use buscaminas::tablero::coordenada::Coordenadas2D;
    /// #
    /// # fn main() {
    /// // Coordenada en el origen
    ///let resultado = Coordenadas2D::new(0,0);
    /// // Coordenada en la posicion (5,2). Equivalente a la celda de la sexta columna y tercera fila.
    ///let resultado = Coordenadas2D::new(5,2);
    /// # }
    /// ```
    pub fn new(x: usize, y: usize) -> Coordenadas2D {
        Coordenadas2D { x, y }
    }

    /// Retorna una vector con las [coordenadas][Coordenadas2D] que sean adyacentes. La adyacencia está dada por cualquier coordenada que este a un movimiento de distancia de la misma. Los movimientos válidos son de manera recta y diagonal coordenada que este a un movimiento de distancia de la misma. Los movimientos válidos son en la dirección recta o diagonal.
    ///
    ///# Ejemplos
    ///
    ///```
    /// # use buscaminas::tablero::coordenada::Coordenadas2D;
    /// #
    /// # fn main() {
    /// // Asumiendo un mapa con un tamaño mínimo de 3x3
    /// # let (ancho_mapa, largo_mapa) = (3,3);
    ///let coordenada = Coordenadas2D::new(0,0);
    ///let resultado = coordenada.coordenadas_adyacentes(ancho_mapa,largo_mapa);
    ///let esperados = vec![ Coordenadas2D::new(0,1), Coordenadas2D::new(1,0), Coordenadas2D::new(1,1)];
    ///assert_eq!(resultado, esperados);
    ///
    ///let coordenada = Coordenadas2D::new(1,0);
    ///let resultado = coordenada.coordenadas_adyacentes(ancho_mapa,largo_mapa);
    ///let esperados = vec![Coordenadas2D::new(0,0), Coordenadas2D::new(0,1), Coordenadas2D::new(1,1), Coordenadas2D::new(2,0), Coordenadas2D::new(2,1)];
    ///assert_eq!(resultado, esperados);
    ///
    ///let coordenada = Coordenadas2D::new(1,1);
    ///let resultado = coordenada.coordenadas_adyacentes(ancho_mapa,largo_mapa);
    ///let esperados = vec![Coordenadas2D::new(0,0), Coordenadas2D::new(0,1), Coordenadas2D::new(0,2), Coordenadas2D::new(1,0), Coordenadas2D::new(1,2), Coordenadas2D::new(2,0), Coordenadas2D::new(2,1), Coordenadas2D::new(2,2)];
    ///assert_eq!(resultado, esperados);
    ///
    /// # }
    ///
    /// ```
    pub fn coordenadas_adyacentes(&self, ancho: usize, largo: usize) -> Vec<Coordenadas2D> {
        let coordenadas_x = Self::obtener_coordenada_adyacente(self.x, ancho);
        let coordenadas_y = Self::obtener_coordenada_adyacente(self.y, largo);
        let mut coordenadas = Vec::with_capacity(coordenadas_x.len() * coordenadas_y.len());
        for _x in &coordenadas_x {
            for _y in &coordenadas_y {
                // Saltearse a si mismo
                if *_x == self.x && *_y == self.y {
                    continue;
                }
                coordenadas.push(Coordenadas2D::new(*_x, *_y))
            }
        }
        coordenadas
    }
    /// Retorna una vector con las coordenadas adyacentes en una de las dimensiones.
    fn obtener_coordenada_adyacente(coordenada: usize, dimension: usize) -> Vec<usize> {
        let mut coordenadas = Vec::with_capacity(2);
        if coordenada != 0 {
            coordenadas.push(coordenada - 1);
        }
        coordenadas.push(coordenada);
        if coordenada != dimension - 1 {
            coordenadas.push(coordenada + 1);
        }
        coordenadas
    }

    /// Retorna la coordenada X, equivalente a la posición en las columnas del mapa.
    ///
    /// # Ejemplos
    /// ```
    /// # use buscaminas::tablero::coordenada::Coordenadas2D;
    /// #
    /// # fn main() {
    /// let coordenada = Coordenadas2D::new(2,0);
    /// let resultado = coordenada.x();
    /// assert_eq!(resultado, 2);
    /// # }
    /// ```
    pub fn x(&self) -> usize {
        self.x
    }

    /// Retorna la coordenada X, equivalente a la posición en las columnas del mapa.
    ///
    /// # Ejemplos
    /// ```
    /// # use buscaminas::tablero::coordenada::Coordenadas2D;
    /// #
    /// # fn main() {
    /// let coordenada = Coordenadas2D::new(0,4);
    /// let resultado = coordenada.y();
    /// assert_eq!(resultado, 4);
    /// # }
    /// ```
    pub fn y(&self) -> usize {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crear_nueva_coordenada() {
        let coordenada = Coordenadas2D::new(2, 3);
        let esperado = Coordenadas2D { x: 2, y: 3 };

        assert_eq!(coordenada, esperado);
    }

    #[test]
    fn obtener_indices_adayecentes() {
        // Asumo mapa 3x3
        let ancho = 3;
        let largo = 3;
        // 0,1,2
        // 3,4,5
        // 6,7,8

        //Borde oeste -> x = 0
        let resultado = Coordenadas2D::obtener_coordenada_adyacente(0, ancho);
        let esperado = vec![0, 1];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        //Borde este -> x = ancho - 1
        let resultado = Coordenadas2D::obtener_coordenada_adyacente(ancho - 1, ancho);
        let esperado = vec![ancho - 2, ancho - 1];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }
        //Borde norte -> y = 0 // Analogo a borde oeste
        let resultado = Coordenadas2D::obtener_coordenada_adyacente(0, largo);
        let esperado = vec![0, 1];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }
        //Borde sur -> y = largo - 1 // Analogo a borde este
        let resultado = Coordenadas2D::obtener_coordenada_adyacente(largo - 1, largo);
        let esperado = vec![largo - 2, largo - 1];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        //Coordenada interna -> x o y, distinto a los casos anteriores
        let resultado = Coordenadas2D::obtener_coordenada_adyacente(ancho - 2, ancho);
        let esperado = vec![ancho - 3, ancho - 2, ancho - 1];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        //Coordenada interna -> x o y, distinto a los casos anteriores
        let resultado = Coordenadas2D::obtener_coordenada_adyacente(largo - 2, largo);
        let esperado = vec![largo - 3, largo - 2, largo - 1];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }
    }

    #[test]
    fn obtener_coordenadas_adyacentes() {
        //Asumo un mapa 3x3
        let ancho = 3;
        let largo = 3;
        // (0,0) , (1,0) , (2,0)
        // (0,1) , (1,1) , (2,1)
        // (0,2) , (1,2) , (2,2)
        let coordenada = Coordenadas2D::new(0, 0);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(0, 1),
            Coordenadas2D::new(1, 0),
            Coordenadas2D::new(1, 1),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        let coordenada = Coordenadas2D::new(1, 0);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(0, 0),
            Coordenadas2D::new(0, 1),
            Coordenadas2D::new(1, 1),
            Coordenadas2D::new(2, 0),
            Coordenadas2D::new(2, 1),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        let coordenada = Coordenadas2D::new(2, 0);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(1, 0),
            Coordenadas2D::new(1, 1),
            Coordenadas2D::new(2, 1),
            Coordenadas2D::new(1, 1),
            Coordenadas2D::new(1, 2),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        let coordenada = Coordenadas2D::new(0, 1);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(0, 0),
            Coordenadas2D::new(0, 2),
            Coordenadas2D::new(1, 0),
            Coordenadas2D::new(1, 1),
            Coordenadas2D::new(1, 2),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        let coordenada = Coordenadas2D::new(1, 1);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(0, 0),
            Coordenadas2D::new(0, 1),
            Coordenadas2D::new(0, 2),
            Coordenadas2D::new(1, 0),
            Coordenadas2D::new(1, 2),
            Coordenadas2D::new(2, 0),
            Coordenadas2D::new(2, 1),
            Coordenadas2D::new(2, 2),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        let coordenada = Coordenadas2D::new(2, 1);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(1, 0),
            Coordenadas2D::new(1, 1),
            Coordenadas2D::new(1, 2),
            Coordenadas2D::new(2, 0),
            Coordenadas2D::new(2, 2),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        let coordenada = Coordenadas2D::new(0, 2);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(0, 1),
            Coordenadas2D::new(1, 1),
            Coordenadas2D::new(1, 2),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        let coordenada = Coordenadas2D::new(1, 2);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(0, 1),
            Coordenadas2D::new(0, 2),
            Coordenadas2D::new(1, 1),
            Coordenadas2D::new(2, 1),
            Coordenadas2D::new(2, 2),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }

        let coordenada = Coordenadas2D::new(2, 2);
        let resultado = coordenada.coordenadas_adyacentes(ancho, largo);
        let esperado = vec![
            Coordenadas2D::new(1, 1),
            Coordenadas2D::new(1, 2),
            Coordenadas2D::new(2, 1),
        ];
        for (r, e) in resultado.iter().zip(esperado.iter()) {
            assert_eq!(r, e);
        }
    }
}
