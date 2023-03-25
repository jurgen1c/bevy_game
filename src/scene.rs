use {
  bevy::{ prelude::*, window::PrimaryWindow, app::AppExit },
  crate::resources::{
    score::update_score,
    high_scores::update_high_scores
  }
};

pub struct SceneStartUpPlugin;

impl Plugin for SceneStartUpPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(spawn_camera);
  }
}

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
  fn build(&self, app: &mut App) {
    app.add_system(update_score)
      .add_system(update_high_scores)
      .add_system(exit_game);
  }
}

fn exit_game(
  keyboard_input: Res<Input<KeyCode>>,
  mut app_exit_event_writer: EventWriter<AppExit>
) {
  if keyboard_input.just_pressed(KeyCode::Escape) {
    app_exit_event_writer.send(AppExit);
  }
}

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
