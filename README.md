# Buscaminas
**Taller de Programación I - 2do Cuatrimestre 2022**

## Objetivo
El objetivo del ejercicio consiste en agregar el recuento de minas en un tablero de *Buscaminas* completo.

## Introducción 
Buscaminas es un juego popular en el que el usuario tine que encontrat las minas usando pistas numéricas que indican cuántas minas exisen adyacentes (horizontal, vertical, diagonal) a un cuadrado particular. 

El tablero es un rectángulo compuesto por caracteres ('.'). Una mina se respresenta con un asterisco ('*').

Si un espacion no contiene minas adyacentes, se lo deja en blanco.

## Ejemplos
Por ejemplo, se puede recibir un tablero de 5x4 como el siguiente:

```
.*.*.
..*..
..*..
.....
```
El programa debe transformarlo en esto:

```
1*3*1
13*31
.2*2.
.111.
```
Debe recibirse como parámetro la ruta al archivo del tablero de entrada con ese formato.