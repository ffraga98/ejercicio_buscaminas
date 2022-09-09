//! # Lector
//! `lector` es un submodulo para facilitar la lectura de contenido en archivos de texto.

use crate::error::error_io::ErrorIO;
use crate::tablero::builder::NUEVA_LINEA_ICONO;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
/// Estructura capaz de leer contenido UTF-8 de un archivo de texto
pub struct Lector<'a> {
    /// Ruta al archivo de texto de lectura.
    path: &'a str,
}

impl<'a> Lector<'a> {
    ///Construye un Lector con el campo [path] igual al que se le pasa por parametro.    
    ///
    /// [path]: ./struct.Lector.html#structfield.path
    ///
    /// # Ejemplos
    /// ```
    /// # use buscaminas::my_io::lector::Lector;
    /// #
    /// # fn main() {
    ///
    /// let lector = Lector::new("./foo/filename.txt");
    ///
    /// # }
    /// ```
    pub fn new(path: &str) -> Lector {
        Lector { path }
    }

    ///Lee el archivo y retorna un [`String`] con el contenido del mismo.
    ///
    /// # Ejemplo
    ///
    /// ```
    /// # use buscaminas::my_io::lector::Lector;
    /// # use buscaminas::error::error_io::ErrorIO;
    /// #
    /// # fn main() -> Result<(),ErrorIO> {
    /// let lector = Lector::new("./test_files/foo.txt");
    /// let texto: String = lector.leer_archivo()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errores
    ///
    /// Retorna un [`ErrorIO`] con dos valores posibles.
    ///
    /// - En caso de fallar al abrir el archivo, se lanza un [`ErrorFile`]
    ///
    /// - En caso de fallar el almacenamiento de las lineas leídas en memoria, se lanza un [`ErrorBufReader`]
    ///
    /// [`ErrorBufReader`]: ../../error/enum.ErrorIO.html#variant.ErrorBufReader
    /// [`ErrorFile`]: ../../error/enum.ErrorIO.html#variant.ErrorFile
    ///
    ///
    pub fn leer_archivo(&self) -> Result<String, ErrorIO> {
        let file = match File::open(self.path) {
            Ok(f) => f,
            Err(error) => return Err(ErrorIO::ErrorFile(error)),
        };
        let reader = BufReader::new(file);
        let mut palabras = String::new();
        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(error) => return Err(ErrorIO::ErrorBufReader(error)),
            };
            palabras.push_str(line.as_str());
            // Separa los saltos de linea
            palabras.push_str((NUEVA_LINEA_ICONO as char).to_string().as_str());
        }
        Ok(palabras)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crear_lector() {
        let resultado = Lector::new("test_files/test.txt");
        let esperado = Lector {
            path: "test_files/test.txt",
        };
        let no_esperado = Lector { path: "test.txt" };

        assert_eq!(resultado, esperado);
        assert_ne!(resultado, no_esperado);
    }

    #[test]
    fn lectura_archivo_valido_ascii() {
        let lector = Lector::new("./test_files/test1.txt");
        let resultado = lector.leer_archivo().expect("Error al leer el archivo.");
        let esperado = ".**..*-..*..*-";
        assert_eq!(resultado, esperado);

        let sin_nueva_lineas = ".**..*..*..*";
        assert_ne!(resultado, sin_nueva_lineas);

        let sin_nueva_linea_al_final = ".**..*-..*..*";
        assert_ne!(resultado, sin_nueva_linea_al_final);
    }

    #[test]
    fn abrir_archivo_inexistente() {
        let lector = Lector::new("./test_file/test_not_found.txt");
        let resultado = lector.leer_archivo();

        assert!(resultado.is_err());
    }
}
