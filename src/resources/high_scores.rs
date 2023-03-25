use {
  bevy::prelude::*,
  crate::events::game_over::GameOver,
};

#[derive(Resource)]
pub struct HighScores {
  pub scores: Vec<(String, u32)>
}

impl Default for HighScores {
  fn default() -> Self {
    Self { scores: vec![] }
  }
}

pub fn update_high_scores(
  mut game_over_event_reader: EventReader<GameOver>,
  mut high_scores: ResMut<HighScores>,
) {
  for game_over in game_over_event_reader.iter() {
    high_scores.scores.push(("Player".to_string(), game_over.score));
    high_scores.scores.sort_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", high_scores.scores)
  }
}