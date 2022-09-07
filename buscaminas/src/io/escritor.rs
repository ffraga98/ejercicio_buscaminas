//! # Escritor
//! `escritor` es un submodulo para facilitar la escritura de contenido en archivos de texto.

use crate::error::ErrorIO;
use std::fmt::Display;
use std::fs;
use std::io::Write;

/// Estructura que capaz de escribir en un archivo
#[derive(Debug, PartialEq)]
pub struct Escritor<'a> {
    /// Ruta del archivo a escribir
    path: &'a str,
}

impl<'a> Escritor<'a> {
    ///Construye un nuevo escritor, con el `path` inicializado con el `&str` que se pase por paramentro.
    /// # Ejemplos
    /// ```
    /// let escritor = Escritor::new("./foo/filename.txt");
    /// ```
    pub fn new(path: &str) -> Escritor {
        Escritor { path }
    }

    /// Escribe item, que recibe por referencia, en el archivo `path`. El item puede ser cualquiera que implemente el Trait `Display`.
    /// # Ejemplos
    /// ```
    /// let escritor = Escritor::new("./foo/filename.txt");
    /// let texto = String::from("texto ejemplo");
    /// escritor.imprimir_item(&texto).expect("No ha sido posible escribir en el archivo.");
    /// ```
    pub fn imprimir_item<T: Display>(&self, item: &T) -> Result<(), ErrorIO> {
        let mut f = match fs::File::create(self.path) {
            Ok(f) => f,
            Err(error) => return Err(ErrorIO::ErrorFile(error)),
        };
        if let Err(error) = f.write_all(item.to_string().as_bytes()) {
            return Err(ErrorIO::ErrorWrite(error));
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crear_escritor() {
        let resultado = Escritor::new("test_files/test.txt");
        let esperado = Escritor { path: "test_files/test.txt" };
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn escritura_archivo_inexistente() {
        let escritor = Escritor::new("test_files/test.txt");
        let palabra = String::from("hola");
        let resultado = escritor.imprimir_item(&palabra);
        assert!(!resultado.is_err());
    }
}
