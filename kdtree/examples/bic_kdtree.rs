use kdtree::KdTree;
use octree::Point;
use std::time;

fn main() {
    let npoints = 640 * 480;
    let points: Vec<Point> = std::iter::from_fn(|| Some(Point::random()))
        .take(npoints)
        .collect();

    // at least no overflow
    let now = time::Instant::now();
    let _tree = KdTree::from_vec(points);

    println!(
        "KdTree for {} points in {} ms",
        npoints,
        now.elapsed().as_millis()
    )
}
