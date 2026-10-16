// LOCAL PROBE, never committed.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use kurbo::{Point, Shape};
use vectorcraft_trace::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let img = Raster::decode_to_fit(&std::fs::read(&a[1]).unwrap()).unwrap();
    let p = preset(&a[2]).unwrap();
    let t = std::time::Instant::now();
    let r = trace_within(&img, &p, 2_000_000);
    let r = match r { Ok(r) => r, Err(e) => { println!("ERR {e}"); return; } };
    println!("{}x{} paths {} anchors {} colours {} ({:?})", img.width, img.height, r.paths.len(), r.anchor_count(), r.palette.len(), t.elapsed());
    println!("palette {:?}", r.palette);
    if a.get(3).map(String::as_str) != Some("scan") { return; }
    let bez: Vec<_> = r.paths.iter().map(|t| { let b = t.path.to_bezpath(); let bb = b.bounding_box(); (b, bb) }).collect();
    let (w, h) = (img.width as i64, img.height as i64);
    // bucket paths by 32px tiles
    let ts = 32i64; let tw = (w + ts - 1) / ts; let th = (h + ts - 1) / ts;
    let mut tiles: Vec<Vec<usize>> = vec![vec![]; (tw * th) as usize];
    for (i, (_, bb)) in bez.iter().enumerate() {
        let x0 = ((bb.x0 / ts as f64).floor() as i64).max(0); let x1 = ((bb.x1 / ts as f64).floor() as i64).min(tw - 1);
        let y0 = ((bb.y0 / ts as f64).floor() as i64).max(0); let y1 = ((bb.y1 / ts as f64).floor() as i64).min(th - 1);
        for ty in y0..=y1 { for tx in x0..=x1 { tiles[(ty * tw + tx) as usize].push(i); } }
    }
    let mut holes = vec![]; let mut over = 0usize;
    for y in 0..h { for x in 0..w {
        let pt = Point::new(x as f64 + 0.5, y as f64 + 0.5);
        let tl = &tiles[((y / ts) * tw + x / ts) as usize];
        let n = tl.iter().filter(|&&i| bez[i].1.contains(pt) && bez[i].0.winding(pt) != 0).count();
        if n == 0 { holes.push((x, y)); } else if n > 1 { over += 1; }
    }}
    println!("uncovered centres {} overlapped {}", holes.len(), over);
    for (x, y) in holes.iter().take(60) { println!("  hole {x},{y}"); }
}
