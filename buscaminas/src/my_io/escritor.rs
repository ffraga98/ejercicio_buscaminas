//! # Escritor
//! `escritor` es un submodulo para facilitar la escritura de contenido en archivos de texto.

use crate::error::error_io::ErrorIO;
use std::fmt::Display;
use std::fs;
use std::io::Write;

/// Estructura que capaz de escribir en un archivo
#[derive(Debug, PartialEq, Eq)]
pub struct Escritor<'a> {
    /// Ruta del archivo a escribir
    path: &'a str,
}

impl<'a> Escritor<'a> {
    ///Construye un nuevo [`Escritor`], con el [path] inicializado con el `&str` que se pase por paramentro.
    ///
    /// [path]: ./struct.Escritor.html#structfield.path
    ///
    /// # Ejemplos
    /// ```
    /// # use buscaminas::my_io::escritor::Escritor;
    /// #
    /// # fn main() {
    /// let escritor = Escritor::new("./foo/filename.txt");
    /// # }
    /// ```
    pub fn new(path: &str) -> Escritor {
        Escritor { path }
    }

    /// Escribe item, que recibe por referencia, en el archivo `path`. El item puede ser cualquiera que implemente el Trait [`Display`].
    /// # Ejemplos
    /// ```
    /// # use buscaminas::my_io::escritor::Escritor;
    /// # use buscaminas::error::error_io::ErrorIO;
    /// # use std::fs;
    /// #
    /// # fn main() -> Result<(),ErrorIO> {
    /// let escritor = Escritor::new("./test_files/filename.txt");
    /// let texto = String::from("texto ejemplo");
    /// escritor.imprimir_item(&texto)?;
    /// # fs::remove_file("./test_files/filename.txt");
    /// # Ok(())
    /// # }
    /// ```
    /// # Errores
    /// Retorna un [`ErrorIO`] con dos valores posibles.
    /// - En caso de fallar la creacion del archivo, se lanza un [`ErrorFile`]
    /// - En caso de fallar la escritura en el archivo, se lanza un [`ErrorWrite`]
    ///
    /// [`ErrorWrite`]: ../../error/enum.ErrorIO.html#variant.ErrorWrite
    /// [`ErrorFile`]: ../../error/enum.ErrorIO.html#variant.ErrorFile
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
        let esperado = Escritor {
            path: "test_files/test.txt",
        };
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn escritura_archivo_inexistente() {
        let escritor = Escritor::new("test_files/test.txt");
        let palabra = String::from("hola");
        let resultado = escritor.imprimir_item(&palabra);
        assert!(resultado.is_ok());
        fs::remove_file("test_files/test.txt").unwrap();
    }
}