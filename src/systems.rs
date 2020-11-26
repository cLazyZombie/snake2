use bevy::{prelude::*};

use crate::assets;

pub struct Snake;

pub fn startup(mut commands: Commands, mat: Res<assets::Materials>) {
    commands.spawn(Camera2dComponents::default());

    commands.spawn(SpriteComponents{
        material: mat.head_material.clone(),
        sprite: Sprite::new(Vec2::new(50.0, 50.0)),
        ..Default::default()
    })
    .with(Snake);
}

pub fn move_snake(input: Res<Input<KeyCode>>, time: Res<Time>, q2: Query<(&Snake, &Handle<Mesh>)>, mut q: Query<(&Snake, &mut Transform)>) {
    let movement = time.delta_seconds * 100.;

    for (_, _) in q2.iter() {
        println!("q2");
    }

    for (_snake, mut transform) in q.iter_mut() {
        println!("move");

        let x = transform.translation.x();
        let y = transform.translation.y();
        
        if input.pressed(KeyCode::Left) {
            *transform.translation.x_mut() = x - movement;
        }
    }
}