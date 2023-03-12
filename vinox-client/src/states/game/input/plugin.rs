use bevy::prelude::*;

use crate::states::components::GameState;

use super::player::{movement_input, spawn_camera, MouseSensitivity};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MouseSensitivity(1.0))
            .add_systems((spawn_camera, movement_input).in_set(OnUpdate(GameState::Game)));
    }
}