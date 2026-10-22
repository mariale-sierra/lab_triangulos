//! Relleno de triángulos: baricéntrico y por líneas horizontales.
//! Regla común: se evalúa el centro (x+0.5, y+0.5); un centro sobre un borde
//! izquierdo/superior cuenta como dentro, sobre derecho/inferior como fuera.

#[derive(Clone, Copy)]
pub enum TriangleMode {
    Wireframe,
    FilledBarycentric,
    FilledScanline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Fragment {
    pub x: i32,
    pub y: i32,
    pub color: u32,
}

pub const WIDTH: i32 = 800;
pub const HEIGHT: i32 = 600;

fn edge(a: Vec2, b: Vec2, p: Vec2) -> f32 {
    (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x)
}

// Con área > 0 (y hacia abajo): borde superior = horizontal hacia +x; izquierdo = dy < 0
fn is_top_left(a: Vec2, b: Vec2) -> bool {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    (dy == 0.0 && dx > 0.0) || dy < 0.0
}

/// Línea de Bresenham (para Wireframe). Sustituye por tu `line` real.
pub fn line(a: Vec2, b: Vec2, w: i32, h: i32, color: u32) -> Vec<Fragment> {
    let (mut x0, mut y0) = (a.x.round() as i32, a.y.round() as i32);
    let (x1, y1) = (b.x.round() as i32, b.y.round() as i32);
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut out = Vec::new();
    loop {
        if x0 >= 0 && x0 < w && y0 >= 0 && y0 < h {
            out.push(Fragment { x: x0, y: y0, color });
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    out
}

pub fn triangle_wireframe(a: Vec2, b: Vec2, c: Vec2, w: i32, h: i32, color: u32) -> Vec<Fragment> {
    let mut out = line(a, b, w, h, color);
    out.extend(line(b, c, w, h, color));
    out.extend(line(c, a, w, h, color));
    out
}

pub fn triangle_barycentric(
    v0: Vec2,
    mut v1: Vec2,
    mut v2: Vec2,
    w: i32,
    h: i32,
    color: u32,
) -> Vec<Fragment> {
    let mut area = edge(v0, v1, v2);
    if area == 0.0 {
        return Vec::new(); // degenerado
    }
    if area < 0.0 {
        std::mem::swap(&mut v1, &mut v2); // orden inverso
        area = -area;
    }
    let _ = area; // λi = wi / area, para interpolar atributos más adelante

    let min_x = v0.x.min(v1.x).min(v2.x).floor().max(0.0) as i32;
    let max_x = (v0.x.max(v1.x).max(v2.x).ceil() as i32).min(w - 1);
    let min_y = v0.y.min(v1.y).min(v2.y).floor().max(0.0) as i32;
    let max_y = (v0.y.max(v1.y).max(v2.y).ceil() as i32).min(h - 1);

    let tl0 = is_top_left(v1, v2);
    let tl1 = is_top_left(v2, v0);
    let tl2 = is_top_left(v0, v1);

    let mut out = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vec2 { x: x as f32 + 0.5, y: y as f32 + 0.5 };
            let w0 = edge(v1, v2, p);
            let w1 = edge(v2, v0, p);
            let w2 = edge(v0, v1, p);
            let inside = (w0 > 0.0 || (w0 == 0.0 && tl0))
                && (w1 > 0.0 || (w1 == 0.0 && tl1))
                && (w2 > 0.0 || (w2 == 0.0 && tl2));
            if inside {
                out.push(Fragment { x, y, color });
            }
        }
    }
    out
}

pub fn triangle_scanline(a: Vec2, b: Vec2, c: Vec2, w: i32, h: i32, color: u32) -> Vec<Fragment> {
    if edge(a, b, c) == 0.0 {
        return Vec::new();
    }
    let mut v = [a, b, c];
    v.sort_by(|p, q| p.y.partial_cmp(&q.y).unwrap());
    let [top, mid, bot] = v;

    // x donde la arista p->q cruza cy; None si cy no está en [p.y, q.y)
    let cross = |p: Vec2, q: Vec2, cy: f32| -> Option<f32> {
        if p.y <= cy && cy < q.y {
            Some(p.x + (cy - p.y) * (q.x - p.x) / (q.y - p.y))
        } else {
            None
        }
    };

    let y0 = (top.y - 0.5).ceil().max(0.0) as i32;
    let y1 = ((bot.y - 0.5).ceil() as i32).min(h);

    let mut out = Vec::new();
    for y in y0..y1 {
        let cy = y as f32 + 0.5;
        let long = cross(top, bot, cy);
        let short = cross(top, mid, cy).or_else(|| cross(mid, bot, cy));
        let (Some(xa), Some(xb)) = (long, short) else { continue };
        let (xl, xr) = (xa.min(xb), xa.max(xb));
        let x0 = ((xl - 0.5).ceil() as i32).max(0);
        let x1 = ((xr - 0.5).ceil() as i32).min(w);
        for x in x0..x1 {
            out.push(Fragment { x, y, color });
        }
    }
    out
}

/// Rasteriza una lista de triángulos con el modo elegido.
pub fn rasterize(triangles: &[[Vec2; 3]], mode: TriangleMode) -> Vec<Fragment> {
    let color = 0xFFFFFF;
    let mut frags = Vec::new();
    for t in triangles {
        let (a, b, c) = (t[0], t[1], t[2]);
        match mode {
            TriangleMode::Wireframe => frags.extend(triangle_wireframe(a, b, c, WIDTH, HEIGHT, color)),
            TriangleMode::FilledBarycentric => frags.extend(triangle_barycentric(a, b, c, WIDTH, HEIGHT, color)),
            TriangleMode::FilledScanline => frags.extend(triangle_scanline(a, b, c, WIDTH, HEIGHT, color)),
        }
    }
    frags
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn set(f: Vec<Fragment>) -> HashSet<(i32, i32)> {
        f.into_iter().map(|f| (f.x, f.y)).collect()
    }
    fn v(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
    fn check(a: Vec2, b: Vec2, c: Vec2) {
        let (w, h) = (100, 80);
        let s1 = set(triangle_barycentric(a, b, c, w, h, 0));
        let s2 = set(triangle_scanline(a, b, c, w, h, 0));
        let r1 = set(triangle_barycentric(a, c, b, w, h, 0));
        let r2 = set(triangle_scanline(a, c, b, w, h, 0));
        assert!(!s1.is_empty());
        assert_eq!(s1, s2, "difieren: {:?}", s1.symmetric_difference(&s2).collect::<Vec<_>>());
        assert_eq!(s1, r1);
        assert_eq!(s2, r2);
    }

    #[test] fn pequeno() { check(v(5., 5.), v(8., 5.), v(5., 9.)); }
    #[test] fn grande() { check(v(2., 3.), v(95., 10.), v(40., 75.)); }
    #[test] fn horizontal() { check(v(10., 10.), v(50., 10.), v(30., 40.)); }
    #[test] fn vertical() { check(v(10., 10.), v(10., 50.), v(40., 30.)); }
    #[test] fn fuera() { check(v(-20., -10.), v(60., 20.), v(10., 120.)); }
    #[test]
    fn colineales() {
        let (a, b, c) = (v(0., 0.), v(10., 10.), v(20., 20.));
        assert!(triangle_barycentric(a, b, c, 100, 80, 0).is_empty());
        assert!(triangle_scanline(a, b, c, 100, 80, 0).is_empty());
    }
}
