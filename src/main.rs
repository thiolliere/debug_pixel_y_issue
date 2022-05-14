use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 1 })
        .add_startup_system(setup)
        .run();
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Minifantasy_TownsAllTiles.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 64, 36);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Transform {
            translation: Vec3::new(0.0, 230.12491, 0.0),
            scale: Vec3::new(1.0, 0.25, 1.0),
            ..Default::default()
        });

    for y in 0..100 {
        commands.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(66),
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, y as f32 * 8.0, 0.0),
                scale: Vec3::new(100.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}
