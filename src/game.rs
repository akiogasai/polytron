use std::f32::consts::PI;
use glam::{vec2, vec3, Mat4, Vec3, Vec4Swizzles};
use crate::{color::Color, console::System, graphics::{Camera2d, Camera3d, Graphics, Rect2d}, inputs::Inputs, object::Object, time::TimeStep};

const LOOK_SPEED: f32 = 0.1;
const MOVE_SPEED: f32 = 5.0;

pub struct Game {
    time_step: TimeStep,
    camera_3d: Camera3d,
    camera_2d: Camera2d,
    cube: Object,
    plane: Object,
    pitch: f32,
    yaw: f32,
}

impl Default for Game {
    fn default() -> Self {
        let camera_3d = Camera3d::new();
        let camera_2d = Camera2d::new();
        Self { 
            time_step: Default::default(),
            camera_3d,
            camera_2d,
            cube: Object::new_cube(Color::red()),
            plane: Object::new_plane(Color::white()),
            yaw: PI / 2.0,
            pitch: 0.0,
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Default::default()
    }
}

impl System for Game {
    fn init(&mut self) {
        // Uncomment the following lines if you want to hide the mouse cursor
        // miniquad::window::show_mouse(false);
        miniquad::window::set_cursor_grab(true);

        self.plane
            .translate(vec3(0.0, -1.0, 0.0))
            .scale(vec3(10.0, 10.0, 10.0));
    }

    fn update(&mut self, inputs: &Inputs) {
        let dt = self.time_step.tick();
        self.cube
            .rotate_z((PI / 4.0) * dt)
            .rotate_y((PI / 4.0) * dt);

        let dt = self.time_step.delta_time();
        self.camera_3d.translate(
            if inputs.key(miniquad::KeyCode::W) {
                vec3(0.0, 0.0, -1.0)
            } else if inputs.key(miniquad::KeyCode::S) {
                vec3(0.0, 0.0, 1.0)
            } else if inputs.key(miniquad::KeyCode::D) {
                vec3(1.0, 0.0, 0.0)
            } else if inputs.key(miniquad::KeyCode::A) {
                vec3(-1.0, 0.0, 0.0)
            } else {
                vec3(0.0, 0.0, 0.0)
            }
            * dt
            * MOVE_SPEED,
        );

        if inputs.key(miniquad::KeyCode::Q) {
            self.camera_3d.rotate_y((PI / 2.0) * dt);
        }

        if inputs.key(miniquad::KeyCode::E) {
            self.camera_3d.rotate_y((-PI / 2.0) * dt);
        }

        self.yaw -= inputs.mouse_delta().x * LOOK_SPEED * dt;
        self.pitch += inputs.mouse_delta().y * LOOK_SPEED * dt;

        self.pitch = self.pitch.clamp(-1.5, 1.5);

        let front = vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize();

        let right = front.cross(vec3(0.0, 1.0, 0.0)).normalize();
        let up = right.cross(front).normalize();

        let new_mat = Mat4::from_cols(
            vec4(right.x, right.y, right.z, 0.0),
            vec4(up.x, up.y, up.z, 0.0),
            vec4(front.x, front.y, front.z, 0.0),
            vec4(self.camera_3d.position().x, self.camera_3d.position().y, self.camera_3d.position().z, 1.0),
        );

        self.camera_3d.set_transform(&new_mat);
    }

    fn draw(&self, g: &mut Graphics) {
        g.set_camera(&self.camera_3d);

        for x in -5..=5 {
            g.draw_line(
                vec3(x as f32, -1.0, -5.0),
                vec3(x as f32, -1.0, 5.0),
                Color::white(),
            );
        }
        for z in -5..=5 {
            g.draw_line(
                vec3(-5.0, -1.0, z as f32),
                vec3(5.0, -1.0, z as f32),
                Color::white(),
            );
        }

        g.draw_object(&self.cube)
            .draw_rectangle(vec2(-1.0, -1.0), vec2(2.0, 2.0), Color::blue())
            .draw_line(vec3(-1.0, -1.0, 0.0), vec3(1.0, 1.0, 0.0), Color::green())
            .set_camera(&self.camera_2d)
            .draw_line(vec3(0.0, 0.0, 0.0), vec3(320.0, 200.0, 0.0), Color::green())
            .draw_rectangle(vec2(10.0, 10.0), vec2(100.0, 50.0), Color::gray());
    }

    fn key_down(&mut self, keycode: miniquad::KeyCode, _keymods: miniquad::KeyMods, _repeat: bool) {
        // Handle key down events if needed
    }
}
