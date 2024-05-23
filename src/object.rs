use glam::{Mat4, Vec3};

use crate::{color::Color, graphics::Vertex, shapes::{Cube, Plane}};

/// Represents a 3D object with vertices, indices, and a transform matrix.
pub struct Object {
    vertices: Vec<Vertex>,
    indices: Vec<i32>,
    transform: Mat4,
}

impl Default for Object {
    fn default() -> Self {
        Self {
            vertices: Default::default(),
            indices: Default::default(),
            transform: Default::default(),
        }
    }
}

impl Object {
    /// Creates a new object from vertices and indices.
    pub fn new_mesh(vertices: Vec<Vertex>, indices: Vec<i32>) -> Self {
        Self {
            vertices,
            indices,
            ..Default::default()
        }
    }

    /// Creates a new cube object with the specified color.
    pub fn new_cube(color: Color) -> Self {
        Self {
            vertices: Cube::vertices(color),
            indices: Cube::indices(),
            transform: Mat4::IDENTITY,
        }
    }

    /// Creates a new plane object with the specified color.
    pub fn new_plane(color: Color) -> Self {
        Self {
            vertices: Plane::vertices(color),
            indices: Plane::indices(),
            transform: Mat4::IDENTITY,
        }
    }

    /// Sets the transform matrix for the object.
    pub fn with_transform(mut self, transform: &Mat4) -> Self {
        self.transform = *transform;
        self
    }

    /// Translates the object by the given vector.
    pub fn with_translation(mut self, translation: Vec3) -> Self {
        self.transform = Mat4::from_translation(translation) * self.transform;
        self
    }

    /// Rotates the object around the X axis.
    pub fn with_rotation_x(mut self, angle: f32) -> Self {
        self.transform = Mat4::from_rotation_x(angle) * self.transform;
        self
    }

    /// Rotates the object around the Y axis.
    pub fn with_rotation_y(mut self, angle: f32) -> Self {
        self.transform = Mat4::from_rotation_y(angle) * self.transform;
        self
    }

    /// Rotates the object around the Z axis.
    pub fn with_rotation_z(mut self, angle: f32) -> Self {
        self.transform = Mat4::from_rotation_z(angle) * self.transform;
        self
    }

    /// Scales the object by the given vector.
    pub fn with_scale(&mut self, scale: Vec3) -> &mut Self {
        self.transform = Mat4::from_scale(scale) * self.transform;
        self
    }

    /// Translates the object by the given vector.
    pub fn translate(&mut self, translation: Vec3) -> &mut Self {
        self.transform *= Mat4::from_translation(translation);
        self
    }

    /// Rotates the object around the X axis.
    pub fn rotate_x(&mut self, angle: f32) -> &mut Self {
        self.transform *= Mat4::from_rotation_x(angle);
        self
    }

    /// Rotates the object around the Y axis.
    pub fn rotate_y(&mut self, angle: f32) -> &mut Self {
        self.transform *= Mat4::from_rotation_y(angle);
        self
    }

    /// Rotates the object around the Z axis.
    pub fn rotate_z(&mut self, angle: f32) -> &mut Self {
        self.transform *= Mat4::from_rotation_z(angle);
        self
    }

    /// Scales the object by the given vector.
    pub fn scale(&mut self, scale: Vec3) -> &mut Self {
        self.transform *= Mat4::from_scale(scale);
        self
    }

    /// Returns the transform matrix of the object.
    pub fn transform(&self) -> &Mat4 {
        &self.transform
    }

    /// Returns the vertices of the object.
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    /// Returns the indices of the object.
    pub fn indices(&self) -> &Vec<i32> {
        &self.indices
    }
}
