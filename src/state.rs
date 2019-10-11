
use ggez::timer;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

use std::collections::{HashSet};

use super::{WINDOW_RESOLUTION};
use crate::player::{Player};

type Vector2 = na::Vector2<f32>;
type Point2 = na::Point2<f32>;

const CAMERA_RESOLUTION: (f32, f32) = (160., 120.);
const CAMERA_SCALE: (f32, f32) = (WINDOW_RESOLUTION.0/CAMERA_RESOLUTION.0, WINDOW_RESOLUTION.1/CAMERA_RESOLUTION.1);
const CAMERA_FOLLOW_SPEED: f32 = 0.2;

const BG_TILE_SIZE: (f32, f32) = (20., 20.);
pub const WORLD_SIZE: (f32, f32) = (320., 120.);
pub const PLAYER_SIZE: (f32, f32) = (10., 18.);

pub const FRICTION: f32 = 1.;

struct Bullet {
    center: Vector2,
    half_extents: Vector2,
    velocity: Vector2,
    damage: f32,
    bullet_type: BulletType
}
enum BulletType {
    Player, Enemy
}

pub struct MainState {
    player: Player,
    camera_target: Vector2,
    bullets: Vec<Bullet>,
    input_events: HashSet<event::KeyCode>
}
impl MainState {
    pub fn new() -> ggez::GameResult<MainState> {
        let ms = MainState {
            player: Player::new(),
            camera_target: Vector2::new(0., 0.),
            bullets: Vec::new(),
            input_events: HashSet::new()
        };
        Ok(ms)
    }
    pub fn process_inputs(&mut self) -> ggez::GameResult {
        //use a hashset for consistent input polling
        let mut player_aim_dir = Vector2::new(0., 0.);
        for key in &self.input_events {
            match key {
                event::KeyCode::Z => {
                    //allow double jumps with some separation
                    if self.player.jump_counter < 2 && self.player.jump_timer == 0 {
                        self.player.velocity.y = -150.0;
                        self.player.jump_timer = 20;
                        self.player.jump_counter += 1;
                    }
                },
                event::KeyCode::X => {
                    //charge weapon
                    if self.player.charging_time < 100 {
                        self.player.charging_time += 1;
                    }
                },
                event::KeyCode::Left => {
                    if !self.player.is_aiming {
                        self.player.velocity.x -= 2.0;
                    }
                    player_aim_dir.x -= 1.;
                },
                event::KeyCode::Right => {
                    if !self.player.is_aiming {
                        self.player.velocity.x += 2.0;
                    }
                    player_aim_dir.x += 1.;
                },
                event::KeyCode::Up => {
                    player_aim_dir.y -= 1.;
                }
                event::KeyCode::Down => {
                    player_aim_dir.y += 1.;
                }
                _ => { /* do nothing */ }
            }
        }
        //aim player's gun in the appropriate direction
        if player_aim_dir.norm() > 0. {
            self.player.aim_in_direction(player_aim_dir/player_aim_dir.norm());
        }
        Ok(())
    }
    pub fn update_camera(&mut self) -> ggez::GameResult {
        //always follow player
        let dist = self.player.position - self.camera_target;
        self.camera_target += dist*CAMERA_FOLLOW_SPEED;
        //constrain camera so it doesn't show world borders
        self.camera_target.x = self.camera_target.x
            .max(CAMERA_RESOLUTION.0/2.)
            .min(WORLD_SIZE.0.max(CAMERA_RESOLUTION.0) - CAMERA_RESOLUTION.0/2.);
        self.camera_target.y = self.camera_target.y
            .max(CAMERA_RESOLUTION.1/2.)
            .min(WORLD_SIZE.1.max(CAMERA_RESOLUTION.1) - CAMERA_RESOLUTION.1/2.);
        Ok(())
    }
    pub fn update_bullets(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let dt = timer::duration_to_f64(timer::delta(ctx)) as f32;
        //move all bullets
        for bullet in &mut self.bullets {
            bullet.center += bullet.velocity*dt;
        }
        //remove those that are off-screen
        self.bullets.retain(|bullet| {
            bullet.center.x - bullet.half_extents.x > 0. &&
            bullet.center.y - bullet.half_extents.y > 0. &&
            bullet.center.x + bullet.half_extents.x < WORLD_SIZE.0 &&
            bullet.center.y + bullet.half_extents.y < WORLD_SIZE.1
        });
        Ok(())
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.process_inputs()?;
        self.player.update(ctx)?;
        self.update_bullets(ctx)?;
        self.update_camera()?;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.].into());

        //collect meshes for efficient drawing
        let mut mesh_builder = graphics::MeshBuilder::new();
        //center camera on it's current target
        let camera_draw_params = graphics::DrawParam::default()
            .scale(Vector2::new(CAMERA_SCALE.0, CAMERA_SCALE.1))
            .dest(Point2::new(
                (CAMERA_RESOLUTION.0/2. - self.camera_target.x)*CAMERA_SCALE.0,
                (CAMERA_RESOLUTION.1/2. - self.camera_target.y)*CAMERA_SCALE.1
            ));

        //create background tiles
        let world_tile_size_x = (WORLD_SIZE.0/BG_TILE_SIZE.0).ceil() as u32;
        let world_tile_size_y = (WORLD_SIZE.1/BG_TILE_SIZE.1).ceil() as u32;
        for tx in 0..world_tile_size_x {
            for ty in 0..world_tile_size_y {
                let px = (tx as f32)*BG_TILE_SIZE.0;
                let py = (ty as f32)*BG_TILE_SIZE.1;
                mesh_builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(px, py, BG_TILE_SIZE.0, BG_TILE_SIZE.1),
                    graphics::Color::from_rgba(
                        interpolate_u8(0, 255, px/WORLD_SIZE.0),
                        interpolate_u8(0, 255, (WORLD_SIZE.1-py)/WORLD_SIZE.1),
                        255, 255
                    )
                );
            }
        }

        for bullet in &self.bullets {
            mesh_builder.rectangle(
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    bullet.center.x-bullet.half_extents.x, bullet.center.y-bullet.half_extents.y,
                    bullet.half_extents.x*2., bullet.half_extents.y*2.
                ),
                graphics::Color::from_rgba(0, 0, 0, 255)
            );
        }

        //create player
        mesh_builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.player.position.x, self.player.position.y, PLAYER_SIZE.0, PLAYER_SIZE.1),
            graphics::Color::from_rgba(0, 0, 0, 255)
        );
        //create player gun; change color depending on charge
        let gun_pos = self.player.position + Vector2::new(PLAYER_SIZE.0-5.,PLAYER_SIZE.1-5.)/2. + self.player.arm_direction*self.player.arm_offset;
        let gun_brightness_u8 = interpolate_u8(0, 128, ((self.player.charging_time as f32)/100. + 1.).ln());
        mesh_builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(gun_pos.x, gun_pos.y, 5., 5.),
            graphics::Color::from_rgba(gun_brightness_u8, gun_brightness_u8, gun_brightness_u8, 255)
        );

        //build and draw all meshes
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, camera_draw_params)?;

        graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
        match keycode {
            event::KeyCode::LShift => {
                self.player.is_aiming = true;
            },
            _ => { /* do nothing */ }
        };
        self.input_events.insert(keycode);
    }
    fn key_up_event(&mut self, _ctx: &mut ggez::Context, keycode: event::KeyCode, _keymods: event::KeyMods) {
        match keycode {
            event::KeyCode::LShift => {
                self.player.is_aiming = false;
            },
            event::KeyCode::X => {
                //fire gun with appropriate strength depending on charge
                if self.player.shoot_timer == 0 {
                    self.player.shoot_timer = 10;
                    let bullet_size = interpolate_f32(1., 4., (self.player.charging_time as f32)/100.);
                    self.bullets.push(Bullet {
                        center: self.player.position + Vector2::new(PLAYER_SIZE.0, PLAYER_SIZE.1)/2. + self.player.arm_direction*(self.player.arm_offset + bullet_size),
                        half_extents: Vector2::new(bullet_size, bullet_size),
                        velocity: self.player.arm_direction/self.player.arm_direction.norm()*(160. + self.player.velocity.norm()),
                        damage: 10.*bullet_size,
                        bullet_type: BulletType::Player
                    });
                    self.player.charging_time = 0;
                }
            },
            _ => { /* do nothing */ }
        };
        self.input_events.remove(&keycode);
    }
}

fn interpolate_u8(start: u8, end: u8, fade: f32) -> u8 {
    ((start as f32)*(1.-fade) + (end as f32)*fade) as u8
}
fn interpolate_f32(start: f32, end: f32, fade: f32) -> f32 {
    start*(1.-fade) + end*fade
}
