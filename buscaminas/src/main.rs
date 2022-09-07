use crate::io::escritor::Escritor;
use crate::io::lector::Lector;
use crate::tablero::builder::TableroBuilder;
use std::env::args;

pub mod error;
pub mod io;
pub mod tablero;

const ARCHIVO_SOLUCION: &str = "solucion.txt";

fn main() -> Result<(), ()> {
    //DONE: Al invocarse se debe proveer el nombre del archivo.
    //DONE: Modularizacion
    //DONE: Unit Testing
    //TODO: Integration Testing
    //TODO: Documentacion 
        //TODO: Cambiar links
        //TODO: Cambiar ejemplos, que sean ejecutables 
        //TODO: Cambiar ejemplos, no haya unwraps
        //TODO: Cambiar ejemplos, no haya unwraps
    //DONE:   * error
    //TODO: Separar en archivos distintos los errores.
    //TODO:   * io
    //TODO:     - escritor -> Errores
    //TODO:     - lector -> Errores
    //TODO:   * tablero
    //TODO:     - builder
    //TODO:     - casillero
    //TODO:     - coordenada
    //TODO:     - tablero


    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        //FIXME: Agregar error
        println!("Para resolver el problema, se requiere unicamente de la ruta del archivo.");
        return Err(())
    }
    let path = &args[1];

    let lector = Lector::new(path);

    let casilleros = match lector.leer_archivo() {
        Ok(valores) => valores,
        Err(error) => {
            println!("{}", error);
            return Err(())
        }
    };

    let builder = match TableroBuilder::new(&casilleros) {
        Ok(builder) => builder,
        Err(error) => {
            println!("{}", error);
            return Err(())
        }
    };

    let solucion = match builder.crear_tablero() {
        Ok(tablero) => tablero,
        Err(error) => {
            println!("{}", error);
            return Err(())
        }
    };

    let escritor = Escritor::new(ARCHIVO_SOLUCION);
    if let Err(error) = escritor.imprimir_item(&solucion) {
        println!("{}", error);
        return Err(())
    }

    Ok(())
}
