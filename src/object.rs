use crate::{mat4::Mat4, vec2::Vec2, Vec3};
use std::{
    fs::{self, File},
    io,
    num::ParseIntError,
    vec,
};

#[derive(Debug)]
pub enum ModelLoadError {
    IoError(io::Error),
    ParseError(ParseIntError),
}

impl From<io::Error> for ModelLoadError {
    fn from(error: io::Error) -> Self {
        ModelLoadError::IoError(error)
    }
}
impl From<ParseIntError> for ModelLoadError {
    fn from(error: ParseIntError) -> Self {
        ModelLoadError::ParseError(error)
    }
}

///
/// Holds data relating to a single face of an object - the tuples refer to lists of indexes
///
#[derive(Clone)]
pub struct Face {
    pub vertices: (usize, usize, usize),
    pub tex_coords: (usize, usize, usize),
    pub normals: (usize, usize, usize),
}

///
///Holds a pixel buffer, along with the dimensions of the image it represents
///
#[derive(Clone)]
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Texture {
    ///
    /// Sample a texture at `(x, y)`, where `x` and `y` are values between 0 and 1
    ///
    pub fn sample(&self, coords: Vec2) -> u32 {
        let x = coords.x * self.width as f64;
        let y = coords.y * self.height as f64;
        let mut i = self.width * y as usize + x as usize;
        while i > self.pixels.len() - 1 {
            i -= self.pixels.len();
        }
        self.pixels[i]
    }

    ///
    /// Loads a PNG texture from a given path into a u32 pixel buffer
    ///
    pub fn load_from(path: &str) -> Result<Texture, io::Error> {
        let decoder = png::Decoder::new(File::open(path)?);
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];

        let info = reader.next_frame(&mut buf).unwrap();

        let bytes = &buf[..info.buffer_size()];
        let mut pixels: Vec<u32> = vec![];

        for i in (0..bytes.len() - 2).step_by(3) {
            // Shift some bytes around to get an 32 bit colour value
            let rgba = (bytes[i] as u32) << 16 | (bytes[i + 1] as u32) << 8 | bytes[i + 2] as u32;
            pixels.push(rgba);
        }

        Ok(Texture {
            width: info.width as usize,
            height: info.height as usize,
            pixels,
        })
    }
}

///
/// Holds data parsed from a .obj file
/// - `vertices` are the 3D coordinates that make up the object
/// - `tex_coords` are coordinates within the texture
/// - `faces` is a list of the faces that make up the object
///     - The data in a `Face` object is a set of indexes referring to vertices and texture coordinates - when each face is drawn,
///         its texture and vertices must be accessed from their corresponding fields
/// - `texture` is a pixel buffer containing a texture for the object
/// - `transformation` is the transformation applied to this object in world space
///
#[derive(Clone)]
pub struct Object {
    pub vertices: Vec<Vec3>,
    pub tex_coords: Vec<Vec2>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub texture: Texture,
    pub transformation: Mat4,
}

impl Object {
    ///
    /// Loads in a 3D model from a .obj file
    ///
    pub fn from_obj(name: &str) -> Result<Object, ModelLoadError> {
        // IO operations
        let obj_path = String::from(name) + ".obj";
        let obj_str = fs::read_to_string(obj_path)?;
        let texture_path = String::from(name) + ".png";
        let texture = Texture::load_from(&texture_path)?;

        let mut vertices: Vec<Vec3> = vec![];
        let mut tex_coords: Vec<Vec2> = vec![];
        let mut normals: Vec<Vec3> = vec![];

        let mut faces: Vec<Face> = vec![];

        for line in obj_str.split("\r\n") {
            let tokens: Vec<&str> = line.split(" ").into_iter().collect::<Vec<&str>>();
            let line_type = tokens[0];
            match line_type {
                "v" => {
                    // Parse each token as a float - if 3 floats are found on the line, add them to a new Vec3
                    let mut vec_buffer: Vec<f64> = vec![];
                    for token in tokens {
                        if let Ok(f) = token.parse::<f64>() {
                            vec_buffer.push(f)
                        };
                    }
                    if vec_buffer.len() >= 3 {
                        vertices.push(Vec3::new(vec_buffer[0], vec_buffer[1], vec_buffer[2]));
                    }
                }
                "vt" => {
                    // As above
                    let mut coord_buffer: Vec<f64> = vec![];
                    for token in tokens {
                        if let Ok(f) = token.parse::<f64>() {
                            coord_buffer.push(f)
                        }
                    }
                    if coord_buffer.len() >= 2 {
                        tex_coords.push(Vec2::new(coord_buffer[0], coord_buffer[1]));
                    }
                }
                "vn" => {
                    // As above
                    let mut coord_buffer: Vec<f64> = vec![];
                    for token in tokens {
                        if let Ok(f) = token.parse::<f64>() {
                            coord_buffer.push(f)
                        }
                    }
                    if coord_buffer.len() >= 3 {
                        normals.push(Vec3::new(coord_buffer[0], coord_buffer[1], coord_buffer[2]));
                    }
                }
                "f" => {
                    // As the renderer only deals with triangles, faces of more than three points must be split into triangles
                    let mut slice_offset = 1;

                    // Loop over triangular faces on the current line
                    while slice_offset + 2 < tokens.len() {
                        let mut vertices: Vec<usize> = vec![];
                        let mut tex_coords: Vec<usize> = vec![];
                        let mut normals: Vec<usize> = vec![];

                        // Parses three sets of vertex and texture coord from the current offset
                        for i in 0..3 {
                            let line_index = slice_offset + i;

                            // .obj file indices start at 1, so make sure to normalise this type to zero indexed
                            if let Ok(vertex) = get_face_data(tokens[line_index], 0) {
                                vertices.push(vertex - 1);
                            }
                            if let Ok(tex_coord) = get_face_data(tokens[line_index], 1) {
                                tex_coords.push(tex_coord - 1);
                            }
                            if let Ok(normal) = get_face_data(tokens[line_index], 2) {
                                normals.push(normal - 1);
                            }
                        }

                        let face = Face {
                            vertices: (vertices[0], vertices[1], vertices[2]),
                            tex_coords: (tex_coords[0], tex_coords[1], tex_coords[2]),
                            normals: (normals[0], normals[1], normals[2]),
                        };
                        faces.push(face);
                        slice_offset += 1;
                    }
                }
                _ => (),
            }
        }

        Ok(Object {
            vertices,
            tex_coords,
            normals,
            faces,
            texture,
            transformation: Mat4::identity(),
        })
    }

    pub fn transform(&mut self, mat: Mat4) {
        self.transformation = self.transformation.mult(mat);
    }
}

///
/// Extracts a 'usize' value from a slash delimited string at a given index2
///
fn get_face_data(face_string: &str, index: usize) -> Result<usize, ParseIntError> {
    let slash_split = face_string.split('/').collect::<Vec<&str>>();
    if slash_split.len() > index {
        return slash_split[index].parse::<usize>();
    }
    Ok(0)
}
