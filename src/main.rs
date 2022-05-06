use bevy::{
    math::{const_vec2},
    prelude::*,
    winit::WinitSettings,
    input::mouse::{MouseButtonInput},
    input::ElementState,
    sprite::collide_aabb::{collide},
};
use bevy_mouse_tracking_plugin::{MousePosPlugin, MousePosWorld};
use bevy_sprite_material::{MaterialSpritePlugin, Sprite, SpriteBundle};

const WINDOW_WIDTH: f32 = 524.;
const WINDOW_HEIGHT: f32 = 524.;

const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

const GRID_LEFT: f32 = -260.;
const GRID_TOP: f32 = 260.;
const GRID_RIGHT: f32 = 260.;
const GRID_DOWN: f32 = -260.;
const GRID_ROWS: u32 = 5;
const GRID_COLUMNS: u32 = 5;

const CELL_MARGIN: f32 = 4.;
const CELL_OFF_COLOR: Color = Color::rgb(0.3, 0.3, 1.0);
const CELL_HOVER_COLOR: Color = Color::rgb(0.1, 0.1, 0.8);
const CELL_ON_COLOR: Color = Color::rgb(1.0, 0.3, 0.3);
const CELL_SIZE: Vec2 = const_vec2!([100., 100.]);

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Lights Out".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MousePosPlugin::SingleCamera)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(LightsOut)
        .run();
}

pub struct LightsOut;
impl Plugin for LightsOut {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(BACKGROUND_COLOR))
            .add_plugin(MaterialSpritePlugin)
            .add_startup_system(setup)
            .add_system(mouse_system)
            .add_system(hover_system)
            ;
    }
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Cell;

#[derive(Component)]
struct Hover;

#[derive(Default)]
struct HoverEvent;

#[derive(Debug, Clone)]
struct MaterialResource {
    material: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // cameras
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    // disabled for now because of conflicting multiple cameras with mouse tracking plugin
    // commands.spawn_bundle(UiCameraBundle::default());

    let cell_x_offset = GRID_LEFT + (CELL_MARGIN / 2.) + (CELL_SIZE.x / 2.);
    let cell_y_offset = GRID_TOP - (CELL_MARGIN / 2.) - (CELL_SIZE.y / 2.);

    // gameplay cells
    for row in 0..GRID_ROWS {
        for column in 0..GRID_COLUMNS {
            let cell_position = Vec2::new (
                cell_x_offset + (CELL_SIZE.x + CELL_MARGIN) * row as f32,
                cell_y_offset - (CELL_SIZE.y + CELL_MARGIN) * column as f32,
            );

            commands
                .spawn()
                .insert(Cell)
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        ..default()
                    },
                    material: materials.add(Color::BLUE.into()),
                    transform: Transform {
                        translation: cell_position.extend(0.0),
                        scale: Vec3::new(CELL_SIZE.x, CELL_SIZE.y, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Hover);
        }
    }
}

fn mouse_system(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    // TODO: track clicked state of cell
    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                println!("click {:?}", ev);
            }
            ElementState::Released => {
                println!("release {:?}", ev);
            }
        }
    }
}

fn hover_system(
    mouse: Res<MousePosWorld>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    hover_query: Query<(Entity, &Transform, &mut Handle<ColorMaterial>), With<Hover>>,
) {
    // mouse sample: [-261.7417, -182.0289, 999.9]
    let mouse_translation = Vec3::new(mouse[0], mouse[1], mouse[2]);
    let mouse_scale = Vec2::new(1., 1.);

    // translation sample: Vec3(208.0, -208.0, 0.0)
    for (entity, transform, color) in hover_query.iter() {
        let hover = collide(
            mouse_translation,
            mouse_scale,
            transform.translation,
            transform.scale.truncate(),
        );
        let material = materials.get_mut(color).unwrap();
        if let Some(hover) = hover {
            material.color = Color::RED.into();
        } else {
            material.color = Color::BLUE.into();
        }

    }
}
