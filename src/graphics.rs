use glam::Mat4;

#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub normal: [f32; 3],
}

#[derive(Clone, Copy)]
pub enum Primitive {
    Unknown,
    Points,
    Lines,
    LineStrips,
    LineLoops,
    Triangles,
    TriangleStrips,
    TriangleFans,
}

impl Default for Primitive {
    fn default() -> Self {
        Primitive::Unknown
    }
}

pub struct Shape {
    primitive: Primitive,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            primitive: Primitive::Triangles,
            vertices: Vec::default(),
            indices: Vec::default(),
        }
    }
}

impl Shape {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_primitive(mut self, primitive: Primitive) -> Self {
        self.primitive = primitive;
        self
    }

    pub fn with_vertices(mut self, vertices: Vec<Vertex>) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn with_indices(mut self, indices: Vec<u32>) -> Self {
        self.indices = indices;
        self
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    } 

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    } 

    pub fn primitive(&self) -> Primitive {
        self.primitive
    } 
}

pub struct DrawCall {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub transform: Mat4,
}

pub struct Graphics {
    draw_calls: Vec<DrawCall>,
}

impl Default for Graphics {
    fn default() -> Self {
        Self { draw_calls: Default::default() }
    }
}

impl Graphics {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn draw(mut self, shape: &Shape, transform: Mat4) -> Self {
        self.draw_calls.push(
            DrawCall {
                vertices: shape.vertices.clone(),
                indices: shape.indices.clone(),
                transform,
            }
        );
        self
    }

    pub fn flush_draws(self) -> Vec<DrawCall> {
        self.draw_calls
    } 
}