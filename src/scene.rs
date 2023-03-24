use {
  bevy::{ prelude::*, window::PrimaryWindow },
  crate::components::{player::{ Player, PLAYER_SIZE }, enemy::Enemy }
};

fn spawn_camera(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>
) {
  let window = window_query.get_single().unwrap();

  commands.spawn(
    Camera2dBundle {
      transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
      ..default()
    }
  );
}

pub fn enemy_hit_player(
  mut commands: Commands,
  mut player_query: Query<(Entity, &Transform), With<Player>>,
  enemy_query: Query<&Transform, With<Enemy>>,
  audio: Res<Audio>,
  asset_server: Res<AssetServer>
) {
  if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
    for enemy_tranform in enemy_query.iter() {
      let distance = player_transform
        .translation
        .distance(enemy_tranform.translation);

      if distance < PLAYER_SIZE {
        println!("Enemy hit player!! Game Over!!");
        let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
        audio.play(sound_effect);
        commands.entity(player_entity).despawn();
      }
    }
  }
}

pub struct SceneStartUpPlugin;

impl Plugin for SceneStartUpPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(spawn_camera);
  }
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
  fn build(&self, app: &mut App) {
    app.add_system(enemy_hit_player);
  }
}