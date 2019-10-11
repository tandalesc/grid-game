
use ggez::timer;
use ggez::nalgebra as na;

use crate::state::{PLAYER_SIZE, WORLD_SIZE, FRICTION};

type Vector2 = na::Vector2<f32>;

pub struct Player {
    pub jump_counter: u32,
    pub jump_timer: u32,
    pub position: Vector2,
    pub velocity: Vector2
}
impl Player {
    pub fn new() -> Player {
        Player {
            jump_counter: 100,
            jump_timer: 0,
            position: Vector2::new(20., 20.),
            velocity: Vector2::new(0., 0.)
        }
    }
    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let dt = timer::duration_to_f64(timer::delta(ctx)) as f32;
        let mut new_pos = self.position + self.velocity*dt;
        let mut new_vel = self.velocity.clone();
        //constrain position on edge of world
        if new_pos.x < 0. || new_pos.x + PLAYER_SIZE.0 > WORLD_SIZE.0 {
            new_pos.x = new_pos.x.max(0.).min(WORLD_SIZE.0 - PLAYER_SIZE.0);
            new_vel.x = 0.
        }
        if new_pos.y < 0. || new_pos.y + PLAYER_SIZE.1 > WORLD_SIZE.1 {
            new_pos.y = new_pos.y.max(0.).min(WORLD_SIZE.1 - PLAYER_SIZE.1);
            new_vel.y = 0.;
            self.jump_counter = 0;
        }
        if new_pos.y + PLAYER_SIZE.1 < WORLD_SIZE.1 {
            new_vel.y += 200.*dt;
        }
        new_vel += (new_pos - self.position)*dt;
        new_vel -= FRICTION*self.velocity*dt;
        self.position = new_pos;
        self.velocity = new_vel;

        if self.jump_timer > 0 {
            self.jump_timer -= 1;
        }
        Ok(())
    }
}
