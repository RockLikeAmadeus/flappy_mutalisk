use bevy::{app::AppExit, prelude::*, window::WindowResolution};
use my_library::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Mutalisk".into(),
                resolution: WindowResolution::new(1024, 768).with_scale_factor_override(6.0), //(1024, 768).into(),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(RandomPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut rng: ResMut<RandomNumberGenerator>,
) {
    let player_sprite_sheet = asset_server.load("Dream-Catcher-enemies.png");
    let player_sprite_layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 10, 21, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(player_sprite_layout);
    // commands.insert_resource()
    // let assets = Assets {
    //     muta:
    //     wall:
    // }
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_atlas_image(player_sprite_sheet, TextureAtlas {
            layout: texture_atlas_layout,
            index: 10,
        }),
        // Transform::from_scale(Vec3::splat(6.0)),
        Transform::from_xyz(-55.0, 0.0, 1.0),
    ));
}