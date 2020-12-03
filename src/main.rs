use bevy::prelude::*;
use snake2::{assets, systems};

fn pre_update() {
    //println!("-----------------------------------------------------------------");
}

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(WindowDescriptor {
            title: "snake2".to_string(),
            width: 800,
            height: 800,
            ..Default::default()
        })
        .add_resource(assets::Materials::default())
        .add_startup_system(assets::init_materials.system())
        .add_startup_system(systems::startup.system())
        .add_system_to_stage(stage::PRE_UPDATE, pre_update.system())
        .add_system(systems::control_snake.system())
        .add_system(systems::move_snake.system())
        .add_plugins(DefaultPlugins)
        //.add_stage_after(stage::UPDATE, "body_update")
        .add_system_to_stage(stage::POST_UPDATE, systems::move_food_transform.system())
        .add_system_to_stage(stage::POST_UPDATE, systems::move_snake_transform.system())
        .run();
}
