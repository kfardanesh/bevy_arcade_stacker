use bevy::{
    prelude::*,
    render::{
        camera::{OrthographicProjection, WindowOrigin},
        pass::ClearColor,
    },
    window::WindowMode,
};

const TILE_WIDTH: f32 = 50.0;
const WINDOW_WIDTH: u32 = (TILE_WIDTH * 7.0 + 150.0) as u32 + 4;
const WINDOW_HEIGHT: u32 = (TILE_WIDTH * 15.0) as u32 + 4;
const NUM_ROWS: i32 = 15;
const NUM_COLS: i32 = 7;

#[derive(Default)]
struct GameState {
    level: usize,
    spawn_new: bool,
    game_over: bool,
}

struct Score {}

struct Block {
    size: u8,
    position: i32,
    direction: i32,
    active: bool,
}

struct StackerPlugin;

impl Plugin for StackerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_resource(GameState {
                game_over: false,
                spawn_new: true,
                level: 0,
            })
            .add_startup_system(create_grid.system())
            .add_startup_system(create_camera.system())
            .add_startup_system(create_ui.system())
            .add_system(spawn_blocks.system())
            .add_system(block_movement_system.system())
            .add_system(button_system.system())
            .add_system(score_system.system());
    }
}

fn create_grid(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    for x in 0..=NUM_COLS {
        commands.spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            sprite: Sprite {
                size: Vec2::new(4.0, TILE_WIDTH * (NUM_ROWS as f32) + 4.0),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                (x as f32) * TILE_WIDTH + 2.0,
                (TILE_WIDTH * (NUM_ROWS as f32)) / 2.0 + 2.0,
                1.0,
            )),
            ..Default::default()
        });
    }

    for y in -NUM_ROWS..=0 {
        let mut color = Color::rgb(0.02, 0.02, 0.02);
        if y == -NUM_ROWS + 1 || y == -NUM_ROWS || y == -NUM_ROWS + 5 || y == -NUM_ROWS + 4 {
            color = Color::rgb(0.01, 0.01, 0.4);
        }
        commands.spawn(SpriteComponents {
            material: materials.add(color.into()),
            sprite: Sprite {
                size: Vec2::new(TILE_WIDTH * (NUM_COLS as f32) + 4.0, 4.0),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(
                TILE_WIDTH * (NUM_COLS as f32) / 2.0 + 2.0,
                (y as f32) * -TILE_WIDTH + 2.0,
                1.0,
            )),
            ..Default::default()
        });
    }

    commands.spawn(SpriteComponents {
        material: materials.add(Color::rgb(0.01, 0.01, 0.4).into()),
        sprite: Sprite {
            size: Vec2::new(4.0, TILE_WIDTH + 4.0),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.0 * TILE_WIDTH + 2.0,
            TILE_WIDTH * 11.0 + 2.0 - TILE_WIDTH / 2.0,
            1.0,
        )),
        ..Default::default()
    });

    commands.spawn(SpriteComponents {
        material: materials.add(Color::rgb(0.01, 0.01, 0.4).into()),
        sprite: Sprite {
            size: Vec2::new(4.0, TILE_WIDTH + 4.0),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(
            0.0 * TILE_WIDTH + 2.0,
            TILE_WIDTH * 15.0 + 2.0 - TILE_WIDTH / 2.0,
            1.0,
        )),
        ..Default::default()
    });

    commands.spawn(SpriteComponents {
        material: materials.add(Color::rgb(0.01, 0.01, 0.4).into()),
        sprite: Sprite {
            size: Vec2::new(4.0, TILE_WIDTH + 4.0),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(
            7.0 * TILE_WIDTH + 2.0,
            TILE_WIDTH * 11.0 + 2.0 - TILE_WIDTH / 2.0,
            1.0,
        )),
        ..Default::default()
    });

    commands.spawn(SpriteComponents {
        material: materials.add(Color::rgb(0.01, 0.01, 0.4).into()),
        sprite: Sprite {
            size: Vec2::new(4.0, TILE_WIDTH + 4.0),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(
            7.0 * TILE_WIDTH + 2.0,
            TILE_WIDTH * 15.0 + 2.0 - TILE_WIDTH / 2.0,
            1.0,
        )),
        ..Default::default()
    });
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dComponents {
        orthographic_projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..Default::default()
        },
        ..Default::default()
    });
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            hovered: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            pressed: materials.add(Color::rgb(0.1, 0.5, 0.1).into()),
        }
    }
}

