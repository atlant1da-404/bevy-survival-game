use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

const ASSETS: &str = "placeholder.png";

pub fn load_graphics(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_assets: ResMut<Assets<TextureAtlas>>
) {

    let image_dir = assets.load(&ASSETS.to_string());
    let atlas = TextureAtlas::new_empty(
        image_dir,
        Vec2::splat(256.0));

    let atlas_handle = texture_assets.add(atlas);

    commands.insert_resource(
         atlas_handle
    );
}

pub fn spawn_camera(
    mut commands: Commands
) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.left = -1. * 16. / 9.;
    camera.orthographic_projection.right = 1. * 16. / 9.;
    camera.orthographic_projection.top = -1.;
    camera.orthographic_projection.bottom = -1.;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}