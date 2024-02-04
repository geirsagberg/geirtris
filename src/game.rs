use bevy::{
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::BevyDefault,
    },
};
use hex_color::HexColor;

use crate::{despawn_with, GameState};

pub struct GamePlugin;

#[derive(Component)]
struct Player;

fn spawn_player(mut commands: Commands) {
    commands.spawn(Player);
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Running), setup_game)
            .add_systems(OnExit(GameState::Running), despawn_with::<Running>)
            .add_systems(
                Update,
                (progress_timer, move_block, spawn_block, check_for_game_over)
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(PostUpdate, render_blocks);
    }
}

#[derive(Component)]
struct Running;

struct Size {
    width: usize,
    height: usize,
}

fn check_for_game_over(query: Query<&GameGrid>, mut next_state: ResMut<NextState<GameState>>) {
    let grid = query.single();
    let top_row = &grid.data[grid.width * (grid.height / 2 - 1)..grid.width * (grid.height / 2)];
    if top_row.iter().any(|color| color != &HexColor::BLACK) {
        next_state.set(GameState::MainMenu);
    }
}

fn setup_game(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let grid_size = Size {
        width: 10,
        height: 40,
    };
    let block_size = 16.;

    let grid_image = Image::new_fill(
        Extent3d {
            width: grid_size.width as u32,
            height: grid_size.height as u32,
            ..default()
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::bevy_default(),
    );

    let grid_image_handle = images.add(grid_image);

    commands.spawn((
        Running,
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    grid_size.width as f32 * block_size,
                    grid_size.height as f32 * block_size,
                )),
                ..default()
            },

            texture: grid_image_handle,
            transform: Transform::from_xyz(0., 160., 0.),
            ..default()
        },
        GameGrid::from_size(grid_size),
        GameTime {
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
    ));
}

fn progress_timer(mut query: Query<&mut GameTime>, time: Res<Time>) {
    for mut game_time in &mut query {
        game_time.timer.tick(time.delta());
    }
}

#[derive(Component)]
struct GameTime {
    timer: Timer,
}

#[derive(Component, Default, Debug)]
struct Block {
    width: usize,
    height: usize,
    shape: Vec<bool>,
    x: usize,
    y: usize,
    color: HexColor,
}

impl Block {
    fn random(_seed: i32, grid_width: usize, grid_height: usize) -> Block {
        Block {
            width: 4,
            height: 1,
            shape: vec![true; 4],
            x: grid_width / 2,
            y: grid_height / 2 - 1,
            color: HexColor::RED,
        }
    }
}

#[derive(Component, Debug)]
struct Controlled;

#[derive(Component, Debug)]
struct GameGrid {
    width: usize,
    height: usize,
    data: Vec<HexColor>,
}

impl GameGrid {
    fn from_size(size: Size) -> Self {
        Self {
            width: size.width,
            height: size.height,
            data: vec![HexColor::BLACK; size.width * size.height],
        }
    }
}

fn spawn_block(
    mut commands: Commands,
    query: Query<&Controlled, With<Block>>,
    grid_query: Query<&GameGrid>,
) {
    if query.is_empty() {
        let grid = grid_query.single();
        commands.spawn((
            Block::random(42, grid.width, grid.height),
            Running,
            Controlled,
        ));
    }
}

fn is_game_over(query: Query<&GameGrid>) -> bool {
    let grid = query.single();
    let top_row = &grid.data[grid.width * (grid.height / 2 - 1)..grid.width * (grid.height / 2)];
    top_row.iter().any(|color| color != &HexColor::BLACK)
}

fn collides_with_grid(block: &Block, game_grid: &GameGrid) -> bool {
    if block.y + block.height > game_grid.height {
        true
    } else {
        for x in 0..block.width {
            for y in 0..block.height {
                let i = y * block.width + x;
                if block.shape[i] {
                    if game_grid.data[block.y * game_grid.width + block.x + x] != HexColor::BLACK {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn add_block_to_grid(block: &Block, grid: &mut GameGrid) {
    add_or_remove_block_in_grid(block, grid, true)
}

fn remove_block_from_grid(block: &Block, grid: &mut GameGrid) {
    add_or_remove_block_in_grid(block, grid, false)
}

fn add_or_remove_block_in_grid(block: &Block, grid: &mut GameGrid, add: bool) {
    for x in 0..block.width {
        for y in 0..block.height {
            let i = y * block.width + x;
            if block.shape[i] {
                grid.data[block.y * grid.width + block.x + i] =
                    if add { block.color } else { HexColor::BLACK };
            }
        }
    }
}

fn move_block(
    mut commands: Commands,
    mut query: Query<(&mut Block, Entity), With<Controlled>>,
    mut grid_query: Query<&mut GameGrid>,
    timer_query: Query<&GameTime>,
) {
    let timer = &timer_query.get_single().unwrap().timer;
    let mut game_grid = &mut grid_query.get_single_mut().unwrap();
    if timer.just_finished() {
        for (mut block, entity) in &mut query {
            remove_block_from_grid(&block, &mut game_grid);
            block.y += 1;

            if collides_with_grid(&block, &game_grid) {
                block.y -= 1;
                commands.entity(entity).despawn();
            }
            add_block_to_grid(&block, &mut game_grid);
        }
    }
}

fn render_blocks(query: Query<(&GameGrid, &Handle<Image>)>, mut images: ResMut<Assets<Image>>) {
    for (grid, image_handle) in &query {
        let image = images.get_mut(image_handle).unwrap();

        for i in 0..(grid.width * grid.height) {
            let color = grid.data[i];

            image.data[i * 4] = color.r;
            image.data[i * 4 + 1] = color.g;
            image.data[i * 4 + 2] = color.b;
        }
    }
}
