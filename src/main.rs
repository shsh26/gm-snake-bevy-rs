use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::PrimaryWindow;
use rand::random;


const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);
const FOOD_COLOR: Color = Color::rgb(1.0, 0.0, 1.0);

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component)]
struct SnakeHead;

#[derive(Component)]
struct Food;

fn setup_camera(mut commands: Commands) {
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: SNAKE_HEAD_COLOR,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(10.0, 10.0, 10.0),
            ..default()
        },
        ..default()
    })
        .insert(SnakeHead)
        .insert(Position { x: 3, y: 3 })
        .insert(Size::square(0.8));
}

fn snake_movement(keyboard_input: Res<ButtonInput<KeyCode>>, mut head_positions: Query<&mut Position, With<SnakeHead>>) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            pos.y += 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            pos.x += 1;
        }
    }
}

fn food_spawner(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: FOOD_COLOR,
            ..default()
        },
        ..default()
    })
        .insert(Food)
        .insert(Position { 
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32, 
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32, 
        })
        .insert(Size::square(0.8));
}

fn size_scaling(windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Size, &mut Transform)>) {
    let window = windows.get_single().unwrap();
    for (size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            size.width / ARENA_WIDTH as f32 * window.width() as f32,
            size.height / ARENA_HEIGHT as f32 * window.height() as f32,
            1.0,
        );
    }
}

fn position_translation(windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.0) + (tile_size / 2.0)
    }
    let window = windows.get_single().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            // 기본 해상도 설정
            primary_window: Some(Window {
                title: "Snake!".to_string(),
                name: Some("Snake Game".to_string()),
                resolution: (500.0, 500.0).into(),
                ..default()
            }),
            ..Default::default()
        }))
        // 프로그램 시작 시 카메라 설정 함수 실행
        .add_systems(Startup, (setup_camera, spawn_snake))
        .add_systems(Update, snake_movement)
        .add_systems(Update, food_spawner.run_if(on_timer(Duration::from_secs_f32(1.0))))
        // 기본 세팅 후 크기, 위치 변환
        .add_systems(PostUpdate, (position_translation, size_scaling))
        .run();
}
