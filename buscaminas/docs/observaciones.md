# Observaciones
## Uso de la memoria
El archivo tablero tendrá una entrada y salida en formato **ASCII**. Los **Strings** de Rust y los **&str** son **UTF-8**. Dado que esperamos que **&str** sea **ASCII**, se puede invocar **.as_bytes()** y referirnos a los datos interiores a través del slice **&[u8]**. Iterar sobre un slice de u8 es mucho más rápido porque no hay verificaciones de codificación de caracteres. (cada ASCII tiene el tamaño de un u8).

Escrbir el programa sin clonar (.clone()) el input.
## Recibir el input
El input es un archivo en el filesystem con el formato de entrada del tablero. 
En la invocación del programa se debe proveer la ruta a ese archivo.