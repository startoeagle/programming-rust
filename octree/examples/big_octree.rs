use octree::{Octree, Point};
use std::time;
fn main() {
    let npoints = 640 * 480;
    let points: Vec<Point> = std::iter::from_fn(|| Some(Point::random()))
        .take(npoints)
        .collect();
    let now = time::Instant::now();
    let _octree = Octree::from_slice(points);
    println!(
        "Octree for {} points in {} ms",
        npoints,
        now.elapsed().as_millis()
    )
}
