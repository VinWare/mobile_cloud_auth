// mod self::point;
use super::point as point;

use point::Point;

pub struct EllipticCurve {
    pub order: u32,
    pub o: Point,
    pub g: Point,
}
