use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct Score {
  pub value: u32
}

impl Default for Score {
  fn default() -> Score {
    Score { value: 0 }
  }
}

pub fn update_score(score: Res<Score>) {
  if score.is_changed() {
    println!("Score: {}", score.value);
  }
}