use buscaminas::error::error_io::ErrorIO;
use buscaminas::error::Error;
use buscaminas::error::Error::Eio;
use buscaminas::my_io::escritor::Escritor;
use buscaminas::my_io::lector::Lector;
use buscaminas::tablero::builder::TableroBuilder;
use buscaminas::ARCHIVO_SOLUCION;
use std::env::args;

fn main() -> Result<(), Error> {
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
