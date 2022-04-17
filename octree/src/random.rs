use crate::*;

use rand::random;

impl Point {
    pub fn random() -> Point {
        Point{ x: random(), y: random(), z: random() } 
    }

}
