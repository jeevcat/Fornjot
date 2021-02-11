pub mod point;
pub mod segment;

use crate::geometry::triangulation::trapezoidation::segment::Segment;

use super::{
    graph::Graph,
    update::{x_split, y_split},
};

pub fn insert(segment: Segment, graph: &mut Graph) {
    if let Some(id) = point::insert(segment.upper(), graph) {
        y_split::update(id, graph);
    }
    if let Some(id) = point::insert(segment.lower(), graph) {
        y_split::update(id, graph);
    }

    let ids = segment::insert(segment, graph);
    x_split::update(&ids, graph);
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph::{self, Node, X, Y},
        point::Point,
        segment::Segment,
    };

    use super::insert;

    // Looks useless, but actually makes sure that our calls to `Graph::new`
    // pick up the default type parameters, without us having to add an
    // additional type hint.
    type Graph = graph::Graph;

    #[test]
    fn insert_should_insert_upper_point_then_lower_point_then_segment() {
        let mut graph = Graph::new();

        let segment =
            Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)).unwrap();

        insert(segment, &mut graph);

        let below = match graph.get(graph.source()) {
            Node::Y(Y { point, below, .. }) => {
                assert_eq!(point, &segment.upper());
                *below
            }
            node => panic!("Unexpected node: {:?}", node),
        };

        let above = match graph.get(below) {
            Node::Y(Y { point, above, .. }) => {
                assert_eq!(point, &segment.lower());
                *above
            }
            node => panic!("Unexpected node: {:?}", node),
        };

        match graph.get(above) {
            Node::X(X { segment: s, .. }) => {
                assert_eq!(s, &segment);
            }
            node => panic!("Unexpected node: {:?}", node),
        }
    }
}
