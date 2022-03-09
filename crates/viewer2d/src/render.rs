use crate::component::Component;
use fizz::base::{Grid, IntoVec, Range};
use fizz::math::*;
use macroquad::prelude::*;
use tracing::trace;

#[derive(Clone, Debug)]
pub struct Context {
    scale: T,
    offset: TV,
}

impl Default for Context {
    fn default() -> Self {
        let scale = screen_height()
            .try_into()
            .expect("floating point cast should not fail");
        let offset = na::Vector2::<f32>::new(0.5 * (screen_width() - screen_height()), 0.).cast();
        Self { scale, offset }
    }
}

impl Context {
    pub fn world_to_screen(&self, x: TV) -> TV {
        x * self.scale + self.offset
    }
}

pub async fn main_loop() {
    let mut frame = 0;

    let cells = IV::from_element(10);
    let min = TV::zeros();
    let max = TV::from_element(1.);
    let grid = Grid::new(cells, Range::new(min, max));
    info!("Created {:?}", grid);

    let mut prev_mouse_position = mouse_position().into_vec().cast();
    let mut ctx = Context::default();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.));

        // Process keys, mouse etc.
        let mouse_position = mouse_position().into_vec().cast();
        if macroquad::input::is_mouse_button_down(MouseButton::Left) {
            let delta = mouse_position - prev_mouse_position;
            ctx.offset += delta;
        }
        prev_mouse_position = mouse_position;

        let mouse_wheel = macroquad::input::mouse_wheel().1 as f64;
        if mouse_wheel.abs() > 0. {
            let fact = 1. + 0.1 * mouse_wheel;
            ctx.scale *= fact;
            let delta = fact * (ctx.offset - mouse_position);
            ctx.offset = mouse_position + delta;
        }

        egui_macroquad::ui(|ui| {
            egui::Window::new("Fizz Viewer2d").show(ui, |ui| {
                ui.label(format!("Frame {frame}"));
                let fps = macroquad::time::get_fps();
                ui.label(format!("FPS {fps}"));

                if ui.button("Click me!").clicked() {
                    trace!("Button clicked!");
                }
            });
        });

        grid.draw(&ctx);

        // Draw things before egui
        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
        frame += 1;
    }
}
