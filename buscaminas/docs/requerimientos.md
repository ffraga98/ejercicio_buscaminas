# Requerimientos no funcionales

Los siguientes son los requerimientos no funcionale para la resolución del proyecto:

- El proyecto deberá ser deasrrollado en lenguaje **Rust**, usando las herramientas de las biblioteca estándar.

- Se deben implementar **test unitarios y de integración**  de las funcionalidades que se consideren más importantes.

- No se permite utilizar crates externos.

- El código fuente debe compilarse en la versión estable del compilador y no se permite utilizar bloques *unsafe*.

- El código deberá funcionar en ambiente Unix / Linux.

- El programa deberá ejecutarse en la línea de comandos.

- La compilación no debe arrojar wanings del compilador, ni del linter clippy.

- Las funciones y los tipos de datos (struct) deben estar documentados siguiendo el estándar de `cargo doc`.

- El código debe formatearse utilizando `cargo fmt`.

- Las funciones no deben tener una exensión mayor de las 30 líneas. Si se requieriera una extensión mayor, se deberá particionarla en varias funciones.

- Cada tipo de dato implementado debe ser colocado en una unidad de compilación (archivo fuente) independiente.
