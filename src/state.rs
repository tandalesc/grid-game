
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
pub const PLAYER_SIZE: (f32, f32) = (10., 10.);

pub const FRICTION: f32 = 1.;

pub struct MainState {
    player: Player,
    camera_target: Vector2,
    input_events: HashSet<event::KeyCode>
}
impl MainState {
    pub fn new() -> ggez::GameResult<MainState> {
        let ms = MainState {
            player: Player::new(),
            camera_target: Vector2::new(0., 0.),
            input_events: HashSet::new()
        };
        Ok(ms)
    }
    pub fn process_inputs(&mut self) -> ggez::GameResult {
        //use a hashset for consistent input polling
        for key in &self.input_events {
            match key {
                event::KeyCode::Space => {
                    //allow double jumps with some separation
                    if self.player.jump_counter < 2 && self.player.jump_timer == 0 {
                        self.player.velocity.y = -100.0;
                        self.player.jump_timer = 20;
                        self.player.jump_counter += 1;
                    }
                },
                event::KeyCode::Left => {
                    self.player.velocity.x -= 2.0;
                },
                event::KeyCode::Right => {
                    self.player.velocity.x += 2.0;
                },
                _ => { /* do nothing */ }
            }
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
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.process_inputs()?;
        self.update_camera()?;
        self.player.update(ctx)?;
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

        //create player
        mesh_builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.player.position.x, self.player.position.y, PLAYER_SIZE.0, PLAYER_SIZE.1),
            graphics::Color::from_rgba(0, 0, 0, 255)
        );

        //build and draw all meshes
        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, camera_draw_params)?;

        graphics::present(ctx)?;
        Ok(())
    }
    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: event::KeyCode, _keymods: event::KeyMods, _repeat: bool) {
        self.input_events.insert(keycode);
    }
    fn key_up_event(&mut self, _ctx: &mut ggez::Context, keycode: event::KeyCode, _keymods: event::KeyMods) {
        self.input_events.remove(&keycode);
    }
}

fn interpolate_u8(start: u8, end: u8, fade: f32) -> u8 {
    ((start as f32)*(1.-fade) + (end as f32)*fade) as u8
}
