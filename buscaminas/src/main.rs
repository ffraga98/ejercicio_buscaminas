use buscaminas::error::error_io::ErrorIO;
use buscaminas::error::Error;
use buscaminas::error::Error::Eio;
use buscaminas::my_io::escritor::Escritor;
use buscaminas::my_io::lector::Lector;
use buscaminas::tablero::builder::TableroBuilder;
use std::env::args;

const ARCHIVO_SOLUCION: &str = "solucion.txt";

fn main() -> Result<(), Error> {
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
        return Err(Eio(ErrorIO::ErrorCLI));
    }
    let path = &args[1];

    let lector = Lector::new(path);

    let casilleros = lector.leer_archivo()?;

    let builder = TableroBuilder::new(&casilleros)?;

    let solucion = builder.crear_tablero()?;

    let escritor = Escritor::new(ARCHIVO_SOLUCION);

    escritor.imprimir_item(&solucion)?;

    Ok(())
}
