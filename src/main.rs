mod components;
mod scene;

use {
    bevy::prelude::*,
    components::{
        player::{ PlayerPlugin, PlayerStartUp },
        enemy::{ EnemyPlugin, EnemyStartUp }
    },
    scene::{ SceneStartUpPlugin, ScenePlugin },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SceneStartUpPlugin)
        .add_plugin(PlayerStartUp)
        .add_plugin(EnemyStartUp)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ScenePlugin)
        .run();
}
