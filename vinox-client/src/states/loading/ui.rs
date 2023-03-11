use bevy::{asset::LoadState, prelude::*};
use vinox_common::{storage::blocks::load::load_all_blocks, world::chunks::storage::BlockTable};

use crate::states::{assets::load::LoadableAssets, components::GameState};

#[derive(Resource, Default)]
pub struct AssetsLoading(pub Vec<HandleUntyped>);

#[allow(clippy::too_many_arguments)]
pub fn switch(
    mut commands: Commands,
    loading: Res<AssetsLoading>,
    asset_server: Res<AssetServer>,
    mut loadable_assets: ResMut<LoadableAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    match asset_server.get_group_load_state(loading.0.iter().map(|h| h.id())) {
        LoadState::Failed => {
            commands.insert_resource(NextState(Some(GameState::Menu)));
        }
        LoadState::Loaded => {
            let mut texture_atlas_builder = TextureAtlasBuilder::default();
            for handle in loadable_assets.block_textures.values() {
                for item in handle {
                    let Some(texture) = textures.get(item) else {
            warn!("{:?} did not resolve to an `Image` asset.", asset_server.get_handle_path(item));
            continue;
                    };

                    texture_atlas_builder.add_texture(item.clone(), texture);
                }
            }
            let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
            let atlas_handle = texture_atlases.add(texture_atlas);
            loadable_assets.block_atlas = atlas_handle;
            commands.insert_resource(NextState(Some(GameState::Game)));
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}

// pub fn timeout(mut commands: Commands, mut timer: Local<Timer>, time: Res<Time>) {
//     timer.set_mode(TimerMode::Repeating);
//     timer.set_duration(Duration::from_secs_f32(5.));

//     timer.tick(time.delta());
//     if timer.just_finished() {
//         commands.insert_resource(NextState(GameState::Menu));
//     }
// }

pub fn setup_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    mut block_table: ResMut<BlockTable>,
) {
    for block in load_all_blocks() {
        let mut name = block.clone().namespace;
        name.push(':');
        name.push_str(&block.name);
        block_table.0.insert(name, block);
    }
}

pub fn load_blocks(
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
    block_table: Res<BlockTable>,
    mut loadable_assets: ResMut<LoadableAssets>,
    mut has_ran: Local<bool>,
) {
    if !(*has_ran) && block_table.is_changed() {
        for block_pair in &block_table.0 {
            let block = block_pair.1;
            let mut texture_array: Vec<Handle<Image>> = Vec::with_capacity(6);
            texture_array.resize(6, Handle::default());
            let mut block_identifier = block.namespace.to_owned();
            block_identifier.push(':');
            block_identifier.push_str(&block.name.to_owned());
            // If there is a front texture preset all faces to use it so someone can use the same texture for all just by providing the front
            if let Some(texture_path) = &block.textures {
                if let Some(front) = texture_path.get(&Some("front".to_string())) {
                    let mut path = "blocks/".to_string();
                    path.push_str(block.name.as_str());
                    path.push('/');
                    path.push_str(front.as_ref().unwrap());
                    let texture_handle: Handle<Image> = asset_server.load(path.as_str());
                    loading.0.push(texture_handle.clone_untyped());
                    texture_array[0] = texture_handle.clone();
                    texture_array[1] = texture_handle.clone();
                    texture_array[2] = texture_handle.clone();
                    texture_array[3] = texture_handle.clone();
                    texture_array[4] = texture_handle.clone();
                    texture_array[5] = texture_handle.clone();
                }
            }
            for texture_path_and_type in block.textures.iter() {
                for texture_path_and_type in texture_path_and_type.iter() {
                    if texture_path_and_type.1.is_some() && texture_path_and_type.0.is_some() {
                        let mut path = "blocks/".to_string();
                        path.push_str(block.name.as_str());
                        path.push('/');
                        path.push_str(texture_path_and_type.1.as_ref().unwrap());
                        let texture_handle: Handle<Image> = asset_server.load(path.as_str());
                        loading.0.push(texture_handle.clone_untyped());
                        match texture_path_and_type.0.as_ref().unwrap().as_str() {
                            "up" => {
                                texture_array[0] = texture_handle;
                            }
                            "down" => {
                                texture_array[1] = texture_handle;
                            }
                            "left" => {
                                texture_array[2] = texture_handle;
                            }
                            "right" => {
                                texture_array[3] = texture_handle;
                            }
                            "front" => {
                                texture_array[4] = texture_handle;
                            }
                            "back" => {
                                texture_array[5] = texture_handle;
                            }
                            _ => {}
                        }
                    }
                }
            }
            let texture_array: [Handle<Image>; 6] =
                texture_array
                    .try_into()
                    .unwrap_or_else(|texture_array: Vec<Handle<Image>>| {
                        panic!(
                            "Expected a Vec of length {} but it was {}",
                            6,
                            texture_array.len()
                        )
                    });
            loadable_assets
                .block_textures
                .insert(block_identifier, texture_array);
        }
        *has_ran = true;
    }
}
