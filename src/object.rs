use crate::{mat4::Mat4, Vec3};
use std::{error::Error, fs};

#[derive(Clone)]
pub struct Object {
    pub transformation: Mat4,
    pub vertices: Vec<Vec3>,
    pub faces: Vec<(usize, usize, usize)>,
}

impl Object {
    // Loads in a 3D model from a .obj file
    pub fn from_obj(path: &str) -> Result<Object, Box<dyn Error>> {
        let obj_str =
            fs::read_to_string(path).expect(format!("Failed to load object at {}", path).as_str());

        let mut vertices: Vec<Vec3> = vec![];
        let mut faces: Vec<(usize, usize, usize)> = vec![];

        for line in obj_str.split("\r\n") {
            let tokens: Vec<&str> = line.split(" ").into_iter().collect::<Vec<&str>>();
            let line_type = tokens[0];

            match line_type {
                "v" => {
                    if tokens.len() == 4 {
                        let x: f64 = tokens[1].parse()?;
                        let y: f64 = tokens[2].parse()?;
                        let z: f64 = tokens[3].parse()?;
                        vertices.push(Vec3::new(x, y, z));
                    }
                }
                "f" => {
                    if tokens.len() == 4 {
                        let x: usize = get_face(tokens[1]).parse()?;
                        let y: usize = get_face(tokens[2]).parse()?;
                        let z: usize = get_face(tokens[3]).parse()?;
                        faces.push((x, y, z));
                    }
                }
                &_ => (),
            }
        }

        Ok(Object {
            transformation: Mat4::identity(),
            vertices,
            faces,
        })
    }
}

/**
Extracts the first part (vertex index) from part of an 'f' .obj file line.
 */
fn get_face(face_string: &str) -> &str {
    face_string.split("/").into_iter().collect::<Vec<&str>>()[0]
}
