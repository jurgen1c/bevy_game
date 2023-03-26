use {
  bevy::{ prelude::*, window::PrimaryWindow},
  rand::prelude::*,
  crate::components::player::{ Player, PLAYER_SIZE },
  crate::resources::score::Score,
};

const NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0;
const STAR_SPAWN_TIME: f32 = 2.0;

#[derive(Component)]
pub struct Star;

pub struct StarStartUp;

impl Plugin for StarStartUp {
  fn build(&self, app: &mut App) {
    app.add_startup_system(spawn_stars);
  }
}

pub struct StarPlugin;

impl Plugin for StarPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(player_hit_star)
      .add_system(tick_star_spawn_timer)
      .add_system(spawn_stars_over_time);
  }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
  pub timer: Timer
}

impl Default for StarSpawnTimer {
  fn default() -> Self {
    Self {
      timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
    }
  }
}


fn spawn_stars(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>
) {
  let window = window_query.get_single().unwrap();
  for _ in 0..NUMBER_OF_STARS {
    let random_x = random::<f32>() * window.width() - 10.0;
    let random_y = random::<f32>() * window.height() - 10.0;
    commands.spawn(
      (
        SpriteBundle {
          transform: Transform::from_xyz(random_x, random_y, -1.0),
          texture: asset_server.load("sprites/star.png"),
          ..default()
        },
        Star {},
      )
    );
  };
}

fn player_hit_star(
  mut commands: Commands,
  star_query: Query<(Entity, &Transform), With<Star>>,
  player_query: Query<&Transform, With<Player>>,
  audio: Res<Audio>,
  asset_server: Res<AssetServer>,
  mut score: ResMut<Score>
) {

  if let Ok(player_transform) = player_query.get_single() {
    for (star, star_transform) in star_query.iter() {
      let distance = player_transform
        .translation
        .distance(star_transform.translation);
      if distance < ((PLAYER_SIZE + STAR_SIZE) / 2.0) {
        score.value += 1;
        let sound_effect = asset_server.load("audio/impactPunch_medium_004.ogg");
        audio.play(sound_effect);
        commands.entity(star).despawn();
      }
    }
  }
}

fn tick_star_spawn_timer(
  mut star_spawn_timer: ResMut<StarSpawnTimer>,
  time: Res<Time>,
) {
  star_spawn_timer.timer.tick(time.delta());
}

fn spawn_stars_over_time(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
  asset_server: Res<AssetServer>,
  star_spawn_timer: Res<StarSpawnTimer>,
) {
  if star_spawn_timer.timer.finished() {
    let window = window_query.get_single().unwrap();

    let random_x = random::<f32>() * window.width() - 10.0;
    let random_y = random::<f32>() * window.height() - 10.0;

    commands.spawn(
      (
        SpriteBundle {
          transform: Transform::from_xyz(random_x, random_y, -1.0),
          texture: asset_server.load("sprites/star.png"),
          ..default()
        },
        Star {},
      )
    );
  }
}