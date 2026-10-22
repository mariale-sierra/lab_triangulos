mod triangle;

use std::hint::black_box;
use std::time::Instant;
use triangle::*;

fn sample_triangles() -> Vec<[Vec2; 3]> {
    let v = |x, y| Vec2 { x, y };
    vec![
        [v(5., 5.), v(8., 5.), v(5., 9.)],          // pequeño
        [v(50., 40.), v(750., 100.), v(300., 560.)], // grande
        [v(100., 100.), v(500., 100.), v(300., 400.)], // borde horizontal
        [v(600., 100.), v(600., 500.), v(700., 300.)], // borde vertical
        [v(-100., -50.), v(400., 200.), v(100., 700.)], // parcialmente fuera
        [v(10., 590.), v(790., 595.), v(400., 20.)],  // delgado/grande
    ]
}

fn benchmark_triangles(triangles: &[[Vec2; 3]]) {
    for (name, mode) in [
        ("wireframe", TriangleMode::Wireframe),
        ("barycentric", TriangleMode::FilledBarycentric),
        ("scanline", TriangleMode::FilledScanline),
    ] {
        let mut times = Vec::new();
        let mut n_frags = 0;
        for _ in 0..11 {
            let start = Instant::now();
            for _ in 0..100 {
                let f = rasterize(black_box(triangles), mode);
                n_frags = f.len();
                black_box(f);
            }
            times.push(start.elapsed());
        }
        times.sort();
        println!(
            "{name}: mediana {:?} / 100 ejecuciones, {n_frags} fragmentos por ejecución",
            times[times.len() / 2]
        );
    }
}

/// Escribe un PPM (se puede convertir a PNG) para las imágenes del README.
fn save_ppm(path: &str, frags: &[Fragment]) {
    let mut buf = vec![0u8; (WIDTH * HEIGHT * 3) as usize];
    for f in frags {
        let i = ((f.y * WIDTH + f.x) * 3) as usize;
        buf[i] = (f.color >> 16) as u8;
        buf[i + 1] = (f.color >> 8) as u8;
        buf[i + 2] = f.color as u8;
    }
    let mut data = format!("P6\n{WIDTH} {HEIGHT}\n255\n").into_bytes();
    data.extend(buf);
    std::fs::write(path, data).unwrap();
}

fn main() {
    let triangles = sample_triangles();
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--benchmark-triangles") {
        benchmark_triangles(&triangles);
        return;
    }
    if args.iter().any(|a| a == "--save-images") {
        save_ppm("wireframe.ppm", &rasterize(&triangles, TriangleMode::Wireframe));
        save_ppm("barycentric.ppm", &rasterize(&triangles, TriangleMode::FilledBarycentric));
        save_ppm("scanline.ppm", &rasterize(&triangles, TriangleMode::FilledScanline));
        println!("Imágenes guardadas (.ppm)");
        return;
    }
    println!("Usa --benchmark-triangles o --save-images");
}
