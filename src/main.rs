use bevy::prelude::*;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

#[derive(Component)]
struct SnakeHead;

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
        .insert(SnakeHead);
}

fn snake_movement(mut head_positions: Query<(&SnakeHead, &mut Transform)>) {
    for (_, mut transform) in head_positions.iter_mut() {
        transform.translation.x += 2.0;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 프로그램 시작 시 카메라 설정 함수 실행
        // .add_systems(Startup, setup_camera)
        // .add_systems(Startup, spawn_snake)
        .add_systems(Update, (setup_camera, spawn_snake, snake_movement))
        .run();
}
