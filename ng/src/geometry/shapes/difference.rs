use crate::{
    geometry::{
        bounding_volume::Aabb,
        edges::Edges,
        faces::{triangulate, Triangle},
        Shape,
    },
    math::Point,
};

impl Shape for fj::Difference {
    fn bounding_volume(&self) -> Aabb {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }

    fn edges(&self) -> Edges {
        // TASK: This method assumes that `b` is fully contained within `a`. As
        //       long as this precondition exists, it should at least be
        //       checked.
        self.a
            .edges()
            .into_iter()
            .chain(self.b.edges().into_iter().map(|edge| edge.reverse()))
            .collect()
    }

    fn triangles(&self, tolerance: f64) -> Vec<Triangle> {
        // TASK: Carefully think about the limits of this algorithm, and make
        //       sure to panic with a `todo!` in cases that are not supported.

        let a: Vec<_> = self
            .a
            .edges()
            .into_iter()
            .map(|edge| edge.vertices(tolerance))
            .flatten()
            .collect();
        let b: Vec<_> = self
            .b
            .edges()
            .into_iter()
            .map(|edge| edge.vertices(tolerance))
            .flatten()
            .collect();

        let mut vertices = Vec::new();
        vertices.extend(&a);
        vertices.extend(&b);

        let mut triangles = triangulate(&vertices);

        // Now we have a full Delaunay triangulation of all vertices. We still
        // need to filter out the triangles that aren't actually part of the
        // difference.
        triangles.retain(|triangle| {
            let mut edges_of_b = 0;

            for [v0, v1] in triangle.edges() {
                if b.contains(&v0) && b.contains(&v1) {
                    edges_of_b += 1;
                }
            }

            edges_of_b <= 1
        });

        triangles
    }

    fn vertices(&self) -> Vec<Point> {
        // TASK: Implement.
        todo!()
    }
}