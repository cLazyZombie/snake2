use bevy::prelude::*;

use crate::assets;

const WORLD_GRID_WIDTH: i32 = 16;
const WORLD_GRID_HEIGHT: i32 = 16;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
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
    pub dir: Direction,
    pub next: Option<Entity>,
    pub tail: Option<Entity>,
}

pub struct SnakeBody {
    pub head: Entity,
    pub next: Option<Entity>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct SpawnSnakeBodyEvent {
    pub entity: Entity,
    pub pos: Position,
    pub remain_count: i32,
}

#[derive(Debug)]
pub struct MoveSnakeElementEvent {
    pub entity: Entity,
    pub pos: Position,
}

pub fn startup(
    mut commands: Commands, 
    mat: Res<assets::Materials>,
    mut spawn_events: ResMut<Events<SpawnSnakeBodyEvent>>
) {
    commands.spawn(Camera2dComponents::default());

    let snake = commands
        .spawn(SpriteComponents {
            material: mat.head_material.clone(),
            sprite: Sprite::new(Vec2::new(40.0, 40.0)),
            ..Default::default()
        })
        .with(Snake {
            dir: Direction::Up,
            next: None,
            tail: None,
        })
        .with(Timer::from_seconds(0.2, true))
        .with(Position { x: 8, y: 8 })
        .current_entity()
        .unwrap();

    let ev = SpawnSnakeBodyEvent {
        entity: snake,
        pos: Position{ x: 8, y: 8},
        remain_count: 3,
    };
    spawn_events.send(ev);
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

pub fn move_snake(
    mut events: ResMut<Events<MoveSnakeElementEvent>>,
    mut q: Query<(&Snake, &mut Position, &Timer)>,
) {
    for (snake, mut position, timer) in q.iter_mut() {
        if timer.finished == false {
            continue;
        }

        let prev_pos = *position;

        match snake.dir {
            Direction::Left => position.x = position.x - 1,
            Direction::Right => position.x = position.x + 1,
            Direction::Up => position.y = position.y + 1,
            Direction::Down => position.y = position.y - 1,
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

        println!("MoveHead {:?}", position);

        if let Some(next) = snake.next {
            let event = MoveSnakeElementEvent {
                entity: next,
                pos: prev_pos,
            };
    
            events.send(event);
        }
    }
}

pub fn move_transform(
    windows: Res<Windows>,
    mut query: Query<(&Position, &mut Transform)>,
) {
    if let Some(window) = windows.get_primary() {
        let window_width = window.width() as i32;
        let window_height = window.height() as i32;

        let cell_size_x = window_width / WORLD_GRID_WIDTH;
        let cell_size_y = window_height as i32 / WORLD_GRID_HEIGHT;

        for (pos, mut transform) in query.iter_mut() {
            let x = (pos.x * cell_size_x) - (window_width / 2) + (cell_size_x / 2);
            let y = (pos.y * cell_size_y) - (window_height / 2) + (cell_size_y / 2);

            *transform.translation.x_mut() = x as f32;
            *transform.translation.y_mut() = y as f32;
        }
    }
}

// SpawnSnakeBodyEvent
pub fn handle_spawn_snake_body(
    mut commands: Commands,
    mut events: ResMut<Events<SpawnSnakeBodyEvent>>,
    mat: Res<assets::Materials>,
    mut query_snake: Query<&mut Snake>,
    mut query_body: Query<&mut SnakeBody>,
) {
    let mut events_to_send = Vec::new();

    for ev in events.drain() {
        let entity = commands
            .spawn(SpriteComponents {
                material: mat.body_material.clone(),
                sprite: Sprite::new(Vec2::new(40.0, 40.0)),
                ..Default::default()
            })
            .with(SnakeBody {
                next: None,
                head: ev.entity,
            })
            .with(Timer::from_seconds(0.2, true))
            .with(ev.pos)
            .current_entity()
            .unwrap();

        if let Ok(mut snake) = query_snake.get_mut(ev.entity) {
            if snake.next == None {
                snake.next = Some(entity);
            }

            snake.tail = Some(entity);
        }

        if let Ok(mut snake_body) = query_body.get_mut(ev.entity) {
            snake_body.next = Some(entity);
        }

        if ev.remain_count > 1 {
            events_to_send.push(SpawnSnakeBodyEvent {
                entity: entity,
                pos: ev.pos,
                remain_count: ev.remain_count - 1,
            });
        }
    }

    for event in events_to_send {
        events.send(event);
    }
}

pub fn handle_move_snake_element(
    mut events: ResMut<Events<MoveSnakeElementEvent>>,
    mut query: Query<(&SnakeBody, &mut Position)>,
) {
    println!("handle_move_snake_element");

    let mut events_to_send = Vec::new();

    for ev in events.drain() {
        if let Ok((body, mut pos)) = query.get_mut(ev.entity) {
            let prev_pos = *pos;
            *pos = ev.pos;

            println!("MoveBody {:?}", ev);

            // send move event to next body
            if let Some(next) = body.next {
                events_to_send.push(MoveSnakeElementEvent {
                    entity: next,
                    pos: prev_pos,
                });
            }
        }
    }

    for move_event in events_to_send {
        events.send(move_event);
    }
}
