use bevy::{
    asset::AssetLoader,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use iyes_loopless::prelude::*;

use crate::{despawn_with, GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Running, setup_game)
            .add_exit_system(GameState::Running, despawn_with::<Running>)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Running)
                    .with_system(spawn_blocks)
                    .into(),
            );
    }
}

#[derive(Component)]
struct Running;

struct Size {
    width: usize,
    height: usize,
}

fn setup_game(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let grid_size = Size {
        width: 32,
        height: 64,
    };
    let block_size = 8.;

    let grid_image = Image::new_fill(
        Extent3d {
            width: grid_size.width as u32,
            height: grid_size.height as u32,
            ..default()
        },
        TextureDimension::D2,
        &[128, 128, 128, 128, 128, 128, 128, 128],
        TextureFormat::Rgba16Float,
    );

    let grid_image_handle = images.add(grid_image);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    grid_size.width as f32 * block_size,
                    grid_size.height as f32 * block_size,
                )),
                ..default()
            },
            texture: grid_image_handle,
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        GameGrid::from_size(grid_size),
        Running,
    ));
}

const RED: i8 = 1;

#[derive(Component, Default)]
struct Block {
    width: i8,
    height: i8,
    shape: Vec<i8>,
    x: usize,
    y: usize,
}

impl Block {
    fn random(_seed: i32, grid_width: usize) -> Block {
        Block {
            width: 4,
            height: 1,
            shape: vec![RED, RED, RED, RED],
            x: grid_width / 2,
            y: 0,
        }
    }
}

#[derive(Component, Debug)]
struct Controlled;

#[derive(Component)]
struct GameGrid {
    width: usize,
    height: usize,
    grid: Vec<i8>,
}

impl GameGrid {
    fn from_size(size: Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
            grid: vec![0; size.width * size.height],
        }
    }
}

fn spawn_blocks(
    mut commands: Commands,
    query: Query<&Controlled, With<Block>>,
    grid_query: Query<&GameGrid>,
) {
    if query.is_empty() {
        let grid = grid_query.get_single().unwrap();
        println!("Spawning block");
        commands.spawn((Block::random(42, grid.width), Running, Controlled));
    }
}
