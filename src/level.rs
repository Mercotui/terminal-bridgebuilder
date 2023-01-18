/// Coordinates in 3 dimensions
#[derive(Debug, PartialEq)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}

/// An index of a vertex
#[derive(Debug, PartialEq, Eq)]
pub struct VertexIndex(pub usize);

/// An edge between two vertices
#[derive(Debug, PartialEq, Eq)]
pub struct Edge(pub VertexIndex, pub VertexIndex);

/// A vehicle that will cross the bridge
#[derive(Debug, PartialEq)]
pub struct Vehicle {
    pub name: String,
    pub position: Coordinates,
    pub rotation: f64,
}

/// A construction build from edges of multiple types
#[derive(Debug, PartialEq, Eq, Default)]
pub struct Bridge {
    pub steel: Vec<Edge>,
    pub wood: Vec<Edge>,
    pub wire: Vec<Edge>,
    pub road: Vec<Edge>,
}

/// A full level
#[derive(Debug, PartialEq, Default)]
pub struct Level {
    pub background: String,
    pub vertices: Vec<Coordinates>,
    pub anchors: Vec<VertexIndex>,
    pub road: Vec<Edge>,
    pub bridge: Bridge,
    pub vehicles: Vec<Vehicle>,
}
