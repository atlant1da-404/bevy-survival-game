use bevy::prelude::*;


const WIDTH: f32 = 900.;
const HEIGHT: f32 = 600.;
const NAME: &str = "bevy-survival-game";



fn main() {

    App::new()
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: NAME.to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins).run();

}
