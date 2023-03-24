use {
  bevy::{prelude::*, window::PrimaryWindow },
  rand::prelude::*,
};

pub const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0;

#[derive(Component)]
pub struct Enemy {
  direction: Vec2,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(enemy_movement)
      .add_system(update_enemy_direction)
      .add_system(confine_enemy_movement);
  }
}

pub struct EnemyStartUp;

impl Plugin for EnemyStartUp {
  fn build(&self, app: &mut App) {
    app.add_startup_system(spawn_enemies);
  }
}

fn spawn_enemies(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
) {
  let window = window_query.get_single().unwrap();

  for _ in 0..NUMBER_OF_ENEMIES {
    let random_x = random::<f32>() * window.width();
    let random_y = random::<f32>() * window.height();

    commands.spawn(
      (
        SpriteBundle {
          transform: Transform::from_xyz(random_x, random_y, 0.0),
          texture: asset_server.load("sprites/ball_red_large.png"),
          ..default()
        },
        Enemy {
          direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        },
      )
    );
  }
}

fn enemy_movement(
  mut enemy_query: Query<(&mut Transform, &Enemy)>,
  time: Res<Time>
) {
  for (mut enemy_transform, enemy) in enemy_query.iter_mut() {
    let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);

    enemy_transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
  }
}

fn update_enemy_direction(
  mut enemy_query: Query<(&Transform, &mut Enemy)>,
  window_query: Query<&Window, With<PrimaryWindow>>,
  audio: Res<Audio>,
  asset_server: Res<AssetServer>,
) {
  let window = window_query.get_single().unwrap();

  let half_enemy_size = ENEMY_SIZE / 2.0;
  let min = 0.0 + half_enemy_size;
  let max_x = window.width() - half_enemy_size;
  let max_y = window.height() - half_enemy_size;
  for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
    let mut direction_changed = false;
    let translation = enemy_transform.translation;

    if translation.x < min || translation.x > max_x {
      direction_changed = true;
      enemy.direction.x *= -1.0;
    }
    if translation.y < min || translation.y > max_y {
      direction_changed = true;
      enemy.direction.y *= -1.0;
    }

    // Play SFX
    if direction_changed {
      let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
      let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
  
      let sound_effect = if random::<f32>() > 0.5 {
        sound_effect_1
      } else {
        sound_effect_2
      };
  
      audio.play(sound_effect);
    }
  }
}

fn confine_enemy_movement(
  mut enemy_query: Query<&mut Transform, With<Enemy>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  let window = window_query.get_single().unwrap();

  let half_enemy_size = ENEMY_SIZE / 2.0;
  let min = 0.0 + half_enemy_size;
  let max_x = window.width() - half_enemy_size;
  let max_y = window.height() - half_enemy_size;

  for mut transform in enemy_query.iter_mut() {
    let mut translation = transform.translation;

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

    transform.translation = translation;
  }
}
