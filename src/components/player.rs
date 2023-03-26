use bevy::{prelude::*, window::PrimaryWindow };

const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.configure_set(PlayerSystemSet::Movement.before(PlayerSystemSet::Confinement))
      .add_startup_system(spawn_player)
      .add_system(player_movement.in_set(PlayerSystemSet::Movement))
      .add_system(confine_player_movement.in_set(PlayerSystemSet::Confinement));
  }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum PlayerSystemSet {
  Movement,
  Confinement,
}

fn spawn_player(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window = window_query.get_single().unwrap();

  commands.spawn(
    (
      SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        texture: asset_server.load("sprites/ball_blue_large.png"),
        ..default()
      },
      Player {}
    )
  );
}

fn player_movement(
  keyboard_input: Res<Input<KeyCode>>,
  mut player_query: Query<&mut Transform, With<Player>>,
  time: Res<Time>
) {
  if let Ok(mut transform) = player_query.get_single_mut() {
    let mut direction = Vec3::ZERO;

    if keyboard_input.any_pressed([KeyCode::Up, KeyCode::W]) {
      direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if keyboard_input.any_pressed([KeyCode::Down, KeyCode::S]) {
      direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if keyboard_input.any_pressed([KeyCode::Left, KeyCode::A]) {
      direction += Vec3::new(-1.0, 0.0, 0.0);
    }

    if keyboard_input.any_pressed([KeyCode::Right, KeyCode::D]) {
      direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if direction.length() > 0.0 {
      direction = direction.normalize();
    }

    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
  }
}

fn confine_player_movement(
  mut player_query: Query<&mut Transform, With<Player>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(mut player_transform) = player_query.get_single_mut() {
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SIZE / 2.0;
    let min = 0.0 + half_player_size;
    let max_x = window.width() - half_player_size;
    let max_y = window.height() - half_player_size;

    let mut translation = player_transform.translation;

    // Bound player x position
    if translation.x < min {
      translation.x = min;
    } else if translation.x > max_x  {
      translation.x = max_x;
    }

    if translation.y < min {
      translation.y = min;
    } else if translation.y > max_y {
      translation.y = max_y;
    }

    player_transform.translation = translation;
  }
}