fn create_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        // ui camera
        .spawn(UiCameraComponents::default())
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Percent(100.0)),
                // border: Rect::all(Val::Px(2.0)),
                margin: Rect {
                    left: Val::Auto,
                    right: Val::default(),
                    top: Val::default(),
                    bottom: Val::Auto,
                },
                flex_wrap: FlexWrap::Wrap,
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeComponents {
                    material: materials.add(Color::rgb(0.02, 0.02, 0.02).into()),
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(TILE_WIDTH)),
                        margin: Rect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::default(),
                            bottom: Val::Auto,
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextComponents {
                            text: Text {
                                value: "Score:".to_string(),
                                font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                                style: TextStyle {
                                    font_size: 18.0,
                                    color: Color::rgb(0.8, 0.8, 0.8),
                                },
                            },
                            ..Default::default()
                        })
                        .with(Score {});
                })
                .spawn(ButtonComponents {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(TILE_WIDTH)),
                        // // center button
                        margin: Rect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::default(),
                            bottom: Val::Auto,
                        },
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: button_materials.normal,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(TextComponents {
                        text: Text {
                            value: "New Game".to_string(),
                            font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                            style: TextStyle {
                                font_size: 18.0,
                                color: Color::rgb(0.8, 0.8, 0.8),
                            },
                        },
                        ..Default::default()
                    });
                });
        });
}

fn button_system(
    mut commands: Commands,
    button_materials: Res<ButtonMaterials>,
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
    mut block_query: Query<(Entity, &mut Block, &mut Transform, &mut Timer)>,
) {
    for (_button, interaction, mut material, _children) in &mut interaction_query.iter() {
        // let mut text = text_query.get_mut::<Text>(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                // text.value = "Press".to_string();
                *material = button_materials.pressed;
                for (entity, _block, _translation, _timer) in &mut block_query.iter() {
                    commands.despawn(entity);
                }
                game_state.game_over = false;
                game_state.spawn_new = true;
                game_state.level = 0;
            }
            Interaction::Hovered => {
                // text.value = "Hover".to_string();
                *material = button_materials.hovered;
            }
            Interaction::None => {
                // text.value = "New Game".to_string();
                *material = button_materials.normal;
            }
        }
    }
}

fn score_system(game_state: Res<GameState>, mut query: Query<(&Score, &mut Text)>) {
    for (_score, mut text) in &mut query.iter() {
        if game_state.level == 15 && game_state.game_over {
            text.value = format!("Score: {}\nYou Won Big", game_state.level);
        } else if game_state.game_over {
            text.value = "Score: Game Over".to_string();
        } else {
            text.value = format!("Score: {}", game_state.level - 1);
        }
    }
}

fn block_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<GameState>,
    mut block_query: Query<(&mut Block, &mut Transform, &mut Timer)>,
) {
    for (mut block, mut transform, timer) in &mut block_query.iter() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            block.direction = 0;
            block.active = false;
            game_state.spawn_new = true;
            if game_state.level == 15 {
                game_state.game_over = true;
            }
        }

        if block.active && timer.finished && block.direction != 0 {
            let (bound_left, bound_right) = match block.size {
                1 => (0, NUM_COLS - 1),
                2 => (0, NUM_COLS - 2),
                3 => (0, NUM_COLS - 3),
                _ => (0, NUM_COLS - 1),
            };

            if block.position == bound_right {
                block.direction = -1;
            } else if block.position == bound_left {
                block.direction = 1;
            }

            block.position += block.direction;

            let translation = transform.translation_mut();

            *translation.x_mut() =
                (block.position as f32) * TILE_WIDTH + TILE_WIDTH / 2.0 * block.size as f32;
        }
    }
}

fn spawn_blocks(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.spawn_new && !game_state.game_over {
        let size;
        let speed;

        match game_state.level {
            0..=2 => {
                size = 3.0;
            }
            3..=7 => {
                size = 2.0;
            }
            _ => {
                size = 1.0;
            }
        }
        speed = 1.0 / (game_state.level as f32 + 3.0);
        commands
            .spawn(SpriteComponents {
                material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
                transform: Transform::from_translation(Vec3::new(
                    -TILE_WIDTH / 2.0 * size,
                    TILE_WIDTH / 2.0 + TILE_WIDTH * game_state.level as f32,
                    0.0,
                )),
                sprite: Sprite {
                    size: Vec2::new(TILE_WIDTH * size, TILE_WIDTH),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with(Timer::from_seconds(speed, true))
            .with(Block {
                size: size as u8,
                position: 0,
                direction: 1,
                active: true,
            });

        game_state.spawn_new = false;
        game_state.level += 1;
    }
}

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy Arcade Stacker".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: true,
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.02, 0.02, 0.02)))
        .add_default_plugins()
        .add_plugin(StackerPlugin)
        .run();
}
