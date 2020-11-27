use bevy::prelude::*;

use crate::assets;

const WORLD_GRID_WIDTH: i32 = 16;
const WORLD_GRID_HEIGHT: i32 = 16;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        match self {
            Direction::Left => other == Direction::Right,
            Direction::Right => other == Direction::Left,
            Direction::Up => other == Direction::Down,
            Direction::Down => other == Direction::Up,
        }
    }
}

pub struct Snake {
    pub dir: Direction
}

pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn startup(mut commands: Commands, mat: Res<assets::Materials>) {
    commands.spawn(Camera2dComponents::default());

    commands
        .spawn(SpriteComponents {
            material: mat.head_material.clone(),
            sprite: Sprite::new(Vec2::new(40.0, 40.0)),
            ..Default::default()
        })
        .with(Snake{dir: Direction::Up})
        .with(Timer::from_seconds(0.2, true))
        .with(Position { x: 8, y: 8 });
}

pub fn control_snake(input: Res<Input<KeyCode>>, mut query: Query<&mut Snake>) {
    let mut dir = None;

    if input.pressed(KeyCode::Left) {
        dir = Some(Direction::Left);
    } else if input.pressed(KeyCode::Right) {
        dir = Some(Direction::Right);
    } else if input.pressed(KeyCode::Up) {
        dir = Some(Direction::Up);
    } else if input.pressed(KeyCode::Down) {
        dir = Some(Direction::Down);
    }

    if let Some(dir) = dir {
        for mut snake in query.iter_mut() {
            if snake.dir.is_opposite(dir) == false {
                snake.dir = dir;
            }
        }
    }
}

pub fn move_snake(mut q: Query<(&Snake, &mut Position, &Timer)>) {
    for (snake, mut position, timer) in q.iter_mut() {
        if timer.finished == false {
            continue;
        }

        match snake.dir {
            Direction::Left => position.x -= 1,
            Direction::Right => position.x += 1,
            Direction::Up => position.y += 1,
            Direction::Down => position.y -= 1,
        }

        if position.x < 0 {
            position.x = WORLD_GRID_WIDTH - 1;
        } 
        
        if position.x >= WORLD_GRID_WIDTH {
            position.x = 0;
        }

        if position.y < 0 {
            position.y = WORLD_GRID_HEIGHT - 1; 
        }

        if position.y >= WORLD_GRID_HEIGHT {
            position.y = 0;
        }
    }
}

pub fn move_transform(windows: Res<Windows>, mut query: Query<(&Position, &mut Transform)>) {
    if let Some(window) = windows.get_primary() {
        let window_width = window.width() as i32;
        let window_height = window.height() as i32;

        let cell_size_x = window_width / WORLD_GRID_WIDTH;
        let cell_size_y = window_height as i32 / WORLD_GRID_HEIGHT;

        for (pos, mut transform) in query.iter_mut() {
            // pos -> transform
            let x = (pos.x * cell_size_x) - (window_width / 2) + (cell_size_x/2);
            let y = (pos.y * cell_size_y) - (window_height / 2) + (cell_size_y/2);

            *transform.translation.x_mut() = x as f32;
            *transform.translation.y_mut() = y as f32;
        }
    }
}
