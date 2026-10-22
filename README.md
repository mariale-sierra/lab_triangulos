# Laboratorio: relleno de triángulos

## Uso
```
cargo test
cargo run --release -- --benchmark-triangles
cargo run --release -- --save-images        # genera wireframe/barycentric/scanline .ppm
cargo flamegraph -- --benchmark-triangles   # genera flamegraph.svg
```
Las imágenes .ppm se pueden convertir a PNG (p. ej. `convert barycentric.ppm barycentric.png`).

## Imágenes
<img width="800" height="600" alt="barycentric" src="https://github.com/user-attachments/assets/6fc9c3d6-63b5-40f6-b6bc-b3144fc3bdbf" />
<img width="800" height="600" alt="scanline" src="https://github.com/user-attachments/assets/ca34aef3-e6d7-4b6f-8510-5447ee3c63ad" />
<img width="800" height="600" alt="wireframe" src="https://github.com/user-attachments/assets/2e940aa9-14d4-41ef-8759-1dfa93ce2772" />


## Regla de bordes
Se evalúa el centro (x+0.5, y+0.5). Un centro sobre un borde izquierdo o superior cuenta como dentro; sobre uno derecho o inferior, fuera (regla top-left).
- Baricéntrico: funciones de borde; si w == 0 se acepta solo si el borde es top-left. Área 0 → vacío. Área negativa → se intercambian dos vértices.
- Scanline: intervalo vertical semiabierto [y_min, y_max) por arista (sin división por 0 en horizontales, vértices contados una vez) e intervalo horizontal [xl, xr) por fila.

## Resultados de pruebas
`cargo test`: 6 pruebas pasadas (triángulo pequeño, grande, borde horizontal,
borde vertical, parcialmente fuera del framebuffer y vértices colineales).
En cada caso se compararon las posiciones generadas por ambos algoritmos
(no solo la imagen) y también con el orden de vértices invertido: coinciden
exactamente. Los colineales producen 0 fragmentos en ambos. No se encontraron
diferencias en bordes porque ambos usan la misma regla top-left sobre el centro
del píxel.

## Mediciones
Mediana de 11 mediciones, cada una de 100 ejecuciones, con `cargo run --release`.

| Algoritmo | Mediana (100 ejecuciones) | Fragmentos por ejecución |
|---|---|---|
| Wireframe | 4.90 ms | 6 508 |
| Baricéntrico | 356.70 ms | 619 663 |
| Scanline | 106.23 ms | 619 663 |

## Flamegraph
<img width="1763" height="977" alt="flamegraph" src="https://github.com/user-attachments/assets/60279593-ff05-44f9-94c3-dd68c5bd3bd2" />
El perfil se tomó ejecutando `cargo flamegraph -- --benchmark-triangles`
(7 497 muestras; perf reportó algunas muestras perdidas, por lo que los
porcentajes son aproximados). Incluye los tres modos de dibujo en la misma
ejecución, así que los porcentajes son sobre el tiempo total del benchmark.

| Función | Ancho aprox. |
|---|---|
| `triangle_barycentric` | ~63 % |
| `Vec::extend` en `rasterize` (copia de fragmentos, `memcpy`) | ~27 % |
| `triangle_scanline` | ~10 % |
| Wireframe | <1 % |

Dentro de `triangle_barycentric`, alrededor de dos tercios es el bucle propio
sobre el rectángulo envolvente (comparaciones y recorrido de píxeles), ~13 %
es la función `edge` y ~9 % es `Vec::push`. En `triangle_scanline` casi todo
lo visible es `Vec::push`: el cálculo de intersecciones por fila casi no
aparece.

Nota: el ancho de un bloque indica la proporción de muestras en que apareció
esa función, no el orden de ejecución. Los tiempos de esta corrida no se usan
en la tabla de mediciones porque perf añade sobrecarga.

## Conclusiones
- Ambos rellenos generan exactamente las mismas posiciones (619 663
  fragmentos), así que la comparación de tiempos es justa: ninguno omite
  trabajo.
- Scanline fue unas 3.4 veces más rápido que baricéntrico (106 ms vs 357 ms).
  El flamegraph lo explica: baricéntrico gasta la mayor parte del tiempo en su
  bucle interior (evaluar tres funciones de borde por cada píxel del
  rectángulo envolvente, incluidos los que quedan fuera del triángulo),
  mientras que scanline solo visita píxeles interiores y su costo geométrico
  es casi despreciable.
- En scanline lo que domina es la creación de fragmentos (`Vec::push`), no la
  geometría; en baricéntrico es la geometría. Esto es coherente con que
  baricéntrico trabaja sobre muchos más píxeles candidatos que fragmentos
  produce.
- Aproximadamente una cuarta parte del tiempo total se va en copiar los
  fragmentos de cada triángulo al `Vec` global con `extend` (`memcpy`). Es un
  costo de diseño, no del algoritmo, y lo pagan ambos modos por igual.
  Mejoras posibles: pasar un `&mut Vec<Fragment>` a las funciones de relleno
  para escribir directamente en él, o usar `Vec::with_capacity` con una
  estimación del área del triángulo para evitar reasignaciones.
- Baricéntrico tiene a su favor que ya calcula las coordenadas baricéntricas,
  lo que facilita interpolar atributos (color, profundidad, textura). En
  scanline habría que interpolar a lo largo de las aristas y de cada tramo.
  Cuando se interpolen atributos, la diferencia de rendimiento podría
  cambiar.
- Wireframe es muchísimo más barato (6 508 fragmentos, ~5 ms) porque su costo
  depende del largo de los bordes y no del área.
