
use ggez::timer;
use ggez::nalgebra as na;

use crate::state::{PLAYER_SIZE, WORLD_SIZE, FRICTION};

type Vector2 = na::Vector2<f32>;

pub struct Player {
    pub jump_counter: u32,
    pub jump_timer: u32,
    pub position: Vector2,
    pub velocity: Vector2,
    pub shoot_timer: u32,
    pub charging_time: u32,
    pub arm_offset: f32,
    pub arm_direction: Vector2,
    pub facing_direction: Vector2,
    pub is_aiming: bool
}
impl Player {
    pub fn new() -> Player {
        Player {
            jump_counter: 100,
            jump_timer: 0,
            position: Vector2::new(20., 20.),
            velocity: Vector2::new(0., 0.),
            shoot_timer: 0,
            charging_time: 0,
            arm_offset: 8.,
            arm_direction: Vector2::new(1., 0.),
            facing_direction: Vector2::new(1., 0.),
            is_aiming: false
        }
    }
    pub fn aim_in_direction(&mut self, direction: Vector2) {
        self.facing_direction.x = direction.x.signum();
        self.arm_direction = {
            let new_angle = direction.y.atan2(direction.x);
            Vector2::new(new_angle.cos(), new_angle.sin())
        };
    }
    pub fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let dt = timer::duration_to_f64(timer::delta(ctx)) as f32;
        let mut friction_multiplier = 1.;
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
            if self.is_aiming {
                friction_multiplier = 10.;
            }
        }
        if new_pos.y + PLAYER_SIZE.1 < WORLD_SIZE.1 {
            new_vel.y += 200.*dt;
        }
        new_vel += (new_pos - self.position)*dt;
        new_vel -= FRICTION*self.velocity*dt*friction_multiplier;
        self.position = new_pos;
        self.velocity = new_vel;


        if self.shoot_timer > 0 {
            self.shoot_timer -= 1;
        }
        if self.jump_timer > 0 {
            self.jump_timer -= 1;
        }
        Ok(())
    }
}
