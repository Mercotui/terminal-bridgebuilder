use crate::level;
use anyhow::{Context, Result};

/// Scene coordinates
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}

impl Coordinates {
    fn new(level_coordinates: &level::Coordinates) -> Self {
        Coordinates {
            x: level_coordinates.x,
            y: level_coordinates.y,
        }
    }
}

/// A pair of Scene coordinates, creating a line between the pair
#[derive(Debug, Copy, Clone)]
pub struct Line(pub(crate) Coordinates, pub(crate) Coordinates);

#[derive(Debug, Copy, Clone)]
pub struct Background {
    pub line: Line,
    pub color: u16,
}

/// The material a beam is made of
#[derive(Debug, Copy, Clone)]
pub enum BeamMaterial {
    Wood,
    Steel,
    Road,
}

/// A rigid structural member
#[derive(Debug, Copy, Clone)]
pub struct Beam {
    pub material: BeamMaterial,
    pub line: Line,
    pub is_static: bool,
}

/// The material a Wire is made of
#[derive(Debug, Copy, Clone)]
pub enum WireMaterial {
    Steel,
}

/// A flexible structural member that can only maintain tension
#[derive(Debug, Copy, Clone)]
pub struct Wire {
    pub material: WireMaterial,
    pub line: Line,
}

/// The type of vehicle
#[derive(Debug, Copy, Clone)]
pub enum VehicleType {
    _Bus,
    _Car,
}

/// A vehicle that can self propel along a road
#[derive(Debug, Copy, Clone)]
pub struct _Vehicle {
    pub vehicle_type: VehicleType,
    pub position: Coordinates,
    pub rotation: f64,
}

/// Generalization of all scene objects
#[derive(Debug, Copy, Clone)]
pub enum Object {
    Wire(Wire),
    Beam(Beam),
    _Vehicle(_Vehicle),
}

// TODO(Menno 28.12.2022) Implement ordering for objects
// impl Object {
//     fn cmp (&self, other: &Self){
//         if std::mem::discriminant(self) == std::mem::discriminant( other) {
//
//         }
//         match other {
//
//         }
//     }
// }

/// A level transformed into renderable objects
#[derive(Default, Clone)]
pub struct Scene(pub Vec<Object>);

// TODO(Menno 28.12.2022) implement iterator for Scene references to make the Vec private
impl IntoIterator for Scene {
    type Item = Object;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Scene {
    pub fn new(level: &level::Level) -> Result<Self> {
        let mut scene: Self = Default::default();
        // Add static roads
        scene.0.append(&mut Self::convert_beams(
            &level.vertices,
            &level.road,
            BeamMaterial::Road,
            true,
        )?);

        // Add all bridge members
        scene.0.append(&mut Self::convert_beams(
            &level.vertices,
            &level.bridge.road,
            BeamMaterial::Road,
            false,
        )?);
        scene.0.append(&mut Self::convert_beams(
            &level.vertices,
            &level.bridge.wood,
            BeamMaterial::Wood,
            false,
        )?);
        scene.0.append(&mut Self::convert_beams(
            &level.vertices,
            &level.bridge.steel,
            BeamMaterial::Steel,
            false,
        )?);
        scene.0.append(&mut Self::convert_wires(
            &level.vertices,
            &level.bridge.wire,
            WireMaterial::Steel,
        )?);

        // TODO(Menno 26.12.2022) Convert remaining level objects into scene objects, i.e. vehicles and the background

        // TODO(Menno 28.12.2022) Once ordering is implemented, sort the scene here
        // We need to sort the scene along the Z axis so that objects are drawn on top of each-other correctly
        // scene
        //     .0
        //     .sort_by(|a, b| a..partial_cmp(&b.depth()).unwrap_or(Ordering::Less));

        Ok(scene)
    }

    fn convert_beams(
        vertices: &[level::Coordinates],
        edges: &[level::Edge],
        material: BeamMaterial,
        is_static: bool,
    ) -> Result<Vec<Object>> {
        let mut objects: Vec<Object> = vec![];
        for edge in edges {
            let vertex_a = Self::get_coordinates(&edge.0, vertices)?;
            let vertex_b = Self::get_coordinates(&edge.1, vertices)?;

            objects.push(Object::Beam(Beam {
                material,
                line: Line(vertex_a, vertex_b),
                is_static,
            }));
        }
        Ok(objects)
    }

    fn convert_wires(
        vertices: &[level::Coordinates],
        edges: &[level::Edge],
        material: WireMaterial,
    ) -> Result<Vec<Object>> {
        let mut objects: Vec<Object> = vec![];
        for edge in edges {
            let vertex_a = Self::get_coordinates(&edge.0, vertices)?;
            let vertex_b = Self::get_coordinates(&edge.1, vertices)?;

            objects.push(Object::Wire(Wire {
                material,
                line: Line(vertex_a, vertex_b),
            }));
        }
        Ok(objects)
    }

    fn get_coordinates(
        index: &level::VertexIndex,
        vertices: &[level::Coordinates],
    ) -> Result<Coordinates> {
        Ok(Coordinates::new(
            vertices
                .get(index.0)
                .context("Could not find vertex at specified index")?,
        ))
    }
}
