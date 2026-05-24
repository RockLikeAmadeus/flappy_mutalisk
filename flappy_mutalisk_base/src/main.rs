use bevy::{app::AppExit, gizmos, prelude::*, window::WindowResolution};
use my_library::*;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;
const SCALE_FACTOR: f32 = 6.0;

const SPRITE_WIDTH_PIXELS:f32 = 8.0;
const SPRITE_HEIGHT_PIXELS:f32 = 8.0;

const VIRTUAL_WIDTH: f32 = WINDOW_WIDTH as f32 / SCALE_FACTOR;
const VIRTUAL_HEIGHT: f32 = WINDOW_HEIGHT as f32 / SCALE_FACTOR;
const WORLD_TOP: f32 = VIRTUAL_HEIGHT / 2.0;

#[derive(Component)]
struct Player {
    gravity: f32
}

#[derive(Component)]
struct Obstacle;

// #[derive(Resource)]
// struct Assets { // Not currently used, need to understand how we would use this with a sprite sheet
//     muta: Handle<Image>,
//     wall: Handle<Image>,
// }

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Mutalisk".into(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                    .with_scale_factor_override(SCALE_FACTOR),
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugins(RandomPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_debug)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut drawer: Gizmos,
    mut rng: ResMut<RandomNumberGenerator>,
) {
    // Camera
    commands.spawn(Camera2d);


    let player_sprite_sheet = asset_server.load("Dream-Catcher-enemies.png");
    let player_sprite_layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 10, 21, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(player_sprite_layout);
    let player_sprite = 
        Sprite::from_atlas_image(player_sprite_sheet, TextureAtlas {
            layout: texture_atlas_layout,
            index: 10,
        });
    commands.spawn((
        player_sprite,
        Transform::from_xyz(-55.0, 0.0, 1.0),
    ));

    let wall_sprite_sheet = asset_server.load("Dream-Catcher-ground-wall.png");
    let wall_sprite_layout = TextureAtlasLayout::from_grid(UVec2::splat(8), 32, 30, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(wall_sprite_layout);
    let wall_sprite = 
        Sprite::from_atlas_image(wall_sprite_sheet, TextureAtlas {
            layout: texture_atlas_layout,
            index: 439,
        });
 
    build_wall(&mut commands, &wall_sprite, 8);
}

fn draw_debug(mut gizmos: Gizmos) {
    // Draw a grid, so we can help orient ourselves
    gizmos.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(100, 100),
        Vec2::new(SPRITE_WIDTH_PIXELS, SPRITE_HEIGHT_PIXELS),
        LinearRgba::RED.with_alpha(0.1),
    ).outer_edges();
}

fn build_wall(
    commands: &mut Commands,
    wall_sprite: &Sprite,
    gap_y: i32,
) {
    println!("WORLD TOP!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!: {WORLD_TOP}");
    commands.spawn((
        wall_sprite.clone(),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Obstacle,
    ));

    for y in -12..=-12{
        if y < gap_y - 4 || y > gap_y + 4 {
            commands.spawn((
                wall_sprite.clone(),
                Transform::from_xyz(45.0, y as f32 * 8.0, 1.0),
                Obstacle,
            ));
        }
    }
}