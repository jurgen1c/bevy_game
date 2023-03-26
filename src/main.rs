mod components;
mod scene;
mod resources;
mod events;

use {
    bevy::prelude::*,
    components::{
        player::{ PlayerPlugin, PlayerStartUp },
        enemy::{ EnemyPlugin, EnemyStartUp, EnemySpawnTimer },
        star::{ StarStartUp, StarPlugin, StarSpawnTimer },
    },
    scene::{
        SceneStartUpPlugin,
        ScenePlugin
    },
    resources:: {
        high_scores::HighScores,
        score::Score
    },
    events::game_over::GameOver
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_plugin(SceneStartUpPlugin)
        .add_plugin(PlayerStartUp)
        .add_plugin(EnemyStartUp)
        .add_plugin(StarStartUp)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(StarPlugin)
        .add_plugin(ScenePlugin)
        .run();
}
