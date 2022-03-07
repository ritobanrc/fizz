use crate::render::Context;
use fizz::math::*;
use macroquad::prelude::*;

pub trait Component {
    fn draw(&self, ctx: &Context);
}

impl Component for fizz::base::Grid {
    fn draw(&self, ctx: &Context) {
        for a in 0..DIM {
            for i in 0..=self.cells[a] {
                let node_start = IV::ith(a, i);
                let node_end = node_start + IV::ith(1 - a, self.cells[1 - a]);
                let start_pos = ctx.world_to_screen(self.node_x(node_start)).cast();
                let end_pos = ctx.world_to_screen(self.node_x(node_end)).cast();

                draw_line(start_pos.x, start_pos.y, end_pos.x, end_pos.y, 0.25, WHITE);
            }
        }
    }
}
