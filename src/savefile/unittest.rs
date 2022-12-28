use crate::level::{Bridge, Coordinates, Edge, Level, Vehicle, VertexIndex};
use crate::savefile::{
    parse_bridge, parse_coordinates, parse_edges, parse_level, parse_vehicle, parse_vehicles,
    parse_vertex_indices, parse_vertices,
};
use json::{array, JsonValue};

#[test]
fn parse_level_test() {
    // Test valid values
    assert_eq!(
        parse_level(
            &json::parse(
                r#"{
                  "background": {
                  },
                  "foreground": {
                    "vertices": [[0, 1], [3, 1], [1, 1], [2, 1], [1.5, 1], [1.5, 2]],
                    "anchors": [0, 1, 2, 3],
                    "road": [[0, 2], [1, 3]],
                    "bridge": {
                      "members": {
                        "steel": [],
                        "wood": [[2, 5], [3, 5], [5, 4]],
                        "wire": [],
                        "road": [[2, 4], [3, 4]]
                      }
                    },
                    "vehicles": [
                      {
                        "type": "car",
                        "position": [0.5, 1]
                      }
                    ]
                  }
                }"#
            )
            .unwrap()
        )
        .unwrap(),
        Level {
            background: "".to_string(),
            vertices: vec![
                Coordinates { x: 0.0, y: 1.0 },
                Coordinates { x: 3.0, y: 1.0 },
                Coordinates { x: 1.0, y: 1.0 },
                Coordinates { x: 2.0, y: 1.0 },
                Coordinates { x: 1.5, y: 1.0 },
                Coordinates { x: 1.5, y: 2.0 }
            ],
            bridge: Bridge {
                steel: vec![],
                wood: vec![
                    Edge(VertexIndex(2), VertexIndex(5)),
                    Edge(VertexIndex(3), VertexIndex(5)),
                    Edge(VertexIndex(5), VertexIndex(4))
                ],
                wire: vec![],
                road: vec![
                    Edge(VertexIndex(2), VertexIndex(4)),
                    Edge(VertexIndex(3), VertexIndex(4))
                ],
            },
            anchors: vec![
                VertexIndex(0),
                VertexIndex(1),
                VertexIndex(2),
                VertexIndex(3)
            ],
            road: vec![
                Edge(VertexIndex(0), VertexIndex(2)),
                Edge(VertexIndex(1), VertexIndex(3))
            ],
            vehicles: vec![Vehicle {
                name: "car".to_string(),
                position: Coordinates { x: 0.5, y: 1.0 }
            }],
        }
    );
}

#[test]
fn parse_background_test() {
    // TODO(Menno 08.12.2022) Implement test once background dats structure has been decided
}

#[test]
fn parse_bridge_test() {
    // Test valid values
    assert_eq!(
        parse_bridge(
            &json::parse(
                r#"{
                "members": {
                    "steel": [[0, 1], [1, 2]],
                    "wood": [[2, 3]],
                    "wire": [[1, 3]],
                    "road": [[0,2], [2, 3]]
                }
            }"#
            )
            .unwrap()
        )
        .unwrap(),
        Bridge {
            steel: vec![
                Edge(VertexIndex(0), VertexIndex(1)),
                Edge(VertexIndex(1), VertexIndex(2))
            ],
            wood: vec![Edge(VertexIndex(2), VertexIndex(3))],
            wire: vec![Edge(VertexIndex(1), VertexIndex(3))],
            road: vec![
                Edge(VertexIndex(0), VertexIndex(2)),
                Edge(VertexIndex(2), VertexIndex(3))
            ]
        }
    );
}

#[test]
fn parse_vehicles_test() {
    // Test valid values
    assert_eq!(parse_vehicles(
        &json::parse(
            r#"[{"type": "car", "position": [5.0, 6.66]}, {"type": "bus", "position": [10.0, 20.0]}]"#
        ).unwrap()).unwrap()
               , vec![Vehicle { name: "car".to_string(), position: Coordinates { x: 5.0, y: 6.66,  } },
                      Vehicle { name: "bus".to_string(), position: Coordinates { x: 10.0, y: 20.0, } }]);
}

#[test]
fn parse_vehicle_test() {
    // Test that null values result in an error
    assert!(parse_vehicle(&JsonValue::Null).is_err());

    // Test that a non-string name value results in an error
    assert!(
        parse_vehicle(&json::parse(r#"{"type": 42, "position": [1.2, 2.5]}"#).unwrap()).is_err()
    );

    // Test that missing coordinates results in an error
    assert!(parse_vehicle(&json::parse(r#"{"type": "hey"}"#).unwrap()).is_err());

    // Test valid value
    assert_eq!(
        parse_vehicle(&json::parse(r#"{"type": "hey", "position": [1.2, 2.5]}"#).unwrap()).unwrap(),
        Vehicle {
            name: "hey".to_string(),
            position: Coordinates { x: 1.2, y: 2.5 },
        }
    );
}

#[test]
fn parse_vertices_test() {
    let mut coordinates = array![];

    // Test that null values result in an error
    assert!(parse_vertices(&JsonValue::Null).is_err());

    // Test valid values
    coordinates.push(array![0.0, 0.5]).unwrap();
    coordinates.push(array![42.42, 6.28]).unwrap();

    assert_eq!(
        parse_vertices(&coordinates).unwrap(),
        vec![
            Coordinates { x: 0.0, y: 0.5 },
            Coordinates { x: 42.42, y: 6.28 }
        ]
    )
}

#[test]
fn parse_vertex_indices_test() {
    // Test that invalid values result in error
    assert!(parse_vertex_indices(&array![1, "unexpected text!"]).is_err());

    // Test that negative indices result in error
    assert!(parse_vertex_indices(&array![1, -2]).is_err());

    // Test that valid values
    assert_eq!(
        parse_vertex_indices(&array![0, 1, 52]).unwrap(),
        vec![VertexIndex(0), VertexIndex(1), VertexIndex(52)]
    );
}

#[test]
fn parse_edges_test() {
    let mut edges = array![];

    // Test that negative indices result in error
    edges.push(array![1, -2]).unwrap();
    let result = parse_edges(&edges);
    assert!(result.is_err());

    // Test valid indices
    edges.clear();
    edges.push(array![1, 50]).unwrap();
    edges.push(array![42, 100]).unwrap();
    assert_eq!(
        parse_edges(&edges).unwrap(),
        vec![
            Edge(VertexIndex(1), VertexIndex(50)),
            Edge(VertexIndex(42), VertexIndex(100)),
        ]
    );
}

#[test]
fn parse_coordinates_test() {
    // Test that null values result in an error
    assert!(parse_coordinates(&JsonValue::Null).is_err());

    // Test that valid values
    assert_eq!(
        parse_coordinates(&json::parse("[3.14, 0.5]").unwrap()).unwrap(),
        Coordinates { x: 3.14, y: 0.5 }
    );
}
