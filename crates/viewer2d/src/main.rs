extern crate fizz2d as fizz;
extern crate nalgebra as na;

mod component;
mod inspector;
mod log;
mod render;

pub use inspector::EguiInspector;

#[macroquad::main("Fizz Viewer2d")]
async fn main() {
    log::setup_logging();

    render::main_loop().await;
}
