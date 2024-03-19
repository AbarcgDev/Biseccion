# Metodo de Biseccion

Este programa es una implementacion sencilla del metodo de biseccion para encontrar
las raices de una funcion.

# Uso del Programa

### Ejemplo de Uso

```bash
./biseccion --funcion="x-cos(x)" --variable="x" -a=0 -b=1 --precision=10e-15 
```

El programa necesita como argumentos los siguientes parametros:

+ Funcion (--funcion -f): Es una string que represente, en formato lineal, la funcion a resolver
como es esperado soporta strings de funciones polinomicas, por ejemplo:
    - x^3 + 2x + 4
    - 2x + 3
    - 4x - x^4 + x^3 - 2
Las funciones trascendentales deben escribirse utilizando las expresiones en formato lineal,de la siguiente forma:
    - Raiz Cuadrada: sqrt()
    - Valor absoluto: abs()
    - Exponencial (e^x): exp()
    - Logaritmo Natural: ln()
    - Logaritmo base 10: log10()
    - Seno: sin()
    - Coseno: cos()
    - Tangente: tan()
    - Arco Seno: asin()
    - Arco Coseno: acos()
    - Arco Tangente: atan()
    - Las versiones hiperbolicas de las funciones trigonometricas llevan una h al final (sinh, asinh...)

Tambien soporta las constantes:
    - pi
    - e

Mas informacion sobre la conversion de expresiones matematicas en: [meval-rs](https://github.com/rekka/meval-rs?tab=readme-ov-file)

+ A (-a): Es el limite inferior del intervalo alrededor del cual buscar la raiz 
+ B (-b): Es el limite superior del intervalo alrededor del cual buscar la raiz

Los siguientes parametros son opcionales segun los requerimientos

+ Variable (--variable): Es la variable con respecto a cual se expresa la funcion, el valor por defecto es X,
en caso de tener una funcion expresada en una variable distinta (de un solo caracter),
se debe utilizar este parametro para indicar cual es la variable, de lo contrario el programa no funcionara

+ Precison (--precision): El programa calculara una aproximacion de la raiz solicitada con una precision de 10e-10,
en caso de requerir mayor o menor precision se puede modificar con ese parametro
