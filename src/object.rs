use crate::{mat4::Mat4, vec2::Vec2, Vec3};
use std::{
    error::Error,
    fs::{self, File},
    vec,
};

/**
Holds data relating to a single face of an object - the tuples refer to lists of indexes
*/
pub struct Face {
    pub vertices: (usize, usize, usize),
    pub tex_coords: (usize, usize, usize),
}

/**
Holds a pixel buffer, along with the dimensions of the image it represents
 */
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Texture {
    pub fn sample(self, x: usize, y: usize) -> u32 {
        let i = self.width * y + x;
        self.pixels[i]
    }
}
/**
Holds data parsed from a .obj file
- `transformation` is the transformation applied to this objects vertices
- `vertices` are the 3D coordinates that make up the object
- `tex_coords` are coordinates within the texture
- `texture` is a pixel buffer containing a texture for the object
- `faces` is a list of the faces that make up the object
    - The data in a `Face` object is a set of indexes referring to vertices and texture coordinates - when each face is drawn,
        its texture and vertices must be accessed from their corresponding fields
*/
pub struct Object {
    pub transformation: Mat4,
    pub vertices: Vec<Vec3>,
    pub tex_coords: Vec<Vec2>,
    pub texture: Texture,
    pub faces: Vec<Face>,
}

impl Object {
    /**
    Loads in a 3D model from a .obj file
    */
    pub fn from_obj(name: &str) -> Result<Object, Box<dyn Error>> {
        let obj_path = String::from(name) + ".obj";
        let obj_str = fs::read_to_string(obj_path)
            .expect(format!("Failed to load object at {}", name).as_str());

        let mut vertices: Vec<Vec3> = vec![];
        let mut tex_coords: Vec<Vec2> = vec![];
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
                    if vec_buffer.len() == 3 {
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
                    if coord_buffer.len() == 2 {
                        tex_coords.push(Vec2::new(coord_buffer[0], coord_buffer[1]));
                    }
                }
                "f" => {
                    // As the renderer only deals with triangles, faces of more than three points must be split into triangles
                    let mut slice_offset = 1;

                    // Loop over triangular faces on the current line
                    while slice_offset + 2 < tokens.len() {
                        let mut vertices: Vec<usize> = vec![];
                        let mut tex_coords: Vec<usize> = vec![];

                        // Parses three sets of vertex and texture coord from the current offset
                        for i in 0..3 {
                            let line_index = slice_offset + i;

                            let vertex = get_face_data(tokens[line_index], 0);
                            let tex_coord = get_face_data(tokens[line_index], 1);

                            vertices.push(vertex);
                            tex_coords.push(tex_coord);
                        }

                        let face = Face {
                            vertices: (vertices[0], vertices[1], vertices[2]),
                            tex_coords: (tex_coords[0], tex_coords[1], tex_coords[2]),
                        };
                        faces.push(face);
                        slice_offset += 1;
                    }
                }
                &_ => (),
            }
        }

        let texture_path = String::from(name) + ".png";
        let texture = load_texture(&texture_path);

        Ok(Object {
            transformation: Mat4::identity(),
            vertices,
            tex_coords,
            faces,
            texture,
        })
    }

    pub fn transform(&mut self, mat: Mat4) {
        self.transformation = self.transformation.mult(mat);
    }
}

/**
Extracts a 'usize' value from a slash delimited string at a given index
 */
fn get_face_data(face_string: &str, index: usize) -> usize {
    face_string.split('/').collect::<Vec<&str>>()[index]
        .parse::<usize>()
        .expect(&format!("Typo in .obj file near {}", face_string))
}

/**
Loads a PNG texture file into a pixel buffer
 */
fn load_texture(path: &str) -> Texture {
    println!("{}", path);
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    let mut pixels: Vec<u32> = vec![];
    for i in (0..bytes.len() - 2).step_by(3) {
        // Shift some bytes around to get an 32 bit colour value
        let rgba = (bytes[i] as u32) << 16 | (bytes[i + 1] as u32) << 8 | bytes[i + 2] as u32;
        pixels.push(rgba);
    }
    Texture {
        width: info.width as usize,
        height: info.height as usize,
        pixels,
    }
}
