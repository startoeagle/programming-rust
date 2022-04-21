use octree::Point;

#[derive(Debug, PartialEq)]
pub enum KdTree {
    Empty,
    NonEmpty(Node),
}

#[derive(Debug, PartialEq)]
pub struct Node {
    point: Point,
    left: Box<KdTree>,
    right: Box<KdTree>,
}

impl KdTree {
    pub fn from_vec(points: Vec<Point>) -> KdTree {
        fn go(mut points: Vec<Point>, depth: usize) -> KdTree {
            if points.is_empty() {
                return KdTree::Empty;
            }

            const DIM: usize = 3;
            let axis = depth % DIM;

            let point_index = |p: &Point| -> f32 { return p.into_iter().nth(axis).unwrap() };

            pdqsort::sort_by(&mut points, |p, q| {
                point_index(&p).partial_cmp(&point_index(&q)).unwrap()
            });

            let index = points.len() / 2;
            let median = points.iter().nth(index).unwrap().clone();
            let _ = points.swap_remove(index);

            let (lower, higher) = points
                .into_iter()
                .partition(|p| p.into_iter().nth(axis).unwrap() < point_index(&median));

            // create new trees using rayon to make a divide and conquer

            let (left, right) = rayon::join(
                || Box::new(go(lower, depth + 1)),
                || Box::new(go(higher, depth + 1)),
            );

            KdTree::NonEmpty(Node {
                point: median,
                left,
                right,
            })
        }

        go(points, 0)
    }
}

#[cfg(test)]
mod tests {
    use octree::Point;

    use crate::KdTree;

    #[test]
    fn create_kdtree() {
        let npoints = 5;
        let points: Vec<Point> = std::iter::from_fn(|| Some(Point::random()))
            .take(npoints)
            .collect();

        // at least no overflow
        let tree = KdTree::from_vec(points);

        println!("{:#?}", tree);
    }

    #[test]
    fn test_correct_kdtree() {
        use crate::KdTree::{Empty, NonEmpty};
        use crate::Node;

        let tree = NonEmpty(Node {
            point: Point {
                x: 0.48508537,
                y: 0.55833566,
                z: 0.3701563,
            },
            left: Box::new(NonEmpty(Node {
                point: Point {
                    x: 0.4760617,
                    y: 0.73099136,
                    z: 0.38488424,
                },
                left: Box::new(NonEmpty(Node {
                    point: Point {
                        x: 0.30849802,
                        y: 0.37044013,
                        z: 0.017071009,
                    },
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
                right: Box::new(Empty),
            })),
            right: Box::new(NonEmpty(Node {
                point: Point {
                    x: 0.62909675,
                    y: 0.422158,
                    z: 0.7771108,
                },
                left: Box::new(NonEmpty(Node {
                    point: Point {
                        x: 0.72619,
                        y: 0.084477186,
                        z: 0.3141619,
                    },
                    left: Box::new(Empty),
                    right: Box::new(Empty),
                })),
                right: Box::new(Empty),
            })),
        });

        let pointcloud = vec![
            Point {
                x: 0.48508537,
                y: 0.55833566,
                z: 0.3701563,
            },
            Point {
                x: 0.4760617,
                y: 0.73099136,
                z: 0.38488424,
            },
            Point {
                x: 0.30849802,
                y: 0.37044013,
                z: 0.017071009,
            },
            Point {
                x: 0.62909675,
                y: 0.422158,
                z: 0.7771108,
            },
            Point {
                x: 0.72619,
                y: 0.084477186,
                z: 0.3141619,
            },
        ];

        assert_eq!(tree, KdTree::from_vec(pointcloud));
    }
}
