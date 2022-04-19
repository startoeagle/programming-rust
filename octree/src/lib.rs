use std::ops::Add;
use std::ops::Div;

mod random;

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl std::ops::Div<f32> for Point {
    type Output = Point;

    fn div(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl IntoIterator for Point {
    type Item = f32;

    type IntoIter = std::array::IntoIter<Self::Item, 3>;

    fn into_iter(self) -> Self::IntoIter {
        [self.x, self.y, self.z].into_iter()
    }
}

pub fn l2(lhs: &Point, rhs: &Point) -> f32 {
    lhs.into_iter()
        .zip(rhs.into_iter())
        .map(|(l, r)| (l - r) * (l - r))
        .sum::<f32>()
        .sqrt()
}

/// An takes a pivot point and that splits the remaning points into separate inner octrees
#[derive(Debug)]
pub enum Octree {
    Empty,
    NonEmpty(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    pub point: Point,
    pub leafs: [Octree; 8],
}

pub fn center_point(points: &[Point]) -> (usize, Point) {
    let center = points.into_iter().fold(Point::default(), |mean, next| {
        mean + (next.div(points.len() as f32))
    });
    if let Some((_, (index, &pivot))) = points
        .into_iter()
        .enumerate()
        .map(|(i, p)| (l2(&center, &p), (i, p)))
        .min_by(|(d1, _), (d2, _)| match d1.partial_cmp(d2) {
            Some(cmp) => cmp,
            _ => unreachable!(),
        })
    {
        (index, pivot)
    } else {
        unreachable!()
    }
}

impl Octree {
    /// Consumes a slice and retuns an Octree
    pub fn from_slice(mut points: Vec<Point>) -> Self {
        if points.len() == 0 {
            Octree::Empty
        } else {
            let (index, point) = center_point(&points);
            let _ = points.swap_remove(index);

            // split points around the pivot
            let (left, right): (Vec<Point>, Vec<Point>) =
                points.into_iter().partition(|p| p.x < point.x);
            let xsplit = [left, right];

            let nodes: Vec<Octree> = xsplit
                .into_iter()
                .flat_map(|split| {
                    let (left, right): (Vec<Point>, Vec<Point>) =
                        split.into_iter().partition(|p| p.y < point.y);
                    [left, right]
                })
                .flat_map(|split| {
                    let (left, right): (Vec<Point>, Vec<Point>) =
                        split.into_iter().partition(|p| p.z < point.z);
                    [left, right]
                })
                .map(|partition| Octree::from_slice(partition))
                .collect();

            let mut tmp = nodes.into_iter();
            let err_msg = "Could not create octree";
            let leafs = [
                tmp.next().expect(err_msg),
                tmp.next().expect(err_msg),
                tmp.next().expect(err_msg),
                tmp.next().expect(err_msg),
                tmp.next().expect(err_msg),
                tmp.next().expect(err_msg),
                tmp.next().expect(err_msg),
                tmp.next().expect(err_msg),
            ];

            let node = Box::new(Node { point, leafs });
            Octree::NonEmpty(node)
        }
    }
}
#[cfg(test)]
mod tests {

    use crate::*;
    #[test]
    fn construct_an_octree() {
        let points: Vec<Point> = std::iter::from_fn(|| Some(Point::random()))
            .take(1000)
            .collect();
        let _octree = Octree::from_slice(points);
    }
}
