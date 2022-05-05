mod graphics;

use bevy::prelude::*;


const WIDTH: f32 = 900.;
const HEIGHT: f32 = 600.;
const NAME: &str = "bevy-survival-game";


fn main() {

    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3, 0.5, 0.3)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: NAME.to_string(),
            ..Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, graphics::load_graphics)
        .add_startup_system(graphics::spawn_camera)
        .add_plugins(DefaultPlugins).run();

}
