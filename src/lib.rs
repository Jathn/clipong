// src/lib.rs
pub mod gameplay;
pub mod components;
pub mod constants;

// Re-export commonly used items

pub use components::ball::Ball;
pub use components::bat::Bat;
pub use gameplay::state::State;
pub use gameplay::action::Action;
pub use constants::{WIDTH, HEIGHT};