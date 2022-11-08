#![deny(clippy::all)]
#![forbid(unsafe_code)]

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use smatrix::user_type::camera::Camera;
use smatrix::user_type::object_buffer::{ObjectBuffer, Triangle};
use smatrix::user_type::position::Pos3;
use smatrix::user_type::matrix::Matrix;
use smatrix::user_type::vector::Vector3;


const WIDTH: u32 = 320;
const HEIGHT: u32 = 240;
const BOX_SIZE: i16 = 64;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    box_x: i16,
    box_y: i16,
    velocity_x: i16,
    velocity_y: i16,
    camera: Camera,
    buffer: ObjectBuffer,
    theta: f32,
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.get_frame_mut());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize_surface(size.width, size.height);
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        let mut _buffer = ObjectBuffer::new();
        _buffer.add_object(Triangle::new(
                Pos3::new(1., 2., -9.5),
                Pos3::new(2., 2.5, -7.5),
                Pos3::new(1.9, -2., -5.5),
                ));

        Self {
            box_x: 24,
            box_y: 16,
            velocity_x: 1,
            velocity_y: 1,
            camera: Camera::new(10., 1., -5., -10.),
            buffer: _buffer,
            theta: 0.,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
            self.velocity_y *= -1;
        }

        self.box_x += self.velocity_x;
        self.box_y += self.velocity_y;
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, frame: &mut [u8]) {
        let p1 = Pos3::new(1., 2., -9.5);
        let p2 = Pos3::new(2., 2.5, -7.5);
        let p3 = Pos3::new(1.9, -2., -5.5);
        let mut buffer = ObjectBuffer::new();
        self.theta += 0.01;
        let _move_origin = Matrix::move_matrix(-2., -2.5, 7.5);
        let _mat = Vector3::new(0., 1., 0.).to_rotation_matrix(self.theta);
        let _move_back = Matrix::move_matrix(2., 2.5, -7.5);
        let _mat = ((&_move_back * &_mat).unwrap() * _move_origin).unwrap();

        let p1 = Pos3::from_matrix(&(&_mat * &p1.to_matrix()).unwrap());
        let p2 = Pos3::from_matrix(&(&_mat * &p2.to_matrix()).unwrap());
        let p3 = Pos3::from_matrix(&(&_mat * &p3.to_matrix()).unwrap());
        println!("theta:{:?}, p:{:?}, {:?}, {:?}", self.theta, p1, p2, p3);
        buffer.add_object(Triangle::new(p1, p2, p3));
        let _buf = self.camera.render(WIDTH, HEIGHT, &buffer);

        frame.copy_from_slice(&_buf.display);
    }
}