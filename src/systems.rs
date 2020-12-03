use crate::assets;
use bevy::prelude::*;

const WORLD_GRID_WIDTH: i32 = 16;
const WORLD_GRID_HEIGHT: i32 = 16;

pub const BODY_UPDATE : &str = "body_update";

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

// TODO
// Snake + SnakeElement 합치기
// pos -> 실제 transform 구하는 함수 코드 중복 삭제

pub struct Snake {
    pub dir: Direction,
}

pub struct Food;

#[derive(Copy, Clone)]
pub struct SnakeElement {
    pub entity: Entity,
    pub pos: Position,
}

pub struct SnakeElements {
    pub elements: Vec<SnakeElement>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct SnakeBody;

fn create_snake_body(commands: &mut Commands, mat: Handle<ColorMaterial>) -> Entity {
    commands
        .spawn(SpriteComponents {
            material: mat,
            sprite: Sprite::new(Vec2::new(40.0, 40.0)),
            ..Default::default()
        })
        .with(SnakeBody)
        .current_entity()
        .unwrap()
}

fn create_food(commands: &mut Commands, mat: Handle<ColorMaterial>, pos: Position) -> Entity {
    commands
        .spawn( SpriteComponents {
            material: mat,
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Food)
        .with(pos)
        .current_entity()
        .unwrap()
}

pub fn startup(mut commands: Commands, mat: Res<assets::Materials>) {
    commands.spawn(Camera2dComponents::default());

    // create body
    let head = create_snake_body(&mut commands, mat.head_material.clone());
    let middle = create_snake_body(&mut commands, mat.body_material.clone());
    let tail = create_snake_body(&mut commands, mat.body_material.clone());
    
    commands
        .spawn((Snake { dir: Direction::Up },))
        .with(SnakeElements {
            elements: vec![
                SnakeElement {
                    entity: head,
                    pos: Position { x: 8, y: 8 },
                },
                SnakeElement {
                    entity: middle,
                    pos: Position { x: 8, y: 7 },
                },
                SnakeElement {
                    entity: tail,
                    pos: Position { x: 8, y: 6 },
                },
            ],
        })
        .with(Timer::from_seconds(0.2, true));

    create_food(&mut commands, mat.food_material.clone(), Position{x: 2, y: 2});
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
    mut commands: Commands,
    mat: Res<assets::Materials>,
    mut q: Query<(&Snake, &mut SnakeElements, &Timer)>,
    food_query: Query<(Entity, &Food, &Position)>,
) {
    for (snake, mut elements, timer) in q.iter_mut() {
        if timer.finished == false {
            continue;
        }

        let mut head_pos = elements.elements[0].pos;

        match snake.dir {
            Direction::Left => head_pos.x = head_pos.x - 1,
            Direction::Right => head_pos.x = head_pos.x + 1,
            Direction::Up => head_pos.y = head_pos.y + 1,
            Direction::Down => head_pos.y = head_pos.y - 1,
        }

        if head_pos.x < 0 {
            head_pos.x = WORLD_GRID_WIDTH - 1;
        }

        if head_pos.x >= WORLD_GRID_WIDTH {
            head_pos.x = 0;
        }

        if head_pos.y < 0 {
            head_pos.y = WORLD_GRID_HEIGHT - 1;
        }

        if head_pos.y >= WORLD_GRID_HEIGHT {
            head_pos.y = 0;
        }

        // 새로운 head 위치에 food 얻어오기
        let mut food_entity: Option<Entity> = None;
        for (entity, _, pos) in food_query.iter() {
            if *pos == head_pos {
                food_entity = Some(entity);
                break;
            }
        }

        let tail_pos = elements.elements.last().unwrap().pos;

        let mut next_positions = Vec::new();
        next_positions.push(head_pos);

        // follow
        elements
            .elements
            .iter()
            .for_each(|elem| next_positions.push(elem.pos));

        // restore
        elements
            .elements
            .iter_mut()
            .enumerate()
            .for_each(|(index, elem)| {
                elem.pos = next_positions[index];
            });

        if let Some(food_entity) = food_entity {
            let tail_entity = create_snake_body(&mut commands, mat.body_material.clone());
            let new_tail = SnakeElement {
                entity: tail_entity,
                pos: tail_pos,
            };
            elements.elements.push(new_tail);

            commands.despawn(food_entity);
        }
    }
}

pub fn move_snake_transform(
    windows: Res<Windows>,
    snake_query: Query<&SnakeElements>,
    mut query: Query<&mut Transform>,
) {
    if let Some(window) = windows.get_primary() {
        let window_width = window.width() as i32;
        let window_height = window.height() as i32;

        let cell_size_x = window_width / WORLD_GRID_WIDTH;
        let cell_size_y = window_height as i32 / WORLD_GRID_HEIGHT;

        for elems in snake_query.iter() {
            for elem in &elems.elements {
                let pos = elem.pos;
                if let Ok(mut transform) = query.get_mut(elem.entity) {
                    let x = (pos.x * cell_size_x) - (window_width / 2) + (cell_size_x / 2);
                    let y = (pos.y * cell_size_y) - (window_height / 2) + (cell_size_y / 2);

                    *transform.translation.x_mut() = x as f32;
                    *transform.translation.y_mut() = y as f32;
                }
            }
        }
    }
}

pub fn move_food_transform(
    windows: Res<Windows>,
    mut query: Query<(&Food, &Position, &mut Transform)>,
) {
    if let Some(window) = windows.get_primary() {
        let window_width = window.width() as i32;
        let window_height = window.height() as i32;

        let cell_size_x = window_width / WORLD_GRID_WIDTH;
        let cell_size_y = window_height as i32 / WORLD_GRID_HEIGHT;

        for (_, pos, mut transform) in query.iter_mut() {
            let x = (pos.x * cell_size_x) - (window_width / 2) + (cell_size_x / 2);
            let y = (pos.y * cell_size_y) - (window_height / 2) + (cell_size_y / 2);

            *transform.translation.x_mut() = x as f32;
            *transform.translation.y_mut() = y as f32;
        }
    }
}