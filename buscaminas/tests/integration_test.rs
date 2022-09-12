use buscaminas::my_io::lector::Lector;
use buscaminas::my_io::escritor::Escritor;
use buscaminas::ARCHIVO_SOLUCION;
use buscaminas::tablero::builder::TableroBuilder;

#[test]
fn resuelve_test1() {
    let path = "test_files/test1.txt";
    let lector = Lector::new(path);
    let casilleros = lector.leer_archivo().unwrap();
    let builder = TableroBuilder::new(&casilleros).unwrap();
    let solucion = builder.crear_tablero().unwrap();
    let escritor = Escritor::new(ARCHIVO_SOLUCION);
    escritor.imprimir_item(&solucion).unwrap();
    let path = "solucion.txt";
    let lector = Lector::new(path);
    let casilleros = lector.leer_archivo().unwrap();
    assert_eq!(casilleros, "1**22*-13*22*-")
}

#[test]
fn resuelve_test2() {
    let path = "test_files/test2.txt";
    let lector = Lector::new(path);
    let casilleros = lector.leer_archivo().unwrap();
    let builder = TableroBuilder::new(&casilleros).unwrap();
    let solucion = builder.crear_tablero().unwrap();
    let escritor = Escritor::new(ARCHIVO_SOLUCION);
    escritor.imprimir_item(&solucion).unwrap();
    let path = "solucion.txt";
    let lector = Lector::new(path);
    let casilleros = lector.leer_archivo().unwrap();
    assert_eq!(casilleros, "124*2.-2***2.-2*421.-")
}

#[test]
fn resuelve_test_vacio() {
    let path = "test_files/test_vacio.txt";
    let lector = Lector::new(path);
    let casilleros = lector.leer_archivo().unwrap();
    let builder = TableroBuilder::new(&casilleros);
    assert!(builder.is_err());
}

#[test]
fn resuelve_test_malformado() {
    let path = "test_files/test_malformado.txt";
    let lector = Lector::new(path);
    let casilleros = lector.leer_archivo().unwrap();
    let builder = TableroBuilder::new(&casilleros).unwrap();
    let solucion = builder.crear_tablero();
    assert!(solucion.is_err());
}

#[test]
fn resuelve_test_no_ascii() {
    let path = "test_files/test_no_ascii.txt";
    let lector = Lector::new(path);
    let casilleros = lector.leer_archivo().unwrap();
    let builder = TableroBuilder::new(&casilleros).unwrap();
    let solucion = builder.crear_tablero();
    assert!(solucion.is_err());
}