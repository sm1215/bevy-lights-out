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
const CELL_OFF_COLOR: Color = Color::rgba(0.3, 0.3, 1.0, 0.8);
const CELL_ON_COLOR: Color = Color::rgba(1.0, 0.3, 0.3, 0.8);
const CELL_OFF_HOVER_COLOR: Color = Color::rgba(0.3, 0.3, 1.0, 1.0);
const CELL_ON_HOVER_COLOR: Color = Color::rgba(1.0, 0.3, 0.3, 1.0);
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
            .add_system(cell_system)
            ;
    }
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

/// The hover bool controls a highlighting color around the currently hovered cell. 
/// Used for UI navigation
/// The state bool captures the current state of the cell. Changing the state of one cell
/// affects the state of neighboring cells.
#[derive(Debug, Component)]
pub struct Cell {
    hover: bool,
    state: bool,
}

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
                .insert(Cell { state: false, hover: false })
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
                });
        }
    }
}

fn mouse_system(
    mouse: Res<MousePosWorld>,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut hover_query: Query<(&mut Cell, &Transform)>,
) {
    // mouse sample: [-261.7417, -182.0289, 999.9]
    let mouse_translation = Vec3::new(mouse[0], mouse[1], mouse[2]);
    let mouse_scale = Vec2::new(1., 1.);

    let mut current_cell = None;
    // translation sample: Vec3(208.0, -208.0, 0.0)
    for (mut cell, transform) in hover_query.iter() {
        let hover = collide(
            mouse_translation,
            mouse_scale,
            transform.translation,
            transform.scale.truncate(),
        );
        if let Some(hover) = hover {
            cell.hover = true;
            current_cell = Some(cell);
        } else {
            cell.hover = false;
        }
    }

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                if let Some(current_cell) = current_cell {
                    current_cell.state = !current_cell.state;
                }
            }
            _ => {}
        }
    }
}

fn cell_system(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cell_query: Query<(&mut Cell, &mut Handle<ColorMaterial>)>,
) {
    for (mut cell, color) in cell_query.iter() {
        let material = materials.get_mut(color).unwrap();
        material.color = match (cell.state, cell.hover) {
            (true, true) => CELL_ON_HOVER_COLOR,
            (true, false) => CELL_ON_COLOR,
            (false, true) => CELL_OFF_HOVER_COLOR,
            (false, false) => CELL_OFF_COLOR,
        }
    }
}
