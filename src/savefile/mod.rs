#[cfg(test)]
mod unittest;

use crate::level::{Bridge, Coordinates, Edge, Level, Object, Vehicle, VertexIndex};
use anyhow::{anyhow, ensure, Context, Result};
use json;
use json::{parse, Array, JsonValue};
use std::fs;

pub fn load(path: &std::path::PathBuf) -> Result<Level> {
    let json_string = fs::read_to_string(path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;

    let json_data = json::parse(&json_string)
        .with_context(|| format!("Could not parse as json `{}`", path.display()))?;

    parse_level(&json_data).with_context(|| format!("Could not parse level `{}`", json_data))
}

fn parse_level(root_object: &json::JsonValue) -> Result<Level> {
    let foreground = &root_object["foreground"];
    ensure!(
        foreground.is_object(),
        "Expected foreground object, found {}",
        foreground
    );

    Ok(Level {
        background: parse_background(&root_object["background"]).context("Invalid background")?,
        vertices: parse_vertices(&foreground["vertices"]).context("Invalid vertices")?,
        anchors: parse_vertex_indices(&foreground["anchors"]).context("Invalid anchors")?,
        road: parse_edges(&foreground["road"]).context("Invalid road")?,
        bridge: parse_bridge(&foreground["bridge"]).context("Invalid bridge")?,
        vehicles: parse_vehicles(&foreground["vehicles"]).context("Invalid vehicles")?,
    })
}

fn parse_background(root_object: &json::JsonValue) -> Result<String> {
    Ok(String::from(""))
}

fn parse_bridge(root_object: &json::JsonValue) -> Result<Bridge> {
    let members = &root_object["members"];
    Ok(Bridge {
        steel: parse_edges(&members["steel"])?,
        wood: parse_edges(&members["wood"])?,
        wire: parse_edges(&members["wire"])?,
        road: parse_edges(&members["road"])?,
    })
}

fn parse_vehicles(root_object: &json::JsonValue) -> Result<Vec<Vehicle>> {
    ensure!(
        root_object.is_array(),
        "Vehicles should be an array, instead found {}",
        root_object
    );

    let mut vehicles: Vec<Vehicle> = vec![];
    for vehicle in root_object.members() {
        vehicles.push(parse_vehicle(vehicle)?);
    }
    Ok(vehicles)
}

fn parse_vehicle(root_object: &json::JsonValue) -> Result<Vehicle> {
    Ok(Vehicle {
        name: root_object["type"]
            .as_str()
            .context("Vehicle type was not string")?
            .to_string(),
        position: parse_coordinates(&root_object["position"])?,
    })
}

fn parse_vertices(root_object: &json::JsonValue) -> Result<Vec<Coordinates>> {
    ensure!(
        root_object.is_array(),
        "Vertices should be an array of coordinates, instead found {}",
        root_object
    );
    let mut vertices: Vec<Coordinates> = vec![];
    for vertex_json in root_object.members() {
        vertices.push(parse_coordinates(vertex_json)?);
    }
    Ok(vertices)
}

fn parse_vertex_indices(root_object: &JsonValue) -> Result<Vec<VertexIndex>> {
    let mut vertex_indices: Vec<VertexIndex> = vec![];
    for vertex_index_json in root_object.members() {
        vertex_indices.push(VertexIndex(
            vertex_index_json
                .as_u64()
                .context("Not an unsigned integer")?,
        ));
    }
    Ok(vertex_indices)
}

fn parse_edges(root_object: &json::JsonValue) -> Result<Vec<Edge>> {
    let mut edges: Vec<Edge> = vec![];
    for edge_json in root_object.members() {
        let mut vertex_indices = parse_vertex_indices(edge_json)?;
        ensure!(
            vertex_indices.len() == 2,
            "Edges should contain two vertex indices, instead found {}",
            edge_json
        );
        // TODO(Menno 03.12.2022) Find cleaner conversion between Vec and tuple
        edges.push(Edge(vertex_indices.remove(0), vertex_indices.remove(0)));
    }
    Ok(edges)
}

fn parse_coordinates(root_object: &JsonValue) -> Result<Coordinates> {
    ensure!(
        root_object.len() == 2,
        "Coordinates should contain two numbers, instead found {}",
        root_object
    );
    Ok(Coordinates {
        x: root_object[0]
            .as_f64()
            .context("First value is not a number")?,
        y: root_object[1]
            .as_f64()
            .context("Second value is not a number")?,
    })
}

pub fn save(objects: &Vec<Object>) -> Result<()> {
    Err(anyhow!("Saving is not implemented yet. {:?}", objects))
}
