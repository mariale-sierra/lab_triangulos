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
(Resultado de `cargo test`: 6 casos, ambos algoritmos y orden inverso.)

## Mediciones
| Algoritmo | Mediana (100 ejecuciones) | Fragmentos |
|---|---|---|
| Wireframe | | |
| Baricéntrico | | |
| Scanline | | |

## Flamegraph

## Conclusiones
