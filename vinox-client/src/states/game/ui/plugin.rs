use crate::states::components::GameState;

use super::dropdown::{create_ui, ConsoleOpen};
use bevy::prelude::*;

pub struct UiPlugin;

#[derive(Resource, Default)]
pub struct InUi(pub bool);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ConsoleOpen(false))
            .insert_resource(InUi(false))
            .add_system(create_ui.in_set(OnUpdate(GameState::Game)));
    }
}