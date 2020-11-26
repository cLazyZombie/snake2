use bevy::prelude::*;

#[derive(Default)]
pub struct Materials {
    pub head_material: Handle<ColorMaterial>,
}

pub fn init_materials(mut mat_assets: ResMut<Materials>, mut color_mat_res: ResMut<Assets<ColorMaterial>>) {
    let head_mat = color_mat_res.add(ColorMaterial::color(Color::rgb(0.2, 0.3, 0.7)));
    mat_assets.head_material = head_mat;
}