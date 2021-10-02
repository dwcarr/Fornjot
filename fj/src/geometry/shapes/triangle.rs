use crate::math::Point;

pub struct Triangle<const D: usize>([Point<D>; 3]);

impl<const D: usize> Triangle<D> {
    /// Create a new `Triangle`
    pub fn new(a: Point<D>, b: Point<D>, c: Point<D>) -> Result<Self, Error> {
        if a == b || a == c || b == c {
            return Err(Error::CollapsedPoints);
        }
        if (b - a).normalize() == (c - b).normalize() {
            return Err(Error::IsALineSegment);
        }

        // TASK: Normalize triangle.
        Ok(Self([a, b, c]))
    }

    /// Return the points of the triangle
    pub fn points(&self) -> [Point<D>; 3] {
        self.0
    }
}

pub enum Error {
    CollapsedPoints,
    IsALineSegment,
}

#[cfg(test)]
mod tests {
    use nalgebra::point;

    use super::Triangle;

    #[test]
    fn validation() {
        let triangle =
            Triangle::new(point![0., 0.], point![0., 1.], point![1., 1.]);
        let points_on_a_line =
            Triangle::new(point![0., 0.], point![1., 1.], point![2., 2.]);
        let collapsed_points =
            Triangle::new(point![0., 0.], point![1., 1.], point![1., 1.]);

        assert!(triangle.is_ok());
        assert!(points_on_a_line.is_err());
        assert!(collapsed_points.is_err());
    }
}