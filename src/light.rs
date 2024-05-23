use glam::Vec3;

/// Represents a light source in the scene.
pub struct Light {
    position: Vec3,
    color: Color,
    intensity: f32,
}

impl Light {
    /// Creates a new light source with the given position, color, and intensity.
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }

    /// Sets the position of the light.
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Sets the color of the light.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Sets the intensity of the light.
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Returns the position of the light.
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Returns the color of the light.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Returns the intensity of the light.
    pub fn intensity(&self) -> f32 {
        self.intensity
    }
}
